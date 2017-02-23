# RustyPrefetch
Just another Prefetch parser...

# Build
To build you must set LIBFWNT_BIN to the path that contains the compiled libscca library. You must also copy the compiled library to the same folder as the compiled rust tools.

# Example output
```RustyPrefetch.exe -p ..\testfiles\Prefetch\ATOM.EXE-9524B8E5.pf```

```json
{
  "header": {
    "version": 30,
    "signature": 1094927187,
    "unknown1": 17,
    "filesize": 139768,
    "filename": "ATOM.EXE",
    "hash": 2502211813,
    "unknon2": 0
  },
  "fileinfo": {
    "metrics_array_offset": 304,
    "metrics_entry_count": 182,
    "trace_array_offset": 6128,
    "trace_entry_count": 12444,
    "filename_offset": 105680,
    "filename_length": 27900,
    "volume_info_offset": 133584,
    "volume_info_count": 1,
    "volume_info_size": 6184,
    "unknown3": 8589934625,
    "last_run_time": [
      "2017-02-11 22:56:33.147428",
      "2017-02-11 22:56:31.666104",
      "2017-02-11 22:56:31.339137",
      "2017-02-11 22:56:32.326141",
      "2017-02-11 05:29:40.208776",
      "2017-02-11 05:04:33.768479",
      "2017-02-11 05:02:09.989924",
      "2017-02-11 04:53:25.309454"
    ],
    "unknown1": "008C864700000000008C864700000000",
    "run_count": 42,
    "unknown2": 2,
    "unknown5": 3,
    "unknown4": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
  }
}
```

```RustyPrefetch.exe -p ..\testfiles\Prefetch\ACRORD32.EXE-0D099F9D.pf```
```json
{
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
      "2013-10-21 20:00:20.379583",
      "2013-10-21 20:00:20.551457",
      "2013-10-03 12:09:02.114840",
      "2013-10-03 12:09:01.725921",
      "2013-10-01 02:11:11.301269",
      "2013-10-01 02:11:11.157499",
      "1601-01-01 00:00:00",
      "1601-01-01 00:00:00"
    ],
    "unknown1": "00000000000000000000000000000000",
    "run_count": 6,
    "unknown2": 6,
    "unknown5": 3,
    "unknown4": "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
  }
}
```

```RustyPrefetch.exe -p ..\testfiles\Prefetch\FIREFOX.EXE-28641590.pf```
```json
{
  "header": {
    "version": 17,
    "signature": 1094927187,
    "unknown1": 15,
    "filesize": 62998,
    "filename": "FIREFOX.EXE",
    "hash": 677647760,
    "unknon2": 0
  },
  "fileinfo": {
    "metrics_array_offset": 152,
    "metrics_entry_count": 44,
    "trace_array_offset": 1032,
    "trace_entry_count": 4589,
    "filename_offset": 56100,
    "filename_length": 5742,
    "volume_info_offset": 61848,
    "volume_info_count": 1,
    "volume_info_size": 1150,
    "last_run_time": "2012-04-03 01:03:30.339440",
    "unknown1": "008C864700000000008C864700000000",
    "run_count": 17,
    "unknown2": 5
  }
}
```

# Decoder Tool
The DecompressPrefetch tool under the examples can be used specifically to decompress MAM prefetch files.

```
DecompressPrefetch.exe -p COMPRESSED.EXE-9524B8E5.pf > DECOMPRESSED_PREFETCH.pf
```
