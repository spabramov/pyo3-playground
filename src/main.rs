use core::time;
use pyo3::prelude::*;
use pyo3::py_run;
use std::process::Command;
use std::thread;
use std::time::Instant;

macro_rules! call1 {
    ($obj:ident, $method:literal, $args:expr) => {
        $obj.getattr($method)
            .and_then(|m| m.call1($args))
            .and_then(|r| r.extract())
    };
}

macro_rules! import {
    ($py:ident, $module:literal) => {
        PyModule::import($py, $module)
    };
}

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

    let mut handles = vec![];

    let start = Instant::now();
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            Python::attach(|py| -> PyResult<()> {
                println!("Importing python backend");
                let backend = import!(py, "pyo3_playground.backend")?;

                println!("Invoking py_rs_sleep");
                let _: usize = call1!(backend, "py_rs_sleep", (2,))?;

                Ok(())
            })
            .expect("Python attach failed")
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total: {:.2?}", start.elapsed());

    Ok(())
}
