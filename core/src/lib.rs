use pyo3::prelude::*;

#[pyfunction]
fn secure_wipe(path: String) -> PyResult<()> {
    forensic::secure_wipe(Path::new(&path))?;
    Ok(())
}

#[pymodule]
fn blackmint(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(secure_wipe, m)?;
    Ok(())
}
