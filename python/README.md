# Python Bindings
These are the python bindings for RustyPrefetch lib. Build the bindings is not yet automated but the binary libs will be included with the RustyPrefetch releases.

# Usage
See RustyPrefetch\python\tests or examples. Currently the only function is pyrpf.as_json(FILENAME,FILEHANDLE).

``` python
with open(filename,"rb") as fh:
    try:
        json_doc = pyrpf.as_json(
            filename,
            fh
        )
    except Exception as error:
        logging.error("Error Parsing {}: {}".format(filename,unicode(error)))
        continue
```

# Building
```cargo build --release``` in the RustyPrefetch/python directory. Then change the compiled .dll to .pyd on Windows or .so on linux.

RustyPrefetch\python\target\release\pyrpf.dll -> RustyPrefetch\python\target\release\pyrpf.pyd

The libfwnt.dll must be present as well.
