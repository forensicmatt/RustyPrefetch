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
    "source_file": "ACRORD32.EXE-0D099F9D.pf",
    "header": {
      "version": 26,
      "signature": 1094927187,
      "unknown1": 17,
      "filesize": 152252,
      "filename": "ACRORD32.EXE",
      "hash": 218734493,
      "unknon2": 0
    },
    "fileinfo": {
      "metrics_array_offset": 304,
      "metrics_entry_count": 135,
      "trace_array_offset": 4624,
      "trace_entry_count": 10458,
      "filename_offset": 130120,
      "filename_length": 18220,
      "volume_info_offset": 148344,
      "volume_info_count": 1,
      "volume_info_size": 3908,
      "unknown3": 4294967323,
      "last_run_time": [
        "2013-10-21 20:00:20.379",
        ...
      ],
      "unknown1": "00000000000000000000000000000000",
      "run_count": 6,
      "unknown2": 6,
      "unknown5": 3,
      "unknown4": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    },
    "metrics": [
      {
        "tracechain_index": 0,
        "tracechain_count": 297,
        "unknown4": 33,
        "filename_offset": 0,
        "filename_length": 81,
        "unknown3": 512,
        "file_reference": {
          "reference": "2533274790580458",
          "entry": 184554,
          "sequence": 9
        },
        "filename": "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)\\ADOBE\\READER 11.0\\READER\\COOLTYPE.DLL",
        "tracechain": [
          {
            "next_index": 1,
            "block_load_count": 0,
            "unknown1": 3,
            "unknown2": 1,
            "unknown3": 22
          },
          ...
        ]
      }
      ...
    ],
    "volumes": [
      {
        "path_offset": 104,
        "path_length": 23,
        "vol_creation_time": "2013-06-02 03:43:28.889",
        "volume_serial": 2119740080,
        "references_offset": 152,
        "references_data_size": 656,
        "directory_offset": 808,
        "directory_string_count": 27,
        "unknown1": 53,
        "unknown2": "00000000000000000000000000000000000000000000000000000000",
        "unknown3": 27,
        "unknown4": "00000000000000000000000000000000000000000000000000000000",
        "unknown5": 0,
        "path_string": "\\DEVICE\\HARDDISKVOLUME5",
        "directory_strings": [
          "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)",
           ...
        ],
        "reference_table": {
          "version": 3,
          "reference_count": 80,
          "references": [
            {
              "reference": "3096224743995331",
              "entry": 178115,
              "sequence": 11
            },
            ...
          ]
        }
      }
    ]
  }
]
```

# Decompression Tool
The DecompressPrefetch tool under the examples can be used specifically to decompress MAM prefetch files.

```
DecompressPrefetch.exe -p COMPRESSED.EXE-9524B8E5.pf > DECOMPRESSED_PREFETCH.pf
```
