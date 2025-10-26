use pyo3::prelude::*;
use std::{thread, time::Duration};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn repository(value: usize) -> PyResult<String> {
    println!("RUST: entering repository: value={value}, converting to str and reversing it");
    let result = value.to_string().chars().rev().collect();
    println!("RUST: exiting repository result='{result}'");
    Ok(result)
}

#[pyfunction]
fn rs_sleep(py: Python<'_>, seconds: u64) -> u64 {
    py.detach(move || {
        println!("RUST: Sleeping for {seconds}");
        thread::sleep(Duration::from_secs(seconds));
        seconds
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_playground(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(repository, m)?)?;
    m.add_function(wrap_pyfunction!(rs_sleep, m)?)?;
    Ok(())
}
