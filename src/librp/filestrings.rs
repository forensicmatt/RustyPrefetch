use librp::errors::{PrefetchError};
use encoding::{Encoding, DecoderTrap};
use encoding::all::UTF_16LE;
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;

pub fn parse_filename_string<R: Read+Seek>(offset: u32, length:u32, mut reader: R)->Result<String,PrefetchError>{
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
        Err(error) => return Err(PrefetchError::decode_error(
                                    format!("Error decoding filename in header. [{}]",error)))
    };

    // Seek back to original offset
    reader.seek(
        SeekFrom::Start(cur_pos)
    )?;

    Ok(filename_string)
}

pub fn get_string_array<R: Read+Seek>(offset: u32, string_count: u32, mut reader: R)->Result<Vec<String>,PrefetchError>{
    let mut strings_read: u32 = 0;
    let mut strings: Vec<String> = Vec::new();

    // Get Current Pos
    let cur_pos = reader.seek(
        SeekFrom::Current(0)
    )?;

    // Seek to string offset
    reader.seek(
        SeekFrom::Start(offset as u64)
    )?;

    while strings_read < string_count {
        let string_size: u16 = reader.read_u16::<LittleEndian>()?;
        if string_size == 0 {
            continue;
        }
        let mut name_buffer: Vec<u8> = vec![0; (string_size * 2) as usize];
        reader.read_exact(&mut name_buffer.as_mut_slice())?;

        let directory_string = match UTF_16LE.decode(name_buffer.as_mut_slice(),DecoderTrap::Ignore){
            Ok(directory_string) => directory_string,
            Err(error) => return Err(PrefetchError::decode_error(
                                        format!("Error decoding filename in header. [{}]",error)))
        };

        strings.push(directory_string);

        strings_read += 1;
    }

    // Seek back to original offset
    reader.seek(
        SeekFrom::Start(cur_pos)
    )?;

    Ok(strings)
}
