extern crate rustyprefetch;
extern crate clap;
use rustyprefetch::librp;
use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use std::io;
use std::io::prelude::*;

fn main() {
    let prefetch_arg = Arg::with_name("prefetch")
        .short("p")
        .long("prefetch")
        .value_name("FILE")
        .help("The Prefetch file to decode")
        .required(true)
        .takes_value(true);

    let options = App::new("DecompressPrefetch")
        .version("0.1.0")
        .author("Matthew Seyer <https://github.com/forensicmatt/RustyPrefetch>")
        .about("Test tool to decompress a compressed prefetch file.")
        .arg(prefetch_arg)
        .get_matches();

    let prefetch_file = options.value_of("prefetch").unwrap();

    // Open a filehandle to the file
    let mut fh = match File::open(prefetch_file) {
        Ok(fh) => fh,
        Err(error) => panic!("Error: {}",error)
    };

    // Check if file is a prefetch file
    let signature = match librp::prefetch::prefetch_signature(&mut fh) {
        Ok(signature) => signature,
        Err(error) => panic!(error)
    };

    // Verify MAM file signature
    if signature != 72171853 {
        panic!("File signatures is not MAM")
    }

    // Get MAM header
    let header = match librp::prefetch::read_mam_head(&mut fh) {
        Ok(header) => header,
        Err(error) => panic!("Error: {}",error)
    };

    // create a buffer for the compressed data
    let mut compressed_buffer: Vec<u8> = Vec::new();

    // read from file into the compressed buffer
    let mut bytes_read = match fh.read_to_end(&mut compressed_buffer){
        Ok(bytes_read) => bytes_read,
        Err(error) => panic!("Error: {}",error)
    };

    // decompress the buffer
    let mut decompressed_buffer = match librp::compression::decompress(
                                            &compressed_buffer[..],
                                            header.uncompressed_size as usize){
        Some(decompressed_buffer) => decompressed_buffer,
        None => panic!("No decopressed buffer")
    };

    // open stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // write decompressed data to stdout
    handle.write(&decompressed_buffer[..]);
}
