use librp::errors::{PrefetchError};
use librp::prefetch::{PrefetchHeader};
use librp::utils;
use librp::utils::{ByteArray};
use rwinstructs::timestamp::{WinTimestamp};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::Read;
use std::io::Cursor;
use std::io::BufReader;
use std::fmt;
use std::mem;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum FileInformation{
    V17(FileInformationV17),
    V23(FileInformationV23),
    V26(FileInformationV26),
    V30(FileInformationV30)
}
#[derive(Serialize)]
pub struct FileInformationV17{
    pub metrics_array_offset: u32, //The offset is relative to the start of the file
    pub metrics_entry_count: u32,
    pub trace_array_offset: u32,  //The offset is relative to the start of the file
    pub trace_entry_count: u32,
    pub filename_offset: u32,
    pub filename_length: u32,
    pub volume_info_offset: u32,
    pub volume_info_count: u32,
    pub volume_info_size: u32,
    pub last_run_time: WinTimestamp,
    pub unknown1: ByteArray, //[u8; 16],
    pub run_count: u32,
    pub unknown2: u32
}
impl fmt::Debug for FileInformationV17 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FileInformationV17 {{ \
            metrics_array_offset: {}, \
            metrics_entry_count: {}, \
            trace_array_offset: {}, \
            trace_entry_count: {}, \
            filename_offset: {}, \
            filename_length: {}, \
            volume_info_offset: {}, \
            volume_info_count: {}, \
            volume_info_size: {}, \
            last_run_time: {:?}, \
            unknown1: {:?}, \
            run_count: {}, \
            unknown2: {}, \
            }}",
            self.metrics_array_offset,
            self.metrics_entry_count,
            self.trace_array_offset,
            self.trace_entry_count,
            self.filename_offset,
            self.filename_length,
            self.volume_info_offset,
            self.volume_info_count,
            self.volume_info_size,
            self.last_run_time,
            self.unknown1,
            self.run_count,
            self.unknown2
        )
    }
}
#[derive(Serialize)]
pub struct FileInformationV23{
    pub metrics_array_offset: u32, //The offset is relative to the start of the file
    pub metrics_entry_count: u32,
    pub trace_array_offset: u32,  //The offset is relative to the start of the file
    pub trace_entry_count: u32,
    pub filename_offset: u32,
    pub filename_length: u32,
    pub volume_info_offset: u32,
    pub volume_info_count: u32,
    pub volume_info_size: u32,
    pub unknown3: u64,
    pub last_run_time: WinTimestamp,
    pub unknown1: ByteArray, //[u8; 16],
    pub run_count: u32,
    pub unknown2: u32,
    pub unknown4: ByteArray //[u8; 80]
}
impl fmt::Debug for FileInformationV23 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FileInformationV23 {{ \
            metrics_array_offset: {}, \
            metrics_entry_count: {}, \
            trace_array_offset: {}, \
            trace_entry_count: {}, \
            filename_offset: {}, \
            filename_length: {}, \
            volume_info_offset: {}, \
            volume_info_count: {}, \
            volume_info_size: {}, \
            unknown3: {}, \
            last_run_time: {:?}, \
            unknown1: {:?}, \
            run_count: {}, \
            unknown2: {}, \
            unknown4: {:?}\
            }}",
            self.metrics_array_offset,
            self.metrics_entry_count,
            self.trace_array_offset,
            self.trace_entry_count,
            self.filename_offset,
            self.filename_length,
            self.volume_info_offset,
            self.volume_info_count,
            self.volume_info_size,
            self.unknown3,
            self.last_run_time,
            self.unknown1,
            self.run_count,
            self.unknown2,
            self.unknown4
        )
    }
}
#[derive(Serialize)]
pub struct FileInformationV26{
    pub metrics_array_offset: u32, //The offset is relative to the start of the file
    pub metrics_entry_count: u32,
    pub trace_array_offset: u32,  //The offset is relative to the start of the file
    pub trace_entry_count: u32,
    pub filename_offset: u32,
    pub filename_length: u32,
    pub volume_info_offset: u32,
    pub volume_info_count: u32,
    pub volume_info_size: u32,
    pub unknown3: u64,
    pub last_run_time: [WinTimestamp;8],
    pub unknown1: ByteArray, //[u8; 16],
    pub run_count: u32,
    pub unknown2: u32,
    pub unknown5: u32,
    pub unknown4: ByteArray //[u8; 88],
}
impl fmt::Debug for FileInformationV26 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FileInformationV26 {{ \
            metrics_array_offset: {}, \
            metrics_entry_count: {}, \
            trace_array_offset: {}, \
            trace_entry_count: {}, \
            filename_offset: {}, \
            filename_length: {}, \
            volume_info_offset: {}, \
            volume_info_count: {}, \
            volume_info_size: {}, \
            unknown3: {}, \
            last_run_time: {:?}, \
            unknown1: {:?}, \
            run_count: {}, \
            unknown2: {}, \
            unknown5: {}, \
            unknown4: {:?}\
            }}",
            self.metrics_array_offset,
            self.metrics_entry_count,
            self.trace_array_offset,
            self.trace_entry_count,
            self.filename_offset,
            self.filename_length,
            self.volume_info_offset,
            self.volume_info_count,
            self.volume_info_size,
            self.unknown3,
            self.last_run_time,
            self.unknown1,
            self.run_count,
            self.unknown2,
            self.unknown5,
            self.unknown4
        )
    }
}
#[derive(Serialize)]
pub struct FileInformationV30{
    pub metrics_array_offset: u32, //The offset is relative to the start of the file
    pub metrics_entry_count: u32,
    pub trace_array_offset: u32,  //The offset is relative to the start of the file
    pub trace_entry_count: u32,
    pub filename_offset: u32,
    pub filename_length: u32,
    pub volume_info_offset: u32,
    pub volume_info_count: u32,
    pub volume_info_size: u32,
    pub unknown3: u64,
    pub last_run_time: [WinTimestamp;8],
    pub unknown1: ByteArray, //[u8; 16],
    pub run_count: u32,
    pub unknown2: u32,
    pub unknown5: u32,
    pub unknown4: ByteArray //[u8; 88]
}
impl fmt::Debug for FileInformationV30 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FileInformationV30 {{ \
            metrics_array_offset: {}, \
            metrics_entry_count: {}, \
            trace_array_offset: {}, \
            trace_entry_count: {}, \
            filename_offset: {}, \
            filename_length: {}, \
            volume_info_offset: {}, \
            volume_info_count: {}, \
            volume_info_size: {}, \
            unknown3: {}, \
            last_run_time: {:?}, \
            unknown1: {:?}, \
            run_count: {}, \
            unknown2: {}, \
            unknown5: {}, \
            unknown4: {:?}\
            }}",
            self.metrics_array_offset,
            self.metrics_entry_count,
            self.trace_array_offset,
            self.trace_entry_count,
            self.filename_offset,
            self.filename_length,
            self.volume_info_offset,
            self.volume_info_count,
            self.volume_info_size,
            self.unknown3,
            self.last_run_time,
            self.unknown1,
            self.run_count,
            self.unknown2,
            self.unknown5,
            self.unknown4
        )
    }
}

