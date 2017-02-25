use librp::prefetch::{PrefetchHeader};
use librp::fileinfo::{FileInformation};
use librp::errors::{PrefetchError};
use librp::filestrings;
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;
use std::mem;

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
                    filename_offset,
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
#[derive(Serialize, Debug)]
pub struct MetricEntryV17{
    pub unknown1: u32,
    pub unknown2: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub filename: String
}
#[derive(Serialize, Debug)]
pub struct MetricEntryV23{
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u64,
    pub filename: String
}
#[derive(Serialize, Debug)]
pub struct MetricEntryV26{
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u64,
    pub filename: String
}
#[derive(Serialize, Debug)]
pub struct MetricEntryV30{
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown4: u32,
    pub filename_offset: u32, //The offset is relative to the start of the filename strings
    pub filename_length: u32, //num of chars (not including end of strign char)
    pub unknown3: u32,
    pub file_reference: u64,
    pub filename: String
}

impl MetricEntry{
    pub fn new<R: Read + Seek>(header: &PrefetchHeader, filename_offset: u32, mut reader: R) -> Result<MetricEntry,PrefetchError> {
        if header.version == 17{
            Ok(
                MetricEntry::V17(
                    get_metricsinfo_v17(filename_offset,&mut reader)?
                )
            )
        } else if header.version == 23{
            Ok(
                MetricEntry::V23(get_metricsinfo_v23(filename_offset,&mut reader)?)
            )
        } else if header.version == 26{
            Ok(
                MetricEntry::V26(get_metricsinfo_v26(filename_offset,&mut reader)?)
            )
        } else if header.version == 30{
            Ok(
                MetricEntry::V30(get_metricsinfo_v30(filename_offset,&mut reader)?)
            )
        } else {
            Err(PrefetchError::parse_error(
                format!("Error parsing file info. Invalid version: {}",header.version)
            ))
        }
    }
}

pub fn get_metricsinfo_v17<R: Read + Seek>(filename_offset: u32,mut reader: R) -> Result<MetricEntryV17,PrefetchError> {
    let mut metricentry_v17: MetricEntryV17 = unsafe {
        mem::zeroed()
    };

    metricentry_v17.unknown1 = reader.read_u32::<LittleEndian>()?;
    metricentry_v17.unknown2 = reader.read_u32::<LittleEndian>()?;
    metricentry_v17.filename_offset = reader.read_u32::<LittleEndian>()?;
    metricentry_v17.filename_length = reader.read_u32::<LittleEndian>()?;
    metricentry_v17.unknown3 = reader.read_u32::<LittleEndian>()?;

    metricentry_v17.filename = filestrings::parse_filename_string(
        filename_offset + metricentry_v17.filename_offset,
        metricentry_v17.filename_length,
        &mut reader
    )?;

    Ok(metricentry_v17)
}

pub fn get_metricsinfo_v23<R: Read + Seek>(filename_offset: u32,mut reader: R) -> Result<MetricEntryV23,PrefetchError> {
    let mut metricentry_v23: MetricEntryV23 = unsafe {
        mem::zeroed()
    };

    metricentry_v23.unknown1 = reader.read_u32::<LittleEndian>()?;
    metricentry_v23.unknown2 = reader.read_u32::<LittleEndian>()?;
    metricentry_v23.unknown4 = reader.read_u32::<LittleEndian>()?;
    metricentry_v23.filename_offset = reader.read_u32::<LittleEndian>()?;
    metricentry_v23.filename_length = reader.read_u32::<LittleEndian>()?;
    metricentry_v23.unknown3 = reader.read_u32::<LittleEndian>()?;
    metricentry_v23.file_reference = reader.read_u64::<LittleEndian>()?;

    metricentry_v23.filename = filestrings::parse_filename_string(
        filename_offset + metricentry_v23.filename_offset,
        metricentry_v23.filename_length,
        &mut reader
    )?;

    Ok(metricentry_v23)
}

pub fn get_metricsinfo_v26<R: Read + Seek>(filename_offset: u32, mut reader: R) -> Result<MetricEntryV26,PrefetchError> {
    let mut metricentry_v26: MetricEntryV26 = unsafe {
        mem::zeroed()
    };

    metricentry_v26.unknown1 = reader.read_u32::<LittleEndian>()?;
    metricentry_v26.unknown2 = reader.read_u32::<LittleEndian>()?;
    metricentry_v26.unknown4 = reader.read_u32::<LittleEndian>()?;
    metricentry_v26.filename_offset = reader.read_u32::<LittleEndian>()?;
    metricentry_v26.filename_length = reader.read_u32::<LittleEndian>()?;
    metricentry_v26.unknown3 = reader.read_u32::<LittleEndian>()?;
    metricentry_v26.file_reference = reader.read_u64::<LittleEndian>()?;

    metricentry_v26.filename = filestrings::parse_filename_string(
        filename_offset + metricentry_v26.filename_offset,
        metricentry_v26.filename_length,
        &mut reader
    )?;

    Ok(metricentry_v26)
}

pub fn get_metricsinfo_v30<R: Read + Seek>(filename_offset: u32,mut reader: R) -> Result<MetricEntryV30,PrefetchError> {
    let mut metricentry_v30: MetricEntryV30 = unsafe {
        mem::zeroed()
    };

    metricentry_v30.unknown1 = reader.read_u32::<LittleEndian>()?;
    metricentry_v30.unknown2 = reader.read_u32::<LittleEndian>()?;
    metricentry_v30.unknown4 = reader.read_u32::<LittleEndian>()?;
    metricentry_v30.filename_offset = reader.read_u32::<LittleEndian>()?;
    metricentry_v30.filename_length = reader.read_u32::<LittleEndian>()?;
    metricentry_v30.unknown3 = reader.read_u32::<LittleEndian>()?;
    metricentry_v30.file_reference = reader.read_u64::<LittleEndian>()?;

    metricentry_v30.filename = filestrings::parse_filename_string(
        filename_offset + metricentry_v30.filename_offset,
        metricentry_v30.filename_length,
        &mut reader
    )?;

    Ok(metricentry_v30)
}
