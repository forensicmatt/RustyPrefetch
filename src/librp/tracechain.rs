use librp::errors::{PrefetchError};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::SeekFrom;
use std::io::Read;
use std::io::Seek;
use std::mem;

#[derive(Serialize, Debug)]
pub struct TraceChainArray(
    pub Vec<TraceChainEntry>
);

impl TraceChainArray{
    pub fn new<Rs: Read+Seek>(version: u16, chain_start_offset: u32, tracechain_count: u32, mut reader: Rs) -> Result<TraceChainArray,PrefetchError>{
        let mut tracechain_array = TraceChainArray(Vec::new());

        // Get Current Pos
        let cur_pos = reader.seek(
            SeekFrom::Current(0)
        )?;

        // Seek to string offset
        reader.seek(
            SeekFrom::Start(chain_start_offset as u64)
        )?;

        for i in 0..tracechain_count{
            tracechain_array.0.push(
                TraceChainEntry::new(
                    version,
                    &mut reader
                )?
            );
        }

        // Seek back to original offset
        reader.seek(
            SeekFrom::Start(cur_pos)
        )?;

        Ok(tracechain_array)
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum TraceChainEntry{
    V17(TraceChainEntryV17),
    V23(TraceChainEntryV23),
    V26(TraceChainEntryV26),
    V30(TraceChainEntryV30)
}
impl TraceChainEntry{
    pub fn new<R: Read>(version: u16, mut reader: R)-> Result<TraceChainEntry,PrefetchError>{
        if version == 17{
            let tc_entry_v17 = TraceChainEntryV17::new(&mut reader)?;
            Ok(TraceChainEntry::V17(tc_entry_v17))
        } else if version == 23 {
            let tc_entry_v23 = TraceChainEntryV23::new(&mut reader)?;
            Ok(TraceChainEntry::V23(tc_entry_v23))
        } else if version == 26 {
            let tc_entry_v26 = TraceChainEntryV26::new(&mut reader)?;
            Ok(TraceChainEntry::V26(tc_entry_v26))
        } else if version == 30 {
            let tc_entry_v30 = TraceChainEntryV30::new(&mut reader)?;
            Ok(TraceChainEntry::V30(tc_entry_v30))
        } else {
            Err(PrefetchError::parse_error(
                format!("Error parsing trace chain entry. Invalid version: {}",version)
            ))
        }
    }
}
#[derive(Serialize, Debug)]
pub struct TraceChainEntryV17 {
    pub next_index: u32,
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
impl TraceChainEntryV17 {
    pub fn new<R: Read>(mut reader: R) -> Result<TraceChainEntryV17,PrefetchError>{
        let mut tc_entry_v17: TraceChainEntryV17 = unsafe {
            mem::zeroed()
        };

        tc_entry_v17.next_index = reader.read_u32::<LittleEndian>()?;
        tc_entry_v17.block_load_count = reader.read_u32::<LittleEndian>()?;
        tc_entry_v17.unknown1 = reader.read_u8()?;
        tc_entry_v17.unknown2 = reader.read_u8()?;
        tc_entry_v17.unknown3 = reader.read_u16::<LittleEndian>()?;

        Ok(tc_entry_v17)
    }
}
#[derive(Serialize, Debug)]
pub struct TraceChainEntryV23 {
    pub next_index: u32,
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
impl TraceChainEntryV23 {
    pub fn new<R: Read>(mut reader: R) -> Result<TraceChainEntryV23,PrefetchError>{
        let mut tc_entry_v23: TraceChainEntryV23 = unsafe {
            mem::zeroed()
        };

        tc_entry_v23.next_index = reader.read_u32::<LittleEndian>()?;
        tc_entry_v23.block_load_count = reader.read_u32::<LittleEndian>()?;
        tc_entry_v23.unknown1 = reader.read_u8()?;
        tc_entry_v23.unknown2 = reader.read_u8()?;
        tc_entry_v23.unknown3 = reader.read_u16::<LittleEndian>()?;

        Ok(tc_entry_v23)
    }
}
#[derive(Serialize, Debug)]
pub struct TraceChainEntryV26 {
    pub next_index: u32,
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
impl TraceChainEntryV26 {
    pub fn new<R: Read>(mut reader: R) -> Result<TraceChainEntryV26,PrefetchError>{
        let mut tc_entry_v26: TraceChainEntryV26 = unsafe {
            mem::zeroed()
        };

        tc_entry_v26.next_index = reader.read_u32::<LittleEndian>()?;
        tc_entry_v26.block_load_count = reader.read_u32::<LittleEndian>()?;
        tc_entry_v26.unknown1 = reader.read_u8()?;
        tc_entry_v26.unknown2 = reader.read_u8()?;
        tc_entry_v26.unknown3 = reader.read_u16::<LittleEndian>()?;

        Ok(tc_entry_v26)
    }
}
#[derive(Serialize, Debug)]
pub struct TraceChainEntryV30 {
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
impl TraceChainEntryV30 {
    pub fn new<R: Read>(mut reader: R) -> Result<TraceChainEntryV30,PrefetchError>{
        let mut tc_entry_v30: TraceChainEntryV30 = unsafe {
            mem::zeroed()
        };

        tc_entry_v30.block_load_count = reader.read_u32::<LittleEndian>()?;
        tc_entry_v30.unknown1 = reader.read_u8()?;
        tc_entry_v30.unknown2 = reader.read_u8()?;
        tc_entry_v30.unknown3 = reader.read_u16::<LittleEndian>()?;

        Ok(tc_entry_v30)
    }
}
