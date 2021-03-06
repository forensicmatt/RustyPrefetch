extern crate rustyprefetch;
use rustyprefetch::librp::fileinfo;
use rustyprefetch::librp::utils;

#[test]
fn fileinfo_v17_test() {
    let v17_fileinfo_buffer: &[u8] = &[
        0x98,0x00,0x00,0x00,0x2C,0x00,0x00,0x00,0x08,0x04,0x00,0x00,0xED,0x11,0x00,0x00,
        0x24,0xDB,0x00,0x00,0x6E,0x16,0x00,0x00,0x98,0xF1,0x00,0x00,0x01,0x00,0x00,0x00,
        0x7E,0x04,0x00,0x00,0x60,0xE8,0x89,0x95,0x35,0x11,0xCD,0x01,0x00,0x8C,0x86,0x47,
        0x00,0x00,0x00,0x00,0x00,0x8C,0x86,0x47,0x00,0x00,0x00,0x00,0x11,0x00,0x00,0x00,
        0x05,0x00,0x00,0x00
    ];

    let v17_fileinfo = match fileinfo::get_fileinfo_v17(v17_fileinfo_buffer){
        Ok(v17_fileinfo) => v17_fileinfo,
        Err(error) => panic!(error)
    };

    assert_eq!(v17_fileinfo.metrics_array_offset, 152);
    assert_eq!(v17_fileinfo.metrics_entry_count, 44);
    assert_eq!(v17_fileinfo.trace_array_offset, 1032);
    assert_eq!(v17_fileinfo.trace_entry_count, 4589);
    assert_eq!(v17_fileinfo.filename_offset, 56100);
    assert_eq!(v17_fileinfo.filename_length, 5742);
    assert_eq!(v17_fileinfo.volume_info_offset, 61848);
    assert_eq!(v17_fileinfo.volume_info_count, 1);
    assert_eq!(v17_fileinfo.volume_info_size, 1150);
    assert_eq!(v17_fileinfo.last_run_time.0, 129778886103394400);
    assert_eq!(utils::to_hex_string(&v17_fileinfo.unknown1.0), "008C864700000000008C864700000000");
    assert_eq!(v17_fileinfo.run_count, 17);
    assert_eq!(v17_fileinfo.unknown2, 5);
}

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
    assert_eq!(v26_fileinfo.last_run_time[0].0, 130268592203795834);
    assert_eq!(v26_fileinfo.last_run_time[1].0, 130268592205514570);
    assert_eq!(v26_fileinfo.last_run_time[2].0, 130252757421148409);
    assert_eq!(v26_fileinfo.last_run_time[3].0, 130252757417259218);
    assert_eq!(v26_fileinfo.last_run_time[4].0, 130250670713012698);
    assert_eq!(v26_fileinfo.last_run_time[5].0, 130250670711574994);
    assert_eq!(v26_fileinfo.last_run_time[6].0, 0);
    assert_eq!(v26_fileinfo.last_run_time[7].0, 0);
    assert_eq!(
        utils::to_hex_string(&v26_fileinfo.unknown1.0),
        "00000000000000000000000000000000"
    );
    assert_eq!(v26_fileinfo.run_count, 6);
    assert_eq!(v26_fileinfo.unknown2, 6);
    assert_eq!(v26_fileinfo.unknown5, 3);
    assert_eq!(
        utils::to_hex_string(&v26_fileinfo.unknown4.0),
        "000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000"
    );
}

#[test]
fn fileinfo_v30_test() {
    let v30_fileinfo_buffer: &[u8] = &[
        0x30,0x01,0x00,0x00,0xEE,0x00,0x00,0x00,0xF0,0x1E,0x00,0x00,0xFB,0x31,0x00,0x00,
        0xC8,0xAE,0x01,0x00,0x26,0xAB,0x00,0x00,0xF0,0x59,0x02,0x00,0x01,0x00,0x00,0x00,
        0x64,0x27,0x00,0x00,0x38,0x00,0x00,0x00,0x02,0x00,0x00,0x00,0x3F,0x9B,0x0C,0x92,
        0x54,0x9C,0xD2,0x01,0x73,0xA0,0xF9,0x0C,0x4B,0x9C,0xD2,0x01,0xC8,0xF9,0x44,0x76,
        0x2D,0x9C,0xD2,0x01,0x04,0x05,0x1B,0x02,0x99,0x9B,0xD2,0x01,0x10,0x91,0x2A,0x01,
        0x99,0x9B,0xD2,0x01,0x44,0xF7,0x75,0xD5,0xAA,0x9A,0xD2,0x01,0x7C,0x2C,0x99,0xD4,
        0xAA,0x9A,0xD2,0x01,0xC9,0xC0,0x73,0xAE,0x81,0x9A,0xD2,0x01,0x00,0x8C,0x86,0x47,
        0x00,0x00,0x00,0x00,0x00,0x8C,0x86,0x47,0x00,0x00,0x00,0x00,0x66,0x00,0x00,0x00,
        0x06,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
    ];

    let v30_fileinfo = match fileinfo::get_fileinfo_v30(v30_fileinfo_buffer){
        Ok(v30_fileinfo) => v30_fileinfo,
        Err(error) => panic!(error)
    };

    assert_eq!(v30_fileinfo.metrics_array_offset, 304);
    assert_eq!(v30_fileinfo.metrics_entry_count, 238);
    assert_eq!(v30_fileinfo.trace_array_offset, 7920);
    assert_eq!(v30_fileinfo.trace_entry_count, 12795);
    assert_eq!(v30_fileinfo.filename_offset, 110280);
    assert_eq!(v30_fileinfo.filename_length, 43814);
    assert_eq!(v30_fileinfo.volume_info_offset, 154096);
    assert_eq!(v30_fileinfo.volume_info_count, 1);
    assert_eq!(v30_fileinfo.volume_info_size, 10084);
    assert_eq!(v30_fileinfo.unknown3, 8589934648);
    assert_eq!(v30_fileinfo.last_run_time[0].0, 131339226188651327);
    assert_eq!(v30_fileinfo.last_run_time[1].0, 131339185301332083);
    assert_eq!(v30_fileinfo.last_run_time[2].0, 131339058218858952);
    assert_eq!(v30_fileinfo.last_run_time[3].0, 131338420614792452);
    assert_eq!(v30_fileinfo.last_run_time[4].0, 131338420599034128);
    assert_eq!(v30_fileinfo.last_run_time[5].0, 131337397663561540);
    assert_eq!(v30_fileinfo.last_run_time[6].0, 131337397649091708);
    assert_eq!(v30_fileinfo.last_run_time[7].0, 131337220915445961);
    assert_eq!(v30_fileinfo.run_count, 102);
    assert_eq!(
        utils::to_hex_string(&v30_fileinfo.unknown1.0),
        "008C864700000000008C864700000000"
    );
    assert_eq!(v30_fileinfo.unknown2, 6);
    assert_eq!(v30_fileinfo.unknown5, 3);
    assert_eq!(
        utils::to_hex_string(&v30_fileinfo.unknown4.0),
        "00000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000\
        0000000000000"
    );
}
