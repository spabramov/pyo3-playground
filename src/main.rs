use pyo3::prelude::*;
use pyo3::py_run;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

macro_rules! call1 {
    ($obj:ident, $method:literal, $args:expr) => {
        $obj.getattr($method)
            .and_then(|m| m.call1($args))
            .and_then(|r| r.extract())
    };
}

macro_rules! import_backend {
    ($py:ident) => {
        PyModule::import($py, "pyo3_playground.backend")
    };
}

#[pyclass]
pub struct RsRepo {}

#[pymethods]
impl RsRepo {
    pub fn rs_sleep(&self, py: Python<'_>, seconds: u64) -> u64 {
        py.detach(move || {
            println!("RUST: Sleeping for {seconds}");
            thread::sleep(Duration::from_secs(seconds));
            seconds
        })
    }
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

fn create_repo() -> Py<RsRepo> {
    Python::attach(|py| Py::new(py, RsRepo {})).expect("failed to create RsRepo")
}

fn clone(repo: &Py<RsRepo>) -> Py<RsRepo> {
    Python::attach(|py| repo.clone_ref(py))
}

fn main() -> PyResult<()> {
    init_python();

    let mut handles = vec![];

    let start = Instant::now();
    let repo = create_repo();
    for _ in 0..5 {
        let repo = clone(&repo);
        let handle = thread::spawn(move || {
            Python::attach(|py| -> PyResult<()> {
                println!("Binding repository");
                let repo = repo.bind(py);

                let backend = import_backend!(py)?;
                let service = backend.getattr("Service")?.call1((repo,))?;

                println!("Invoking repo_sleep");
                let _: usize = call1!(service, "repo_sleep", (2,))?;

                Ok(())
            })
            .expect("Python attach failed")
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total duration: {:.2?}", start.elapsed());

    Python::attach(|py| -> PyResult<()> {
        let backend = import_backend!(py)?;
        let calls: Vec<usize> = backend.getattr("CALLS")?.extract()?;
        let cnt = calls.len();
        println!("Total calls: {cnt}");
        Ok(())
    })
    .expect("Reading CALLS failed");

    Ok(())
}
