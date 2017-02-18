use librp::errors;
use librp::utils;
use std::io::{Error, ErrorKind};
use byteorder::{ReadBytesExt, LittleEndian};
use std::fs::File;
use std::io::Read;
use std::mem;

#[derive(Debug)]
pub struct MamHeader{
    pub signature: u32,
    pub uncompressed_size: u32
}

#[derive(Debug)]
pub struct PrefetchHeader{
    pub version: u32,
    pub signature: u32,
    pub unknown1: u32,
    pub filesize: u32,
    pub filename: [u16; 30],
    pub hash: u32,
    pub unknon2: u32
}

pub fn read_mam_head<R: Read>(mut buffer: R)->Result<MamHeader,Error>{
    let mut header: MamHeader = unsafe {
        mem::zeroed()
    };

    header.signature = buffer.read_u32::<LittleEndian>()?;
    header.uncompressed_size = buffer.read_u32::<LittleEndian>()?;

    Ok(header)
}

pub fn prefetch_signature(filename: &str)->Result<u32,errors::PrefetchError>{
    // Open a filehandle to the file
    let mut fh = try!(File::open(filename));

    // Get File Size to ensure we have enough data to read
    let metadata = try!(fh.metadata());
    let file_size = metadata.len();

    if file_size > 8 {
        // We are only ever going to check the first 8 bytes of the file for a signature
        // check for MAM signature
        let prt1 = try!(fh.read_u32::<LittleEndian>());
        if prt1 == 72171853 {
            Ok(prt1)
        }else{
            // check for SCCA signature
            let prt2 = try!(fh.read_u32::<LittleEndian>());
            if prt2 == 1094927187 {
                Ok(prt2)
            } else {
                // Unknown File signature
                Err(errors::PrefetchError::invalid_file_signature(format!(
                        "Unkown File Signature"
                )))
            }
        }
    } else {
        Err(errors::PrefetchError::invalid_file_signature(format!("File size invalid")))
    }
}
