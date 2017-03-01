use librp::prefetch::{PrefetchHeader};
use librp::fileinfo::{FileInformation};
use librp::timestamp::{WinTimestamp};
use librp::errors::{PrefetchError};
use librp::filestrings;
use librp::utils::{ByteArray};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;
use std::mem;

#[derive(Serialize, Debug)]
pub struct ReferenceTable{
    pub version: u32,
    pub reference_count: u32,
    pub references: Vec<u64>
}
impl ReferenceTable{
    pub fn new<Rs: Read+Seek>(offset: u32, mut reader: Rs) -> Result<ReferenceTable,PrefetchError> {
        let mut reference_table: ReferenceTable = unsafe {
            mem::zeroed()
        };

        // Get Current Pos
        let cur_pos = reader.seek(
            SeekFrom::Current(0)
        )?;

        // Seek to string offset
        reader.seek(
            SeekFrom::Start(offset as u64)
        )?;

        reference_table.version = reader.read_u32::<LittleEndian>()?;
        reference_table.reference_count = reader.read_u32::<LittleEndian>()?;

        for i in 0..reference_table.reference_count{
            reference_table.references.push(
                reader.read_u64::<LittleEndian>()?
            );
        }

        // Seek back to original offset
        reader.seek(
            SeekFrom::Start(cur_pos)
        )?;

        Ok(reference_table)
    }
}

