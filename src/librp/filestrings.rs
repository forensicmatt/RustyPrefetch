use librp::errors;
use encoding::{Encoding, DecoderTrap};
use encoding::all::UTF_16LE;
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

pub fn parse_filename_string<R: Read+Seek>(offset: u32, length:u32, mut reader: R)->Result<String,errors::PrefetchError>{
    // Get Current Pos
    let cur_pos = reader.seek(
        SeekFrom::Current(0)
    )?;

    // Seek to string offset
    reader.seek(
        SeekFrom::Start(offset as u64)
    )?;

    let mut name_buffer: Vec<u8> = vec![0; (length * 2) as usize];
    reader.read_exact(&mut name_buffer.as_mut_slice())?;

    let filename_string = match UTF_16LE.decode(name_buffer.as_mut_slice(),DecoderTrap::Ignore){
        Ok(filename_string) => filename_string,
        Err(error) => return Err(errors::PrefetchError::decode_error(
                                    format!("Error decoding filename in header. [{}]",error)))
    };

    // Seek back to original offset
    reader.seek(
        SeekFrom::Start(cur_pos)
    )?;

    Ok(filename_string)
}