impl FileInformation{
    pub fn new<R: Read>(header: &PrefetchHeader,reader: R) -> Result<FileInformation,PrefetchError> {
        if header.version == 17{
            Ok(
                FileInformation::V17(get_fileinfo_v17(reader)?)
            )
        } else if header.version == 23{
            Ok(
                FileInformation::V23(get_fileinfo_v23(reader)?)
            )
        } else if header.version == 26{
            Ok(
                FileInformation::V26(get_fileinfo_v26(reader)?)
            )
        } else if header.version == 30{
            Ok(
                FileInformation::V30(get_fileinfo_v30(reader)?)
            )
        } else {
            Err(PrefetchError::parse_error(
                format!("Error parsing file info. Invalid version: {}",header.version)
            ))
        }
    }
}
pub fn get_fileinfo_v17<R: Read>(mut reader: R) -> Result<FileInformationV17,PrefetchError> {
    let mut bytes: [u8; 68] = [0;68];
    reader.read_exact(&mut bytes)?;

    // println!("fileinfo buffer hex: {:?}",utils::to_hex_string(bytes.to_vec()));

    let mut buffer = BufReader::new(
        Cursor::new(bytes.to_vec())
    );

    let mut fileinfo_v17: FileInformationV17 = unsafe {
        mem::zeroed()
    };

    fileinfo_v17.metrics_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.metrics_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.trace_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.trace_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.filename_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.filename_length = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.volume_info_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.volume_info_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.volume_info_size = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.last_run_time = WinTimestamp(buffer.read_u64::<LittleEndian>()?);

    fileinfo_v17.unknown1 = ByteArray(vec![0; 16]);
    buffer.read_exact(&mut fileinfo_v17.unknown1.0.as_mut_slice())?;

    fileinfo_v17.run_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v17.unknown2 = buffer.read_u32::<LittleEndian>()?;

    Ok(fileinfo_v17)
}
pub fn get_fileinfo_v23<R: Read>(mut reader: R) -> Result<FileInformationV23,PrefetchError> {
    let mut bytes: [u8; 156] = [0;156];
    reader.read_exact(&mut bytes)?;

    // println!("fileinfo buffer hex: {:?}",utils::to_hex_string(bytes.to_vec()));

    let mut buffer = BufReader::new(
        Cursor::new(bytes.to_vec())
    );

    let mut fileinfo_v23: FileInformationV23 = unsafe {
        mem::zeroed()
    };

    fileinfo_v23.metrics_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.metrics_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.trace_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.trace_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.filename_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.filename_length = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.volume_info_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.volume_info_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.volume_info_size = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.unknown3 = buffer.read_u64::<LittleEndian>()?;
    fileinfo_v23.last_run_time = WinTimestamp(buffer.read_u64::<LittleEndian>()?);

    fileinfo_v23.unknown1 = ByteArray(vec![0; 16]);
    buffer.read_exact(&mut fileinfo_v23.unknown1.0.as_mut_slice())?;

    fileinfo_v23.run_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v23.unknown2 = buffer.read_u32::<LittleEndian>()?;

    fileinfo_v23.unknown4 = ByteArray(vec![0; 80]);
    buffer.read_exact(&mut fileinfo_v23.unknown4.0.as_mut_slice())?;

    Ok(fileinfo_v23)
}
pub fn get_fileinfo_v26<R: Read>(mut reader: R) -> Result<FileInformationV26,PrefetchError> {
    let mut bytes: [u8; 224] = [0;224];
    reader.read_exact(&mut bytes)?;

    // println!("fileinfo buffer hex: {:?}",utils::to_hex_string(bytes.to_vec()));

    let mut buffer = BufReader::new(
        Cursor::new(bytes.to_vec())
    );

    let mut fileinfo_v26: FileInformationV26 = unsafe {
        mem::zeroed()
    };

    fileinfo_v26.metrics_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.metrics_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.trace_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.trace_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.filename_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.filename_length = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.volume_info_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.volume_info_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.volume_info_size = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.unknown3 = buffer.read_u64::<LittleEndian>()?;
    for n in 0..8 {
        fileinfo_v26.last_run_time[n] = WinTimestamp(buffer.read_u64::<LittleEndian>()?);
    }

    fileinfo_v26.unknown1 = ByteArray(vec![0; 16]);
    buffer.read_exact(&mut fileinfo_v26.unknown1.0.as_mut_slice())?;

    fileinfo_v26.run_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.unknown2 = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v26.unknown5 = buffer.read_u32::<LittleEndian>()?;

    fileinfo_v26.unknown4 = ByteArray(vec![0; 88]);
    buffer.read_exact(&mut fileinfo_v26.unknown4.0.as_mut_slice())?;

    Ok(fileinfo_v26)
}
pub fn get_fileinfo_v30<R: Read>(mut reader: R) -> Result<FileInformationV30,PrefetchError> {
    let mut bytes: [u8; 224] = [0;224];
    reader.read_exact(&mut bytes)?;

    // println!("fileinfo buffer hex: {:?}",utils::to_hex_string(bytes.to_vec()));

    let mut buffer = BufReader::new(
        Cursor::new(bytes.to_vec())
    );

    let mut fileinfo_v30: FileInformationV30 = unsafe {
        mem::zeroed()
    };

    fileinfo_v30.metrics_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.metrics_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.trace_array_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.trace_entry_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.filename_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.filename_length = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.volume_info_offset = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.volume_info_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.volume_info_size = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.unknown3 = buffer.read_u64::<LittleEndian>()?;
    for n in 0..8 {
        fileinfo_v30.last_run_time[n] = WinTimestamp(buffer.read_u64::<LittleEndian>()?);
    }

    fileinfo_v30.unknown1 = ByteArray(vec![0; 16]);
    buffer.read_exact(&mut fileinfo_v30.unknown1.0.as_mut_slice())?;

    fileinfo_v30.run_count = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.unknown2 = buffer.read_u32::<LittleEndian>()?;
    fileinfo_v30.unknown5 = buffer.read_u32::<LittleEndian>()?;

    fileinfo_v30.unknown4 = ByteArray(vec![0; 88]);
    buffer.read_exact(&mut fileinfo_v30.unknown4.0.as_mut_slice())?;

    Ok(fileinfo_v30)
}
