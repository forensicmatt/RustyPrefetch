use serde_json;
use rustyprefetch::librp::prefetch::{PrefetchHandle};
use cpython::{  Python, PyObject,
                PyResult, ObjectProtocol,
                PyTuple, PyBytes, PyString};

py_module_initializer!(pyrpf, initpyrpf, PyInit_pyrpf, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "as_json", py_fn!(py, as_json(file_handle: PyObject))));
    Ok(())
});

fn as_json(py: Python, file_handle: PyObject) -> PyResult<PyString> {
    // Seek to EOF
    file_handle.call_method(
        py,
        "seek",
        (0,2),
        None
    )?;

    // Get offset of EOF
    let size = file_handle.call_method(
        py,
        "tell",
        PyTuple::new(py,&[]),
        None
    )?;

    // Seek to start of file
    file_handle.call_method(
        py,
        "seek",
        (0,0),
        None
    )?;

    // Read our file entire file
    let byte_buffer = file_handle.call_method(
        py,
        "read",
        (size,),
        None
    )?;

    // We need to cast our 'str' to bytes
    let py_bytes = byte_buffer.cast_into::<PyBytes>(py).unwrap();

    let pf_handle = PrefetchHandle::new(
        "a_file_name",
        Some(&py_bytes.data(py).to_vec())
    ).unwrap();
    let pf_file = pf_handle.get_prefetch().unwrap();

    let json_string = serde_json::to_string(&pf_file).unwrap();
    Ok(PyString::new(py,&json_string))
    // Ok(py.None())
}
