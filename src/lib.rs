use pyo3::prelude::*;

#[pyfunction]
fn hello() {
    println!("Hello, world!");
}

#[pymodule]
fn layla_math(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}