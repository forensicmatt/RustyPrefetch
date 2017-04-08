# RustyPrefetch
Just another Prefetch parser...

# Build
To build you must set LIBFWNT_BIN to the path that contains the compiled libscca library. You must also copy the compiled library to the same folder as the compiled rust tools. At some point I will try and copy the compiled library from LIBFWNT_BIN to the compiled paths from the build... but for now it is a manual job

To build RustyPrefetch
```cargo build --release```

## Tools
```
RustyPrefetch 0.1.0
Matthew Seyer <https://github.com/forensicmatt/RustyPrefetch>
Parse prefetch.

USAGE:
    RustyPrefetch.exe [FLAGS] --source <FILE>

FLAGS:
    -h, --help          Prints help information
    -t, --tracechain    Output Tracechains
    -V, --version       Prints version information

OPTIONS:
    -s, --source <FILE>    The source path. Can be a file or a directory.
```

```
RustyPrefetch\target\release\examples>DecompressPrefetch.exe -h
DecompressPrefetch 0.1.0
Matthew Seyer <https://github.com/forensicmatt/RustyPrefetch>
Test tool to decompress a compressed prefetch file.

USAGE:
    DecompressPrefetch.exe --prefetch <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --prefetch <FILE>    The Prefetch file to decode
```

# JSON Serialization Features
By default mft reference structures are serialized into a nested structure with a the u64 reference displayed as a string. Though, this is controled by a global variable and can be changed to show either a String or u64 representation of the u64 value. The file reference is only stored as a u64 and only enumerates on serialization. Examples of serialization options can be seen here: https://github.com/forensicmatt/r-winstructs/blob/master/examples/reference_examples.rs.
```
"file_reference": {
  "reference": "2533274790580458",
  "entry": 184554,
  "sequence": 9
},
```

# Example output
```RustyPrefetch.exe -p FIREFOX.EXE-28641590.pf```

```json
[
  {
    "source_file": "..\\testfiles\\Prefetch\\FIREFOX.EXE-6F7B2AEE.pf",
    "header": {
      "version": 30,
      "signature": 1094927187,
      "unknown1": 17,
      "filesize": 164180,
      "filename": "FIREFOX.EXE",
      "hash": 1870342894,
      "unknon2": 0
    },
    "fileinfo": {
      "metrics_array_offset": 304,
      "metrics_entry_count": 238,
      "trace_array_offset": 7920,
      "trace_entry_count": 12795,
      "filename_offset": 110280,
      "filename_length": 43814,
      "volume_info_offset": 154096,
      "volume_info_count": 1,
      "volume_info_size": 10084,
      "unknown3": 8589934648,
      "last_run_time": [
        "2017-03-13 23:50:18.865",
        ...
      ],
      "unknown1": "008C864700000000008C864700000000",
      "run_count": 102,
      "unknown2": 6,
      "unknown5": 3,
      "unknown4": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    },
    "metrics": [
      {
        "tracechain_index": 0,
        "tracechain_count": 44,
        "prefetched_blocks": 42,
        "filename_offset": 0,
        "filename_length": 82,
        "flags": "0x0002: RESOURCE",
        "file_reference": {
          "reference": "73183493944786011",
          "entry": 15451,
          "sequence": 260
        },
        "filename": "\\VOLUME{01cfdb43985a7134-dc989894}\\PROGRAM FILES (X86)\\MOZILLA FIREFOX\\MOZGLUE.DLL",
        "tracechain": [
          {
            "block_load_count": 0,
            "flags": "00000110: EXECUTABLE | RESOURCE",
            "unknown2": 0,
            "usage": "00000000",
            "prefetched": "00000000"
          },
          ...
        ]
      },
      ...
    ],
    "volumes": [
      {
        "path_offset": 96,
        "path_length": 34,
        "vol_creation_time": "2014-09-28 17:42:38.238",
        "volume_serial": 3700988052,
        "references_offset": 168,
        "references_data_size": 1560,
        "directory_offset": 1728,
        "directory_string_count": 56,
        "unknown1": 137,
        "unknown2": "000000000000000000000000000000000000000000000000",
        "unknown3": 56,
        "unknown4": "000000000000000000000000000000000000000000000000",
        "unknown5": 0,
        "path_string": "\\VOLUME{01cfdb43985a7134-dc989894}",
        "directory_strings": [
          "\\VOLUME{01cfdb43985a7134-dc989894}\\$EXTEND",
          ...
        ],
        "reference_table": {
          "version": 3,
          "reference_count": 193,
          "references": [
            {
              "reference": "0",
              "entry": 0,
              "sequence": 0
            },
            ...
          ]
        }
      }
    ]
  },
  ...
]
```

# Decompression Tool
The DecompressPrefetch tool under the examples can be used specifically to decompress MAM prefetch files.

```
DecompressPrefetch.exe -p COMPRESSED.EXE-9524B8E5.pf > DECOMPRESSED_PREFETCH.pf
```

## Change Log
#### RustyUsn 0.1.1 (2017-04-08)
- Updated some unkown fields based on @JamesHabbens [research](http://blog.4n6ir.com/2017/03/windows-prefetch-tech-details-of-new.html).
- Fixed trace chain flag (No trace chains by default).