#[derive(Serialize, Debug)]
pub struct VolumeArray(
    pub Vec<VolumeEntry>
);
impl VolumeArray{
    pub fn new<R: Read + Seek>(header: &PrefetchHeader,fileinfo: &FileInformation,mut reader: R) -> Result<VolumeArray,PrefetchError>{
        let mut volume_array = VolumeArray(Vec::new());
        let mut volume_info_offset:u32 = 0;
        let mut volume_info_count:u32 = 0;

        match *fileinfo {
            FileInformation::V17(ref info) => {
                volume_info_offset = info.volume_info_offset;
                volume_info_count = info.volume_info_count;
            },
            FileInformation::V23(ref info) => {
                volume_info_offset = info.volume_info_offset;
                volume_info_count = info.volume_info_count;
            },
            FileInformation::V26(ref info) => {
                volume_info_offset = info.volume_info_offset;
                volume_info_count = info.volume_info_count;
            },
            FileInformation::V30(ref info) => {
                volume_info_offset = info.volume_info_offset;
                volume_info_count = info.volume_info_count;
            }
        }

        // Seek to metrics array
        reader.seek(
            SeekFrom::Start(volume_info_offset as u64)
        )?;

        // read number of entries
        for i in 0..volume_info_count {
            volume_array.0.push(
                VolumeEntry::new(
                    header,
                    fileinfo,
                    &mut reader
                )?
            );
        }

        Ok(volume_array)
    }
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum VolumeEntry{
    V17(VolumeEntryV17),
    V23(VolumeEntryV23),
    V26(VolumeEntryV26),
    V30(VolumeEntryV30)
}
impl VolumeEntry{
    pub fn new<Rs: Read+Seek>(header: &PrefetchHeader, fileinfo: &FileInformation, mut reader: Rs) -> Result<VolumeEntry,PrefetchError> {
        if header.version == 17{
            let mut volume_entry_v17 = VolumeEntryV17::new(&mut reader)?;
            volume_entry_v17.get_path_string(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v17.get_directory_strings(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v17.get_reference_table(
                fileinfo,
                &mut reader
            )?;
            Ok(VolumeEntry::V17(volume_entry_v17))
        } else if header.version == 23{
            let mut volume_entry_v23 = VolumeEntryV23::new(&mut reader)?;
            volume_entry_v23.get_path_string(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v23.get_directory_strings(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v23.get_reference_table(
                fileinfo,
                &mut reader
            )?;
            Ok(VolumeEntry::V23(volume_entry_v23))
        } else if header.version == 26{
            let mut volume_entry_v26 = VolumeEntryV26::new(&mut reader)?;
            volume_entry_v26.get_path_string(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v26.get_directory_strings(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v26.get_reference_table(
                fileinfo,
                &mut reader
            )?;
            Ok(VolumeEntry::V26(volume_entry_v26))
        } else if header.version == 30{
            let mut volume_entry_v30 = VolumeEntryV30::new(&mut reader)?;
            volume_entry_v30.get_path_string(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v30.get_directory_strings(
                fileinfo,
                &mut reader
            )?;
            volume_entry_v30.get_reference_table(
                fileinfo,
                &mut reader
            )?;
            Ok(VolumeEntry::V30(volume_entry_v30))
        } else {
            Err(PrefetchError::parse_error(
                format!("Error parsing file info. Invalid version: {}",header.version)
            ))
        }
    }
}
#[derive(Serialize, Debug)]
pub struct VolumeEntryV17 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: WinTimestamp,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32,
    pub path_string: String,
    pub directory_strings: Vec<String>,
    pub reference_table: ReferenceTable
}
impl VolumeEntryV17{
    pub fn new<R: Read>(mut reader: R) -> Result<VolumeEntryV17,PrefetchError>{
        let mut volume_entry_v17: VolumeEntryV17 = unsafe {
            mem::zeroed()
        };

        volume_entry_v17.path_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v17.path_length = reader.read_u32::<LittleEndian>()?;
        volume_entry_v17.vol_creation_time = WinTimestamp(reader.read_u64::<LittleEndian>()?);
        volume_entry_v17.volume_serial = reader.read_u32::<LittleEndian>()?;
        volume_entry_v17.references_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v17.references_data_size = reader.read_u32::<LittleEndian>()?;
        volume_entry_v17.directory_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v17.directory_string_count = reader.read_u32::<LittleEndian>()?;
        volume_entry_v17.unknown1 = reader.read_u32::<LittleEndian>()?;


        Ok(volume_entry_v17)
    }
    pub fn get_path_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V17(ref info) => info,
            _ => panic!("FileInformation not V17"),
        };

        self.path_string = filestrings::parse_filename_string(
            info.volume_info_offset + self.path_offset,
            self.path_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_directory_strings<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V17(ref info) => info,
            _ => panic!("FileInformation not V17"),
        };

        self.directory_strings = filestrings::get_string_array(
            info.volume_info_offset + self.directory_offset,
            self.directory_string_count,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_reference_table<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V17(ref info) => info,
            _ => panic!("FileInformation not V17"),
        };

        self.reference_table = ReferenceTable::new(
            info.volume_info_offset + self.references_offset,
            &mut reader
        )?;

        Ok(true)
    }
}
#[derive(Serialize, Debug)]
pub struct VolumeEntryV23 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: WinTimestamp,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32,
    pub unknown2: ByteArray, //[u8; 28],
    pub unknown3: u32,
    pub unknown4: ByteArray, //[u8; 28],
    pub unknown5: u32,
    pub path_string: String,
    pub directory_strings: Vec<String>,
    pub reference_table: ReferenceTable
}
impl VolumeEntryV23{
    pub fn new<R: Read>(mut reader: R) -> Result<VolumeEntryV23,PrefetchError>{
        let mut volume_entry_v23: VolumeEntryV23 = unsafe {
            mem::zeroed()
        };

        volume_entry_v23.path_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v23.path_length = reader.read_u32::<LittleEndian>()?;
        volume_entry_v23.vol_creation_time = WinTimestamp(reader.read_u64::<LittleEndian>()?);
        volume_entry_v23.volume_serial = reader.read_u32::<LittleEndian>()?;
        volume_entry_v23.references_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v23.references_data_size = reader.read_u32::<LittleEndian>()?;
        volume_entry_v23.directory_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v23.directory_string_count = reader.read_u32::<LittleEndian>()?;
        volume_entry_v23.unknown1 = reader.read_u32::<LittleEndian>()?;

        volume_entry_v23.unknown2 = ByteArray(vec![0; 28]);
        reader.read_exact(&mut volume_entry_v23.unknown2.0.as_mut_slice())?;

        volume_entry_v23.unknown3 = reader.read_u32::<LittleEndian>()?;

        volume_entry_v23.unknown4 = ByteArray(vec![0; 28]);
        reader.read_exact(&mut volume_entry_v23.unknown4.0.as_mut_slice())?;

        volume_entry_v23.unknown5 = reader.read_u32::<LittleEndian>()?;

        Ok(volume_entry_v23)
    }
    pub fn get_path_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V23(ref info) => info,
            _ => panic!("FileInformation not V23"),
        };

        self.path_string = filestrings::parse_filename_string(
            info.volume_info_offset + self.path_offset,
            self.path_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_directory_strings<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V23(ref info) => info,
            _ => panic!("FileInformation not V23"),
        };

        self.directory_strings = filestrings::get_string_array(
            info.volume_info_offset + self.directory_offset,
            self.directory_string_count,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_reference_table<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V23(ref info) => info,
            _ => panic!("FileInformation not V23"),
        };

        self.reference_table = ReferenceTable::new(
            info.volume_info_offset + self.references_offset,
            &mut reader
        )?;

        Ok(true)
    }
}
#[derive(Serialize, Debug)]
pub struct VolumeEntryV26 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: WinTimestamp,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32,
    pub unknown2: ByteArray, //[u8; 28],
    pub unknown3: u32,
    pub unknown4: ByteArray, //[u8; 28],
    pub unknown5: u32,
    pub path_string: String,
    pub directory_strings: Vec<String>,
    pub reference_table: ReferenceTable
}
impl VolumeEntryV26{
    pub fn new<R: Read>(mut reader: R) -> Result<VolumeEntryV26,PrefetchError>{
        let mut volume_entry_v26: VolumeEntryV26 = unsafe {
            mem::zeroed()
        };

        volume_entry_v26.path_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v26.path_length = reader.read_u32::<LittleEndian>()?;
        volume_entry_v26.vol_creation_time = WinTimestamp(reader.read_u64::<LittleEndian>()?);
        volume_entry_v26.volume_serial = reader.read_u32::<LittleEndian>()?;
        volume_entry_v26.references_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v26.references_data_size = reader.read_u32::<LittleEndian>()?;
        volume_entry_v26.directory_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v26.directory_string_count = reader.read_u32::<LittleEndian>()?;
        volume_entry_v26.unknown1 = reader.read_u32::<LittleEndian>()?;

        volume_entry_v26.unknown2 = ByteArray(vec![0; 28]);
        reader.read_exact(&mut volume_entry_v26.unknown2.0.as_mut_slice())?;

        volume_entry_v26.unknown3 = reader.read_u32::<LittleEndian>()?;

        volume_entry_v26.unknown4 = ByteArray(vec![0; 28]);
        reader.read_exact(&mut volume_entry_v26.unknown4.0.as_mut_slice())?;

        volume_entry_v26.unknown5 = reader.read_u32::<LittleEndian>()?;

        Ok(volume_entry_v26)
    }
    pub fn get_path_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V26(ref info) => info,
            _ => panic!("FileInformation not V26"),
        };

