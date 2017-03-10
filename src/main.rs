#[macro_use] extern crate log;
extern crate rustyprefetch;
extern crate clap;
extern crate serde_json;
extern crate serde;
use serde::Serializer;
use serde::ser::SerializeSeq;
use rustyprefetch::librp;
use clap::{App, Arg};
use std::fs;
use std::io;

fn process_directory<S>(directory: &str, serializer: S) where S: serde::Serializer {
    let mut seq = serializer.serialize_seq(None).unwrap();
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_string = path.into_os_string().into_string().unwrap();
            if path_string.ends_with(".pf"){
                process_file(&path_string,&mut seq);
            }
        }
    }
    seq.end().unwrap();
}

fn process_file<S: serde::ser::SerializeSeq>(filename: &str, serializer: &mut S) -> bool {
    // Check if file is a prefetch file
    let prefetch_file = match librp::prefetch::PrefetchHandle::new(filename) {
        Ok(prefetch_file) => prefetch_file,
        Err(error) => {
            warn!("Could not parse file: {} [error: {}]", filename, error);
            return false;
        }
    };
    let prefetch = prefetch_file.get_prefetch().unwrap();
    serializer.serialize_element(&prefetch).unwrap();
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

    let metrics_arg = Arg::with_name("pipe")
        .short("t")
        .long("tracechain")
        .help("Output Tracechains");

    let options = App::new("RustyPrefetch")
        .version("0.0.0")
        .author("Matthew Seyer <https://github.com/forensicmatt/RustyPrefetch>")
        .about("Parse prefetch.")
        .arg(prefetch_arg)
        .arg(metrics_arg)
        .get_matches();

    if options.is_present("metrics"){
        unsafe {
            librp::metrics::SKIP_TRACECHAIN = true;
        }
    }

    let source = options.value_of("source").unwrap();

    let mut serializer = serde_json::Serializer::pretty(
        io::stdout()
    );

    if is_directory(source) {
        process_directory(source,&mut serializer);
    } else {
        let mut seq = serializer.serialize_seq(None).unwrap();
        process_file(source,&mut seq);
        seq.end().unwrap();
    }
}
