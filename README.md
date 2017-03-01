# RustyPrefetch
Just another Prefetch parser...

# Build
To build you must set LIBFWNT_BIN to the path that contains the compiled libscca library. You must also copy the compiled library to the same folder as the compiled rust tools. At some point I will try and copy the compiled library from LIBFWNT_BIN to the compiled paths from the build... but for now it is a manual job

To build RustyPrefetch
```cargo build --release```

To build DecompressPrefetch and examples
```cargo test```

# Example output
```RustyPrefetch.exe -p FIREFOX.EXE-6F7B2AEE.pf```

```json
{
  "header": {
    "version": 26,
    "signature": 1094927187,
    "unknown1": 17,
    "filesize": 388998,
    "filename": "FIREFOX.EXE",
    "hash": 1870342894,
    "unknon2": 0
  },
  "fileinfo": {
    "metrics_array_offset": 304,
    "metrics_entry_count": 298,
    "trace_array_offset": 9840,
    "trace_entry_count": 26011,
    "filename_offset": 321972,
    "filename_length": 48894,
    "volume_info_offset": 370872,
    "volume_info_count": 1,
    "volume_info_size": 18126,
    "unknown3": 4294967397,
    "last_run_time": [
      "2013-10-23 02:58:39.078242",
      "2013-10-22 16:27:49.372703",
      "2013-10-21 20:21:52.602801",
      "2013-10-21 19:41:27.790050",
      "2013-10-21 18:20:14.842147",
      "2013-10-20 22:15:12.954735",
      "2013-10-19 19:54:33.945033",
      "2013-10-19 01:52:50.973575"
    ],
    "unknown1": "008C864700000000008C864700000000",
    "run_count": 28,
    "unknown2": 5,
    "unknown5": 3,
    "unknown4": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
  },
  "metrics": [
    {
      "tracechain_index": 0,
      "tracechain_count": 289,
      "unknown4": 284,
      "filename_offset": 0,
      "filename_length": 72,
      "unknown3": 514,
      "file_reference": 1125899906846240,
      "filename": "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)\\MOZILLA FIREFOX\\MSVCR100.DLL",
      "tracechain": [
        {
          "next_index": 1,
          "block_load_count": 0,
          "unknown1": 6,
          "unknown2": 1,
          "unknown3": 65535
        },
        {
          "next_index": 2,
          "block_load_count": 2,
          "unknown1": 2,
          "unknown2": 1,
          "unknown3": 65535
        },
...
    },
...
  ],
  "volumes": [
    {
      "path_offset": 104,
      "path_length": 23,
      "vol_creation_time": "2013-06-02 03:43:28.889595",
      "volume_serial": 2119740080,
      "references_offset": 152,
      "references_data_size": 3016,
      "directory_offset": 3168,
      "directory_string_count": 101,
      "unknown1": 274,
      "unknown2": "00000000000000000000000000000000000000000000000000000000",
      "unknown3": 101,
      "unknown4": "00000000000000000000000000000000000000000000000000000000",
      "unknown5": 0,
      "path_string": "\\DEVICE\\HARDDISKVOLUME5",
      "directory_strings": [
        "\\DEVICE\\HARDDISKVOLUME5\\$EXTEND",
        "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES",
        "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)",
        "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)\\ADOBE",
        "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)\\ADOBE\\READER 11.0",
        "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)\\ADOBE\\READER 11.0\\READER",
        "\\DEVICE\\HARDDISKVOLUME5\\PROGRAM FILES (X86)\\BONJOUR",
...
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCALLOW",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCALLOW\\MICROSOFT",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCALLOW\\MICROSOFT\\CRYPTNETURLCACHE",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCALLOW\\MICROSOFT\\CRYPTNETURLCACHE\\CONTENT",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCALLOW\\MICROSOFT\\CRYPTNETURLCACHE\\METADATA",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\APPLE",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MICROSOFT",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MICROSOFT\\WINDOWS",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MICROSOFT\\WINDOWS\\CACHES",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MICROSOFT\\WINDOWS\\INETCACHE",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MOZILLA",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MOZILLA\\FIREFOX",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MOZILLA\\FIREFOX\\PROFILES",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MOZILLA\\FIREFOX\\PROFILES\\29BMRORB.DEFAULT",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MOZILLA\\FIREFOX\\PROFILES\\29BMRORB.DEFAULT\\CACHE",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MOZILLA\\FIREFOX\\PROFILES\\29BMRORB.DEFAULT\\CACHE\\2",
        "\\DEVICE\\HARDDISKVOLUME5\\USERS\\DONALD\\APPDATA\\LOCAL\\MOZILLA\\FIREFOX\\PROFILES\\29BMRORB.DEFAULT\\CACHE\\2\\6B",
...
      ],
      "reference_table": {
        "version": 3,
        "reference_count": 375,
        "references": [
          0,
          3588,
          9044,
          18068,
          42768,
          42772,
...
          43032,
          43040,
          114804,
          1125899906846240,
          1407374883556895,
          1407374883559521,
...
        ]
      }
    }
  ]
}
```

# Decompression Tool
The DecompressPrefetch tool under the examples can be used specifically to decompress MAM prefetch files.

```
DecompressPrefetch.exe -p COMPRESSED.EXE-9524B8E5.pf > DECOMPRESSED_PREFETCH.pf
```
