#[derive(Debug)]
pub enum FileInformation{
    FileInformationV17,
    FileInformationV23,
    FileInformationV26,
    FileInformationV30
}
#[derive(Debug)]
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
    pub last_run_time: u64,
    // pub unknown1: [u8; 16],
    pub run_count: u32,
    pub unknown2: u32
}
#[derive(Debug)]
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
    pub last_run_time: u64,
    // pub unknown1: [u8; 16],
    pub run_count: u32,
    pub unknown2: u32,
    // pub unknown4: [u8; 80]
}
#[derive(Debug)]
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
    // pub last_run_time: [u64;8],
    // pub unknown1: [u8; 16],
    pub run_count: u32,
    pub unknown2: u32,
    pub unknown5: u32,
    pub unknown6: u32,
    // pub unknown4: [u8; 88]
}
#[derive(Debug)]
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
    // pub last_run_time: [u64;8],
    // pub unknown1: [u8; 16],
    pub run_count: u32,
    pub unknown2: u32,
    pub unknown5: u32,
    pub unknown6: u32,
    // pub unknown4: [u8; 88]
}
