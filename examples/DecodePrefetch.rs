extern crate rustyprefetch;
extern crate clap;
use rustyprefetch::librp;
use clap::{App, Arg};
use std::fs::File;
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

    let options = App::new("DecodePrefetch")
        .version("0.0.0")
        .author("Matthew Seyer <matthew.seyer@gmail.com>")
        .about("Test tool to decode a compressed prefetch file.")
        .arg(prefetch_arg)
        .get_matches();

    let prefetch_file = options.value_of("prefetch").unwrap();

    println!("Prefetch to decode: {:?}",prefetch_file);

    // Open a filehandle to the file
    let mut fh = match File::open(prefetch_file) {
        Ok(fh) => fh,
        Err(error) => panic!("Error: {}",error)
    };

    let header = match librp::prefetch::read_mam_head(&mut fh) {
        Ok(header) => header,
        Err(error) => panic!("Error: {}",error)
    };
    println!("Header: {:?}",header);

    let mut compressed_buffer: Vec<u8> = Vec::new();

    let mut bytes_read = match fh.read_to_end(&mut compressed_buffer){
        Ok(bytes_read) => bytes_read,
        Err(error) => panic!("Error: {}",error)
    };

    println!("Compressed Buffer: {:?}",librp::utils::to_hex_string(compressed_buffer.clone()));

    let mut decompressed_buffer = match librp::compression::decompress(
                                            &compressed_buffer[..],
                                            header.uncompressed_size as usize){
        Some(decompressed_buffer) => decompressed_buffer,
        None => panic!("No decopressed buffer")
    };

    println!("Decompressed Buffer: {:?}",librp::utils::to_hex_string(decompressed_buffer.clone()));

    println!("End!");
}
