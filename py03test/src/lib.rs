use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    return Err(PyErr::new::<PyTypeError, _>("What the heck?"));
    // Ok(format!("Howdy! {}", (a + b).to_string()))
}

/// A Python module implemented in Rust.
#[pymodule]
fn py03test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}