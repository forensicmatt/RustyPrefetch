use librp::prefetch::{PrefetchHeader};
use librp::fileinfo::{FileInformation};
use librp::errors::{PrefetchError};
use librp::filestrings;
use librp::tracechain::{TraceChainArray};
use rwinstructs::reference::{MftReference};
use byteorder::{ReadBytesExt, LittleEndian};
use serde::{ser};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;
use std::mem;

// Additional thanks to James Habben for more research into
// previously unknown struct fields
// http://blog.4n6ir.com/2017/03/windows-prefetch-tech-details-of-new.html

pub static mut SKIP_TRACECHAIN: bool = false;

bitflags! {
    pub flags MetricFlag: u32 {
        const NOT_PREFETCHED      = 0x0001, // blocks should not be prefetched
        const RESOURCE            = 0x0002, // blocks will be loaded as resources, non-executable
        const EXECUTABLE_MEMORY   = 0x0200, // blocks will be loaded into executable memory sections
    }
}

pub fn serialize_metric_flags<S>(&item: &MetricFlag, serializer: S) -> Result<S::Ok, S::Error> where S: ser::Serializer
{
    serializer.serialize_str(&format!("0x{:04X}: {:?}", item.bits(), item))
}

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
        let mut skip_tracechain: bool = false;
        unsafe {
            skip_tracechain = SKIP_TRACECHAIN
        }
        if header.version == 17{
            let mut metric_v17 = MetricEntryV17::new(&mut reader)?;
            metric_v17.get_filename_string(
                fileinfo,
                &mut reader
            )?;
            if !skip_tracechain {
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
            if !skip_tracechain {
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
            if !skip_tracechain {
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
            if !skip_tracechain {
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
    #[serde(serialize_with = "serialize_metric_flags")]
    pub flags: MetricFlag,
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
        metricentry_v17.flags = MetricFlag::from_bits_truncate(
            reader.read_u32::<LittleEndian>()?
        );

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
    pub prefetched_blocks: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    #[serde(serialize_with = "serialize_metric_flags")]
    pub flags: MetricFlag,
    pub file_reference: MftReference,
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
        metricentry_v23.prefetched_blocks = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.filename_offset = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.filename_length = reader.read_u32::<LittleEndian>()?;
        metricentry_v23.flags = MetricFlag::from_bits_truncate(
            reader.read_u32::<LittleEndian>()?
        );
        metricentry_v23.file_reference = MftReference(reader.read_u64::<LittleEndian>()?);

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
    pub prefetched_blocks: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    #[serde(serialize_with = "serialize_metric_flags")]
    pub flags: MetricFlag,
    pub file_reference: MftReference,
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
        metricentry_v26.prefetched_blocks = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.filename_offset = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.filename_length = reader.read_u32::<LittleEndian>()?;
        metricentry_v26.flags = MetricFlag::from_bits_truncate(
            reader.read_u32::<LittleEndian>()?
        );
        metricentry_v26.file_reference = MftReference(reader.read_u64::<LittleEndian>()?);

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
    pub prefetched_blocks: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    #[serde(serialize_with = "serialize_metric_flags")]
    pub flags: MetricFlag,
    pub file_reference: MftReference,
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
        metricentry_v30.prefetched_blocks = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.filename_offset = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.filename_length = reader.read_u32::<LittleEndian>()?;
        metricentry_v30.flags = MetricFlag::from_bits_truncate(
            reader.read_u32::<LittleEndian>()?
        );
        metricentry_v30.file_reference = MftReference(reader.read_u64::<LittleEndian>()?);

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
