extern crate rustyprefetch;
extern crate clap;
use rustyprefetch::librp;
use clap::{App, Arg};

fn main() {
    let prefetch_arg = Arg::with_name("prefetch")
        .short("p")
        .long("prefetch")
        .value_name("FILE")
        .help("The Prefetch file to decode")
        .required(true)
        .takes_value(true);

    let options = App::new("RustyPrefetch")
        .version("0.0.0")
        .author("Matthew Seyer <matthew.seyer@gmail.com>")
        .about("Parse prefetch.")
        .arg(prefetch_arg)
        .get_matches();

    let filename = options.value_of("prefetch").unwrap();

    // Check if file is a prefetch file
    let mut prefetch_file = match librp::prefetch::PrefetchFile::new(filename){
        Ok(prefetch_file) => prefetch_file,
        Err(error) => panic!(error)
    };

    let prefetch_header = match prefetch_file.get_file_header() {
        Ok(prefetch_header) => prefetch_header,
        Err(error) => panic!(error)
    };

    println!("prefetch_header: {:?}",prefetch_header);

    println!("Finished");
}
