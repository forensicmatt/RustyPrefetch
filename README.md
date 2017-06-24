A fast and cross platform Prefetch parser written in Rust that gives you the ability to query the records via [JMES Query](http://jmespath.org/). Output is [JSONL](http://jsonlines.org/).

# Build
To build you must set LIBFWNT_BIN to the path that contains the compiled libscca library. You must also copy the compiled library to the same folder as the compiled rust tools. At some point I will try and copy the compiled library from LIBFWNT_BIN to the compiled paths from the build... but for now it is a manual job

To build RustyPrefetch
```cargo build --release```

## RustyPrefetch
```
RustyPrefetch 0.2.0
Matthew Seyer <https://github.com/forensicmatt/RustyPrefetch>
Parse prefetch.

USAGE:
    RustyPrefetch.exe [FLAGS] [OPTIONS] --source <FILE>

FLAGS:
    -b, --bool_expr     JMES Query as bool only. (Prints whole record if true.)
    -h, --help          Prints help information
    -t, --tracechain    Output Tracechains
    -V, --version       Prints version information

OPTIONS:
    -q, --query <QUERY>    JMES Query
    -s, --source <FILE>    The source path. Can be a file or a directory.

```

## Output
The output is written to stdout as a json list of records.

# DecompressPrefetch
The DecompressPrefetch tool under the examples can be used specifically to decompress MAM prefetch files.

```
DecompressPrefetch.exe -p COMPRESSED.EXE-9524B8E5.pf > DECOMPRESSED_PREFETCH.pf
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

## Change Log
#### RustyPrefetch 0.2.0 (2017-06-23)
- Added JMES Query functionality (http://jmespath.org/)
- Added JSONL output (http://jsonlines.org/)

#### RustyPrefetch 0.1.1 (2017-04-08)
- Updated some unkown fields based on @JamesHabbens [research](http://blog.4n6ir.com/2017/03/windows-prefetch-tech-details-of-new.html).
- Fixed trace chain flag (No trace chains by default).