        self.path_string = filestrings::parse_filename_string(
            info.volume_info_offset + self.path_offset,
            self.path_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_directory_strings<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V26(ref info) => info,
            _ => panic!("FileInformation not V26"),
        };

        self.directory_strings = filestrings::get_string_array(
            info.volume_info_offset + self.directory_offset,
            self.directory_string_count,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_reference_table<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V26(ref info) => info,
            _ => panic!("FileInformation not V26"),
        };

        self.reference_table = ReferenceTable::new(
            info.volume_info_offset + self.references_offset,
            &mut reader
        )?;

        Ok(true)
    }
}
#[derive(Serialize, Debug)]
pub struct VolumeEntryV30 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: WinTimestamp,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32,
    pub unknown2: ByteArray, //[u8; 24],
    pub unknown3: u32,
    pub unknown4: ByteArray, //[u8; 24],
    pub unknown5: u32,
    pub path_string: String,
    pub directory_strings: Vec<String>,
    pub reference_table: ReferenceTable
}
impl VolumeEntryV30{
    pub fn new<R: Read>(mut reader: R) -> Result<VolumeEntryV30,PrefetchError>{
        let mut volume_entry_v30: VolumeEntryV30 = unsafe {
            mem::zeroed()
        };

        volume_entry_v30.path_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v30.path_length = reader.read_u32::<LittleEndian>()?;
        volume_entry_v30.vol_creation_time = WinTimestamp(reader.read_u64::<LittleEndian>()?);
        volume_entry_v30.volume_serial = reader.read_u32::<LittleEndian>()?;
        volume_entry_v30.references_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v30.references_data_size = reader.read_u32::<LittleEndian>()?;
        volume_entry_v30.directory_offset = reader.read_u32::<LittleEndian>()?;
        volume_entry_v30.directory_string_count = reader.read_u32::<LittleEndian>()?;
        volume_entry_v30.unknown1 = reader.read_u32::<LittleEndian>()?;

        volume_entry_v30.unknown2 = ByteArray(vec![0; 24]);
        reader.read_exact(&mut volume_entry_v30.unknown2.0.as_mut_slice())?;

        volume_entry_v30.unknown3 = reader.read_u32::<LittleEndian>()?;

        volume_entry_v30.unknown4 = ByteArray(vec![0; 24]);
        reader.read_exact(&mut volume_entry_v30.unknown4.0.as_mut_slice())?;

        volume_entry_v30.unknown5 = reader.read_u32::<LittleEndian>()?;

        Ok(volume_entry_v30)
    }
    pub fn get_path_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V30(ref info) => info,
            _ => panic!("FileInformation not V30"),
        };

        self.path_string = filestrings::parse_filename_string(
            info.volume_info_offset + self.path_offset,
            self.path_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_directory_strings<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V30(ref info) => info,
            _ => panic!("FileInformation not V30"),
        };

        self.directory_strings = filestrings::get_string_array(
            info.volume_info_offset + self.directory_offset,
            self.directory_string_count,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_reference_table<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V30(ref info) => info,
            _ => panic!("FileInformation not V30"),
        };

        self.reference_table = ReferenceTable::new(
            info.volume_info_offset + self.references_offset,
            &mut reader
        )?;

        Ok(true)
    }
}
