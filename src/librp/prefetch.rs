use librp::errors;
use librp::compression;
use std::io::{Error};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::BufReader;
use std::boxed::Box;
use std::io::Cursor;
use std::fs::File;
use std::io::Read;
use std::mem;

use encoding::{Encoding, DecoderTrap};
use encoding::all::UTF_16LE;

#[derive(Debug)]
pub struct MamHeader{
    pub signature: u32,
    pub uncompressed_size: u32
}

pub struct PrefetchFile{
    pub filehandle: BufReader<Box<Read>>
}
impl PrefetchFile{
    pub fn new(prefetch_file: &str) -> Result<PrefetchFile,errors::PrefetchError> {
        // Check if file is a prefetch file
        let signature = prefetch_signature(prefetch_file)?;

        // Open a filehandle to the file
        let mut fh = File::open(prefetch_file)?;

        // Our buffer that holds the entire file
        let mut buffer: Vec<u8> = Vec::new();

        if signature == 72171853{
            // Get MAM header
            let header = read_mam_head(&mut fh)?;

            // create a buffer for the compressed data
            let mut compressed_buffer: Vec<u8> = Vec::new();
            // read from file into the compressed buffer
            fh.read_to_end(&mut compressed_buffer)?;

            // decompress the buffer
            let decompressed_buffer = match compression::decompress(
                                                    &compressed_buffer[..],
                                                    header.uncompressed_size as usize){
                Some(decompressed_buffer) => decompressed_buffer,
                None => return Err(errors::PrefetchError::decompression_error(
                    format!("Error decompressing file")))
            };

            buffer = decompressed_buffer.to_vec();
        } else {
            try!(fh.read_to_end(&mut buffer));
        }

        Ok(PrefetchFile {
            filehandle: BufReader::new(
                Box::new(Cursor::new(buffer))
            )
        })
    }

    pub fn get_file_header(&mut self) -> Result<PrefetchHeader,errors::PrefetchError>{
        let mut header: PrefetchHeader = unsafe {
            mem::zeroed()
        };

        header.version = self.filehandle.read_u32::<LittleEndian>()?;
        header.signature = self.filehandle.read_u32::<LittleEndian>()?;
        header.unknown1 = self.filehandle.read_u32::<LittleEndian>()?;
        header.filesize = self.filehandle.read_u32::<LittleEndian>()?;

        // Create a vector to store the byte buffer
        let mut name_buffer = [0u8; 60];
        self.filehandle.read_exact(&mut name_buffer)?;

        header.filename = match UTF_16LE.decode(&name_buffer,DecoderTrap::Ignore){
            Ok(filename) => filename,
            Err(error) => return Err(errors::PrefetchError::decode_error(
                                        format!("Error decoding filename in header. [{}]",error)))
        };
        let eo = match header.filename.find("\x00"){
            Some(eo) => eo,
            None => 30
        };

        if eo < 30 {
            header.filename.truncate(eo);
        }

        header.hash = self.filehandle.read_u32::<LittleEndian>()?;
        header.unknon2 = self.filehandle.read_u32::<LittleEndian>()?;

        Ok(header)
    }
}

#[derive(Debug)]
pub struct PrefetchHeader{
    pub version: u32,
    pub signature: u32,
    pub unknown1: u32,
    pub filesize: u32,
    pub filename: String,
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
    let mut fh = File::open(filename)?;

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
