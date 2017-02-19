#[derive(Debug)]
pub enum VolumeEntry{
    VolumeEntryV17,
    VolumeEntryV23,
    VolumeEntryV26,
    VolumeEntryV30
}
#[derive(Debug)]
pub struct VolumeEntryV17 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: u64,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32
}
#[derive(Debug)]
pub struct VolumeEntryV23 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: u64,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32,
    pub unknown2: [u8; 28],
    pub unknown3: u32,
    pub unknown4: [u8; 28],
    pub unknown5: u32
}
#[derive(Debug)]
pub struct VolumeEntryV26 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: u64,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32,
    pub unknown2: [u8; 28],
    pub unknown3: u32,
    pub unknown4: [u8; 28],
    pub unknown5: u32
}
#[derive(Debug)]
pub struct VolumeEntryV30 {
    pub path_offset: u32, // The offset is relative from the start of the volume information
    pub path_length: u32, // number of characters
    pub vol_creation_time: u64,
    pub volume_serial: u32,
    pub references_offset: u32,
    pub references_data_size: u32,
    pub directory_offset: u32,
    pub directory_string_count: u32,
    pub unknown1: u32,
    pub unknown2: [u8; 24],
    pub unknown3: u32,
    pub unknown4: [u8; 24],
    pub unknown5: u32
}
