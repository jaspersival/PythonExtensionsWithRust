
#![allow(unused)]
fn main() {
use pyo3::prelude::*;
use std::collections::HashMap;

fn fibonacci(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => n,
        _ => {
            if let Some(&fib_n) = memo.get(&n) {
                fib_n
            } else {
                let fib_n = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
                memo.insert(n, fib_n);
                fib_n
            }
        }
    }
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
fn alliander_rust_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_fibonacci, m)?)?;

    Ok(())
}
}
