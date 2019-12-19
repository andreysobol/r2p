#[macro_use] extern crate cpython;
use cpython::{Python, PyResult};

fn hello(py: Python, val:u64) -> PyResult<u64> {
    println!("This is Rust");
    Ok(0u64)
}

py_module_initializer!(librlib, initlibrlib, PyInit_librlib, |py, m | {
    try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    try!(m.add(py, "hello", py_fn!(py, hello(val:u64))));
    Ok(())
});