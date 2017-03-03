extern crate rustyprefetch;
extern crate clap;
#[macro_use] extern crate serde_json;
use rustyprefetch::librp;
use clap::{App, Arg};
use std::fs;
use std::io;

fn process_directory(directory: &str){
    println!("process_directory() Not yet implemented");
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            println!("path: {:?}",path);
        }
    }
}

fn process_file(filename: &str){
    // Check if file is a prefetch file
    let prefetch_file = librp::prefetch::PrefetchHandle::new(filename).unwrap();

    let prefetch = prefetch_file.get_prefetch().unwrap();

    let serialized = serde_json::to_string_pretty(&prefetch).unwrap();
    println!("{}",serialized);
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

    let options = App::new("RustyPrefetch")
        .version("0.0.0")
        .author("Matthew Seyer <matthew.seyer@gmail.com>")
        .about("Parse prefetch.")
        .arg(prefetch_arg)
        .get_matches();

    let source = options.value_of("source").unwrap();

    if is_directory(source) {
        process_directory(source);
    } else {
        process_file(source);
    }
}
