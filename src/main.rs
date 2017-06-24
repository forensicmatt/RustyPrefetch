#[macro_use] extern crate log;
extern crate rustyprefetch;
extern crate rwinstructs;
extern crate clap;
extern crate serde_json;
extern crate serde;
extern crate jmespath;
use jmespath::{Expression};
use rwinstructs::reference;
use rwinstructs::serialize;
use rustyprefetch::librp;
use clap::{App, Arg, ArgMatches};
use std::fs;
use std::io;

fn process_directory(directory: &str, options: &ArgMatches) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_string = path.into_os_string().into_string().unwrap();
            if path_string.ends_with(".pf"){
                process_file(&path_string, &options);
            }
        }
    }
}

fn process_file(filename: &str, options: &ArgMatches) -> bool {
    // JMES Expression if needed
    let mut expr: Option<Expression> = None;
    if options.is_present("query") {
        expr = Some(jmespath::compile(
            options.value_of("query").unwrap()
        ).unwrap());
    }

    // Expression bool flag
    let mut expr_as_bool = false;
    if options.is_present("bool_expr"){
        expr_as_bool = true;
    }

    // Check if file is a prefetch file
    let prefetch_file = match librp::prefetch::PrefetchHandle::new(filename,None) {
        Ok(prefetch_file) => prefetch_file,
        Err(error) => {
            warn!("Could not parse file: {} [error: {}]", filename, error);
            return false;
        }
    };
    let prefetch = prefetch_file.get_prefetch().unwrap();

    let json_str = serde_json::to_string(&prefetch).unwrap();

    match expr {
        Some(ref j_expr) => {
            let data = jmespath::Variable::from_json(&json_str).unwrap();
            let result = j_expr.search(data).unwrap();
            if expr_as_bool {
                match result.as_boolean() {
                    Some(bool_value) => {
                        match bool_value {
                            true => println!("{}",json_str),
                            false => {}
                        }
                    },
                    None => {
                        panic!("Query expression is not a bool expression!");
                    }
                }
            } else {
                println!("{}",result)
            }
        },
        None => {
            println!("{}",json_str);
        }
    }

    return true;
}

fn is_directory(source: &str)->bool{
    let metadata = fs::metadata(source).unwrap();

    let file_type = metadata.file_type();

    file_type.is_dir()
}

fn main() {
    let prefetch_arg = Arg::with_name("source")
        .short("s")
        .long("source")
        .value_name("FILE")
        .help("The source path. Can be a file or a directory.")
        .required(true)
        .takes_value(true);

    let jmes_arg = Arg::with_name("query")
        .short("q")
        .long("query")
        .value_name("QUERY")
        .help("JMES Query")
        .takes_value(true);

    let bool_arg = Arg::with_name("bool_expr")
        .short("b")
        .long("bool_expr")
        .help("JMES Query as bool only. (Prints whole record if true.)");

    let metrics_arg = Arg::with_name("tracechain")
        .short("t")
        .long("tracechain")
        .help("Output Tracechains");

    let options = App::new("RustyPrefetch")
        .version("0.2.0")
        .author("Matthew Seyer <https://github.com/forensicmatt/RustyPrefetch>")
        .about("Parse prefetch.")
        .arg(prefetch_arg)
        .arg(jmes_arg)
        .arg(bool_arg)
        .arg(metrics_arg)
        .get_matches();

    if options.is_present("tracechain"){
        unsafe {
            librp::metrics::SKIP_TRACECHAIN = false;
        }
    }

    // Set Reference Display Options
    unsafe{reference::NESTED_REFERENCE = true;}
    unsafe{serialize::U64_SERIALIZATION = serialize::U64Serialization::AsString;}

    let source = options.value_of("source").unwrap();

    if is_directory(source) {
        process_directory(source,&options);
    } else {
        process_file(source,&options);
    }
}
