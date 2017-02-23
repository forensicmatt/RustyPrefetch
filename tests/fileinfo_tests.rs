extern crate rustyprefetch;
use rustyprefetch::librp::fileinfo;

#[test]
fn fileinfo_v26_test() {
    let v26_fileinfo_buffer: &[u8] = &[
        0x30,0x01,0x00,0x00,0x87,0x00,0x00,0x00,0x10,0x12,0x00,0x00,0xDA,0x28,0x00,0x00,
        0x48,0xFC,0x01,0x00,0x2C,0x47,0x00,0x00,0x78,0x43,0x02,0x00,0x01,0x00,0x00,0x00,
        0x44,0x0F,0x00,0x00,0x1B,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x7A,0x8D,0xB2,0x2B,
        0x98,0xCE,0xCE,0x01,0x4A,0xC7,0xCC,0x2B,0x98,0xCE,0xCE,0x01,0xF9,0x48,0x1A,0x59,
        0x31,0xC0,0xCE,0x01,0xD2,0xF0,0xDE,0x58,0x31,0xC0,0xCE,0x01,0xDA,0x01,0x9B,0x7F,
        0x4B,0xBE,0xCE,0x01,0xD2,0x11,0x85,0x7F,0x4B,0xBE,0xCE,0x01,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x06,0x00,0x00,0x00,
        0x06,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
    ];

    let v26_fileinfo = match fileinfo::get_fileinfo_v26(v26_fileinfo_buffer){
        Ok(v26_fileinfo) => v26_fileinfo,
        Err(error) => panic!(error)
    };

    assert_eq!(v26_fileinfo.metrics_array_offset, 304);
    assert_eq!(v26_fileinfo.metrics_entry_count, 135);
    assert_eq!(v26_fileinfo.trace_array_offset, 4624);
    assert_eq!(v26_fileinfo.trace_entry_count, 10458);
    assert_eq!(v26_fileinfo.filename_offset, 130120);
    assert_eq!(v26_fileinfo.filename_length, 18220);
    assert_eq!(v26_fileinfo.volume_info_offset, 148344);
    assert_eq!(v26_fileinfo.volume_info_count, 1);
    assert_eq!(v26_fileinfo.volume_info_size, 3908);
    assert_eq!(v26_fileinfo.unknown3, 4294967323);
    assert_eq!(v26_fileinfo.run_count, 6);
    assert_eq!(v26_fileinfo.unknown2, 6);
    assert_eq!(v26_fileinfo.unknown5, 3);
}