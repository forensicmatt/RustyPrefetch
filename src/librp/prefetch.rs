use std::io::{Error, ErrorKind};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::Read;
use std::mem;

#[derive(Debug)]
pub struct MamHeader{
    pub signature: u32,
    pub uncompressed_size: u32
}

pub fn read_mam_head<R: Read>(mut buffer: R)->Result<MamHeader,Error>{
    let mut header: MamHeader = unsafe {
        mem::zeroed()
    };

    header.signature = buffer.read_u32::<LittleEndian>()?;
    header.uncompressed_size = buffer.read_u32::<LittleEndian>()?;

    Ok(header)
}
