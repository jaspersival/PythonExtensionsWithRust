#![allow(unused)]

mod pi;
mod fibonacci;

fn main() {
    use pyo3::prelude::*;
    use std::collections::HashMap;
    use pi::calculate_pi;
    use fibonacci::fibonacci;


    #[pyfunction]
    fn rust_calculate_pi(iterations: usize) -> PyResult<f64>{
        return Ok(calculate_pi(iterations));
    }

    #[pyfunction]
    fn rust_fibonacci(n: u64) -> PyResult<u64>{
        let mut memo = HashMap::new();
        return Ok(fibonacci(n, &mut memo));
    }
// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         return n;
//     }
//     return fibonacci(n - 1) + fibonacci(n - 2);
// }

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
    #[pymodule]
    fn rust_functions(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(rust_fibonacci, m)?)?;
        m.add_function(wrap_pyfunction!(rust_calculate_pi, m)?)?;

        Ok(())
    }
}
