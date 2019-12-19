#[macro_use] extern crate cpython;
use cpython::{Python, PyResult};

fn hello(py: Python) -> PyResult<()> {
    println!("This is Rust");
    Ok(())
}