extern crate rustyprefetch;
use rustyprefetch::librp::metrics;
use rustyprefetch::librp::utils;

#[test]
fn metric_v17_test() {
    let v17_metric_buffer: &[u8] = &[
        0x00,0x00,0x00,0x00,0x39,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x32,0x00,0x00,0x00,
        0x02,0x00,0x00,0x00
    ];

    let v17_metric_entry = match metrics::MetricEntryV17::new(v17_metric_buffer){
        Ok(v17_metric_entry) => v17_metric_entry,
        Err(error) => panic!(error)
    };

    assert_eq!(v17_metric_entry.tracechain_index, 0);
    assert_eq!(v17_metric_entry.tracechain_count, 57);
    assert_eq!(v17_metric_entry.filename_offset, 0);
    assert_eq!(v17_metric_entry.filename_length, 50);
    assert_eq!(v17_metric_entry.unknown3, 2);
}

#[test]
fn metric_v26_test() {
    let v26_metric_buffer: &[u8] = &[
        0x00,0x00,0x00,0x00,0x29,0x01,0x00,0x00,0x21,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x51,0x00,0x00,0x00,0x00,0x02,0x00,0x00,0xEA,0xD0,0x02,0x00,0x00,0x00,0x09,0x00
    ];

    let v26_metric_entry = match metrics::MetricEntryV26::new(v26_metric_buffer){
        Ok(v26_metric_entry) => v26_metric_entry,
        Err(error) => panic!(error)
    };

    assert_eq!(v26_metric_entry.tracechain_index, 0);
    assert_eq!(v26_metric_entry.tracechain_count, 297);
    assert_eq!(v26_metric_entry.unknown4, 33);
    assert_eq!(v26_metric_entry.filename_offset, 0);
    assert_eq!(v26_metric_entry.filename_length, 81);
    assert_eq!(v26_metric_entry.unknown3, 512);
    assert_eq!(v26_metric_entry.file_reference, 2533274790580458);
}

#[test]
fn metric_v30_test() {
    let v30_metric_buffer: &[u8] = &[
        0x00,0x00,0x00,0x00,0x2C,0x00,0x00,0x00,0x2A,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x52,0x00,0x00,0x00,0x02,0x01,0x00,0x00,0x5B,0x3C,0x00,0x00,0x00,0x00,0x04,0x01
    ];

    let v30_metric_entry = match metrics::MetricEntryV30::new(v30_metric_buffer){
        Ok(v30_metric_entry) => v30_metric_entry,
        Err(error) => panic!(error)
    };

    assert_eq!(v30_metric_entry.tracechain_index, 0);
    assert_eq!(v30_metric_entry.tracechain_count, 44);
    assert_eq!(v30_metric_entry.unknown4, 42);
    assert_eq!(v30_metric_entry.filename_offset, 0);
    assert_eq!(v30_metric_entry.filename_length, 82);
    assert_eq!(v30_metric_entry.unknown3, 258);
    assert_eq!(v30_metric_entry.file_reference, 73183493944786011);
}
