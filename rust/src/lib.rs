pub mod provider;
pub mod transport;
pub mod tasks;
pub mod datasets;
pub mod models;

pub mod batcher;
pub mod tokenizer;
pub mod logger;
pub mod config;
pub mod py_interface;

use pyo3::prelude::*;




/// A Python module implemented in Rust.
#[pymodule]
fn maturin_build(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_interface::test, m)?)?;
    Ok(())
}