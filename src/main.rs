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
    let prefetch_file = match librp::prefetch::PrefetchHandle::new(filename){
        Ok(prefetch_file) => prefetch_file,
        Err(error) => panic!(error)
    };

    let prefetch = match prefetch_file.get_prefetch(){
        Ok(prefetch) => prefetch,
        Err(error) => panic!(error)
    };

    println!("prefetch: {:?}",prefetch);

    println!("Finished");
}
