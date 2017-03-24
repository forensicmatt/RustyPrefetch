extern crate rustyprefetch;
use rustyprefetch::librp::tracechain;
use rustyprefetch::librp::utils;

#[test]
fn tracechain_v17_test() {
    let v17_tracechain_buffer: &[u8] = &[
        0x01,0x00,0x00,0x00,0x00,0x04,0x00,0x00,0xFA,0xFF,0x07,0x00
    ];

    let v17_tracechain_entry = match tracechain::TraceChainEntryV17::new(v17_tracechain_buffer){
        Ok(v17_tracechain_entry) => v17_tracechain_entry,
        Err(error) => panic!(error)
    };

    assert_eq!(v17_tracechain_entry.next_index, 1);
    assert_eq!(v17_tracechain_entry.block_load_count, 1024);
    assert_eq!(v17_tracechain_entry.unknown1, 250);
    assert_eq!(v17_tracechain_entry.unknown2, 255);
    assert_eq!(v17_tracechain_entry.unknown3, 7);
}

#[test]
fn tracechain_v26_test() {
    let v26_tracechain_buffer: &[u8] = &[
        0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0x01,0x16,0x00
    ];

    let v26_tracechain_entry = match tracechain::TraceChainEntryV26::new(v26_tracechain_buffer){
        Ok(v26_tracechain_entry) => v26_tracechain_entry,
        Err(error) => panic!(error)
    };

    assert_eq!(v26_tracechain_entry.next_index, 1);
    assert_eq!(v26_tracechain_entry.block_load_count, 0);
    assert_eq!(v26_tracechain_entry.unknown1, 3);
    assert_eq!(v26_tracechain_entry.unknown2, 1);
    assert_eq!(v26_tracechain_entry.unknown3, 22);
}

#[test]
fn tracechain_v30_test() {
    let v30_tracechain_buffer: &[u8] = &[
        0x00,0x00,0x00,0x00,0x06,0x00,0x00,0x00
    ];

    let v30_tracechain_entry = match tracechain::TraceChainEntryV30::new(v30_tracechain_buffer){
        Ok(v30_tracechain_entry) => v30_tracechain_entry,
        Err(error) => panic!(error)
    };

    assert_eq!(v30_tracechain_entry.block_load_count, 0);
    assert_eq!(v30_tracechain_entry.unknown1,6);
    assert_eq!(v30_tracechain_entry.unknown2, 0);
    assert_eq!(v30_tracechain_entry.unknown3, 0);
}
