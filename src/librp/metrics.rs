use librp::prefetch::{PrefetchHeader};
use librp::fileinfo::{FileInformation};
use librp::errors::{PrefetchError};
use librp::filestrings;
use librp::tracechain::{TraceChainArray};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;
use std::mem;

pub static mut SKIP_TRACECHAIN: bool = false;

#[derive(Serialize, Debug)]
pub struct MetricsArray(
    pub Vec<MetricEntry>
);
impl MetricsArray{
    pub fn new<R: Read + Seek>(header: &PrefetchHeader,fileinfo: &FileInformation,mut reader: R) -> Result<MetricsArray,PrefetchError>{
        let mut metrics_array = MetricsArray(Vec::new());
        let mut metrics_array_offset: u64 = 0;
        let mut metrics_entry_count: u32 = 0;
        let mut filename_offset: u32 = 0;

        match *fileinfo {
            FileInformation::V17(ref info) => {
                metrics_array_offset = info.metrics_array_offset as u64;
                metrics_entry_count = info.metrics_entry_count;
                filename_offset = info.filename_offset;
            },
            FileInformation::V23(ref info) => {
                metrics_array_offset = info.metrics_array_offset as u64;
                metrics_entry_count = info.metrics_entry_count;
                filename_offset = info.filename_offset;
            },
            FileInformation::V26(ref info) => {
                metrics_array_offset = info.metrics_array_offset as u64;
                metrics_entry_count = info.metrics_entry_count;
                filename_offset = info.filename_offset;
            },
            FileInformation::V30(ref info) => {
                metrics_array_offset = info.metrics_array_offset as u64;
                metrics_entry_count = info.metrics_entry_count;
                filename_offset = info.filename_offset;
            }
        }

        // Seek to metrics array
        reader.seek(
            SeekFrom::Start(metrics_array_offset)
        )?;

        // read number of entries
        for i in 0..metrics_entry_count {
            metrics_array.0.push(
                MetricEntry::new(
                    header,
                    fileinfo,
                    &mut reader
                )?
            );
        }

        Ok(metrics_array)
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum MetricEntry{
    V17(MetricEntryV17),
    V23(MetricEntryV23),
    V26(MetricEntryV26),
    V30(MetricEntryV30)
}

impl MetricEntry{
    pub fn new<Rs: Read+Seek>(header: &PrefetchHeader, fileinfo: &FileInformation, mut reader: Rs) -> Result<MetricEntry,PrefetchError> {
        if header.version == 17{
            let mut metric_v17 = MetricEntryV17::new(&mut reader)?;
            metric_v17.get_filename_string(
                fileinfo,
                &mut reader
            )?;
            if unsafe{SKIP_TRACECHAIN} {
                metric_v17.get_tracechains(
                    fileinfo,
                    &mut reader
                )?;
            }
            Ok(MetricEntry::V17(metric_v17))
        } else if header.version == 23{
            let mut metric_v23 = MetricEntryV23::new(&mut reader)?;
            metric_v23.get_filename_string(
                fileinfo,
                &mut reader
            )?;
            if unsafe{SKIP_TRACECHAIN} {
                metric_v23.get_tracechains(
                    fileinfo,
                    &mut reader
                )?;
            }
            Ok(MetricEntry::V23(metric_v23))
        } else if header.version == 26{
            let mut metric_v26 = MetricEntryV26::new(&mut reader)?;
            metric_v26.get_filename_string(
                fileinfo,
                &mut reader
            )?;
            if unsafe{SKIP_TRACECHAIN} {
                metric_v26.get_tracechains(
                    fileinfo,
                    &mut reader
                )?;
            }
            Ok(MetricEntry::V26(metric_v26))
        } else if header.version == 30{
            let mut metric_v30 = MetricEntryV30::new(&mut reader)?;
            metric_v30.get_filename_string(
                fileinfo,
                &mut reader
            )?;
            if unsafe{SKIP_TRACECHAIN} {
                metric_v30.get_tracechains(
                    fileinfo,
                    &mut reader
                )?;
            }
            Ok(MetricEntry::V30(metric_v30))
        } else {
            Err(PrefetchError::parse_error(
                format!("Error parsing file info. Invalid version: {}",header.version)
            ))
        }
    }
}
#[derive(Serialize, Debug)]
pub struct MetricEntryV17{
    pub tracechain_index: u32,
    pub tracechain_count: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub filename: String,
    pub tracechain: TraceChainArray
}
impl MetricEntryV17{
    pub fn new<R: Read>(mut reader: R) -> Result<MetricEntryV17,PrefetchError>{
        let mut metricentry_v17: MetricEntryV17 = unsafe {
            mem::zeroed()
        };

        metricentry_v17.tracechain_index = reader.read_u32::<LittleEndian>()?;
        metricentry_v17.tracechain_count = reader.read_u32::<LittleEndian>()?;
        metricentry_v17.filename_offset = reader.read_u32::<LittleEndian>()?;
        metricentry_v17.filename_length = reader.read_u32::<LittleEndian>()?;
        metricentry_v17.unknown3 = reader.read_u32::<LittleEndian>()?;

        Ok(metricentry_v17)
    }
    pub fn get_filename_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V17(ref info) => info,
            _ => panic!("FileInformation not V17"),
        };

        self.filename = filestrings::parse_filename_string(
            info.filename_offset + self.filename_offset,
            self.filename_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_tracechains<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V17(ref info) => info,
            _ => panic!("FileInformation not V17"),
        };

        // trace chain size for v17 is 12 bytes
        let chain_start_offset = info.trace_array_offset + (self.tracechain_index * 12);

        self.tracechain = TraceChainArray::new(
            17,
            chain_start_offset, //start12bytes
            self.tracechain_count,
            &mut reader
        )?;

        Ok(true)
    }
}
#[derive(Serialize, Debug)]
pub struct MetricEntryV23{
    pub tracechain_index: u32,
    pub tracechain_count: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u64,
    pub filename: String,
    pub tracechain: TraceChainArray
}
impl MetricEntryV23{
    pub fn new<R: Read>(mut reader: R) -> Result<MetricEntryV23,PrefetchError>{
        let mut metricentry_v23: MetricEntryV23 = unsafe {
            mem::zeroed()
        };

        metricentry_v23.tracechain_index = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.tracechain_count = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.unknown4 = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.filename_offset = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.filename_length = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.unknown3 = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.file_reference = reader.read_u64::<LittleEndian>()?;

        Ok(metricentry_v23)
    }
    pub fn get_filename_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V23(ref info) => info,
            _ => panic!("FileInformation not V23"),
        };

        self.filename = filestrings::parse_filename_string(
            info.filename_offset + self.filename_offset,
            self.filename_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_tracechains<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V23(ref info) => info,
            _ => panic!("FileInformation not V23"),
        };

        // trace chain size for v23 is 12 bytes
        let chain_start_offset = info.trace_array_offset + (self.tracechain_index * 12);

        self.tracechain = TraceChainArray::new(
            23,
            chain_start_offset, //start12bytes
            self.tracechain_count,
            &mut reader
        )?;

        Ok(true)
    }
}
#[derive(Serialize, Debug)]
pub struct MetricEntryV26{
    pub tracechain_index: u32,
    pub tracechain_count: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u64,
    pub filename: String,
    pub tracechain: TraceChainArray
}
impl MetricEntryV26{
    pub fn new<R: Read>(mut reader: R) -> Result<MetricEntryV26,PrefetchError>{
        let mut metricentry_v26: MetricEntryV26 = unsafe {
            mem::zeroed()
        };

        metricentry_v26.tracechain_index = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.tracechain_count = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.unknown4 = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.filename_offset = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.filename_length = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.unknown3 = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.file_reference = reader.read_u64::<LittleEndian>()?;

        Ok(metricentry_v26)
    }
    pub fn get_filename_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V26(ref info) => info,
            _ => panic!("FileInformation not V26"),
        };

        self.filename = filestrings::parse_filename_string(
            info.filename_offset + self.filename_offset,
            self.filename_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_tracechains<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V26(ref info) => info,
            _ => panic!("FileInformation not V26"),
        };

        // trace chain size for v26 is 12 bytes
        let chain_start_offset = info.trace_array_offset + (self.tracechain_index * 12);

        self.tracechain = TraceChainArray::new(
            26,
            chain_start_offset, //start12bytes
            self.tracechain_count,
            &mut reader
        )?;

        Ok(true)
    }
}
#[derive(Serialize, Debug)]
pub struct MetricEntryV30{
    pub tracechain_index: u32,
    pub tracechain_count: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u64,
    pub filename: String,
    pub tracechain: TraceChainArray
}
impl MetricEntryV30{
    pub fn new<R: Read>(mut reader: R) -> Result<MetricEntryV30,PrefetchError>{
        let mut metricentry_v30: MetricEntryV30 = unsafe {
            mem::zeroed()
        };

        metricentry_v30.tracechain_index = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.tracechain_count = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.unknown4 = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.filename_offset = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.filename_length = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.unknown3 = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.file_reference = reader.read_u64::<LittleEndian>()?;

        Ok(metricentry_v30)
    }
    pub fn get_filename_string<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V30(ref info) => info,
            _ => panic!("FileInformation not V30"),
        };

        self.filename = filestrings::parse_filename_string(
            info.filename_offset + self.filename_offset,
            self.filename_length,
            &mut reader
        )?;

        Ok(true)
    }
    pub fn get_tracechains<Rs: Read+Seek>(&mut self,fileinfo: &FileInformation, mut reader: Rs)->Result<bool,PrefetchError>{
        let info = match *fileinfo {
            FileInformation::V30(ref info) => info,
            _ => panic!("FileInformation not V30"),
        };

        // trace chain size for v30 is 8 bytes
        let chain_start_offset = info.trace_array_offset + (self.tracechain_index * 8);

        self.tracechain = TraceChainArray::new(
            30,
            chain_start_offset,
            self.tracechain_count,
            &mut reader
        )?;

        Ok(true)
    }
}
