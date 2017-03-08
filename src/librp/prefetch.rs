use librp::errors;
use librp::compression;
use librp::metrics;
use librp::fileinfo;
use librp::volume;
use std::io::{Error};
use byteorder::{ReadBytesExt, LittleEndian};
use seek_bufread::BufReader;
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

#[derive(Debug)]
pub struct PrefetchHandle{
    pub filename: String,
    pub buffer: Vec<u8>
}
impl PrefetchHandle{
    pub fn new(prefetch_filename: &str) -> Result<PrefetchHandle,errors::PrefetchError> {
        let mut prefetch_file: PrefetchHandle = unsafe {
            mem::zeroed()
        };
        prefetch_file.filename = String::from(prefetch_filename);

        // Check if file is a prefetch file
        let signature = prefetch_signature(prefetch_filename)?;

        // Open a filehandle to the file
        let mut fh = File::open(prefetch_filename)?;

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

            prefetch_file.buffer = decompressed_buffer.to_vec();
        } else {
            try!(fh.read_to_end(&mut prefetch_file.buffer));
        }

        Ok(
            prefetch_file
        )
    }

    pub fn get_prefetch(&self) -> Result<PrefetchFile,errors::PrefetchError> {
        Ok(
            PrefetchFile::new(self.filename.clone(),&self.buffer)?
        )
    }
}

#[derive(Serialize, Debug)]
pub struct PrefetchFile{
    pub source_file: String,
    pub header: PrefetchHeader,
    pub fileinfo: fileinfo::FileInformation,
    pub metrics: metrics::MetricsArray,
    pub volumes: volume::VolumeArray
}
impl PrefetchFile{
    pub fn new(filename: String, buffer: &Vec<u8>) -> Result<PrefetchFile,errors::PrefetchError> {
        let mut reader = BufReader::new(
            Cursor::new(buffer)
        );

        let prefetch_header = PrefetchHeader::new(
            &mut reader
        )?;

        let file_info = fileinfo::FileInformation::new(
            &prefetch_header,
            &mut reader
        )?;

        let metrics_array = metrics::MetricsArray::new(
            &prefetch_header,
            &file_info,
            &mut reader
        )?;

        let volume_array = volume::VolumeArray::new(
            &prefetch_header,
            &file_info,
            &mut reader
        )?;

        Ok(
            PrefetchFile{
                source_file: filename,
                header: prefetch_header,
                fileinfo: file_info,
                metrics: metrics_array,
                volumes: volume_array
            }
        )
    }
}

#[derive(Serialize, Debug)]
pub struct PrefetchHeader{
    pub version: u32,
    pub signature: u32,
    pub unknown1: u32,
    pub filesize: u32,
    pub filename: String,
    pub hash: u32,
    pub unknon2: u32
}
impl PrefetchHeader{
    pub fn new<R: Read>(mut buffer: R)->Result<PrefetchHeader,errors::PrefetchError>{
        let mut header: PrefetchHeader = unsafe {
            mem::zeroed()
        };
        header.version = buffer.read_u32::<LittleEndian>()?;
        header.signature = buffer.read_u32::<LittleEndian>()?;
        header.unknown1 = buffer.read_u32::<LittleEndian>()?;
        header.filesize = buffer.read_u32::<LittleEndian>()?;

        // Create a vector to store the byte buffer
        let mut name_buffer = [0u8; 60];
        buffer.read_exact(&mut name_buffer)?;

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

        header.hash = buffer.read_u32::<LittleEndian>()?;
        header.unknon2 = buffer.read_u32::<LittleEndian>()?;

        Ok(header)
    }
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
