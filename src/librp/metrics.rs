#[derive(Debug)]
pub enum MetricEntry{
    MetricEntryV17,
    MetricEntryV23,
    MetricEntryV26,
    MetricEntryV30
}
#[derive(Debug)]
pub struct MetricEntryV17{
    pub unknown1: u32,
    pub unknown2: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32
}
pub struct MetricEntryV23{
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u32,
}
pub struct MetricEntryV26{
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u32,
}
pub struct MetricEntryV30{
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u32,
}
