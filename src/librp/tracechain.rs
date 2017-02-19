#[derive(Debug)]
pub enum TraceChainEntry{
    TraceChainEntryV17,
    TraceChainEntryV23,
    TraceChainEntryV26,
    TraceChainEntryV30
}
#[derive(Debug)]
pub struct TraceChainEntryV17 {
    pub next_index: u32,
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
#[derive(Debug)]
pub struct TraceChainEntryV23 {
    pub next_index: u32,
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
#[derive(Debug)]
pub struct TraceChainEntryV26 {
    pub next_index: u32,
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
#[derive(Debug)]
pub struct TraceChainEntryV30 {
    pub block_load_count: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u16
}
