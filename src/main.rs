use pyo3::prelude::*;
use pyo3::py_run;
use std::process::Command;

/// Initialize Python interpreter and fix sys.path, to match possibly activated virtual env
fn init_python() {
    // Print sys.path
    let output = Command::new("python3")
        .args(["-c", "import sys; print('\\n'.join(sys.path))"])
        .output()
        .expect("Failed to execute `python3`");

    let venv_paths: Vec<&str> = str::from_utf8(output.stdout.as_slice())
        .expect("not a utf-8")
        .lines()
        .collect();

    println!("Starting python interpreter");
    Python::initialize();

    println!("Patching sys.path to reflect virtual environment");
    Python::attach(|py| -> PyResult<()> {
        let sys = PyModule::import(py, "sys")?;
        py_run!(py, sys venv_paths, "sys.path = venv_paths");
        Ok(())
    })
    .expect("Failed to patch python interpreter with venv paths");
}

fn main() -> PyResult<()> {
    init_python();

    Python::attach(|py| -> PyResult<()> {
        println!("Importing python backend");
        let backend = PyModule::import(py, "pyo3_playground.backend")?;

        println!("Invoking service function");
        let total: String = backend.getattr("service")?.call1((19,))?.extract()?;

        println!("Success: result={total}");
        Ok(())
    })?;
    Ok(())
}
