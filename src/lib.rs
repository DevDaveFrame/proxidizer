use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// ImgProxy class
#[pyclass]
struct ImgProxy {
    image_url: String,
    proxy_host: String,
    key: String,
    salt: String,
    resizing_type: String,
    width: usize,
    height: usize,
    gravity: String,
    enlarge: bool,
    extension: String,
}

/// A Python module implemented in Rust.
#[pymodule]
fn proxidizer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

// I have an idea for a python library.
// I'd like to be able to create an object that generates a string,
// to which I can chain method calls to modify that string by adding different
// sections/substrings. How can build a class that allows that chaining?
// Answer: By using the __getattr__ method.
// https://stackoverflow.com/questions/136097/what-is-the-difference-between-staticmethod-and-classmethod-in-python
