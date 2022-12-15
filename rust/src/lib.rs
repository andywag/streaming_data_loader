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
pub mod py_conversions;

use py_interface::TrainingRun;
use pyo3::prelude::*;




/// A Python module implemented in Rust.
#[pymodule]
fn rust_loader(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TrainingRun>()?;
    Ok(())
}