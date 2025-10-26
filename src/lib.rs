use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn repository(value: usize) -> PyResult<String> {
    println!("RUST: entering repository: value={value}, converting to str and reversing it");
    let result = value.to_string().chars().rev().collect();
    println!("RUST: exiting repository result='{result}'");
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_playground(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(repository, m)?)?;
    Ok(())
}
