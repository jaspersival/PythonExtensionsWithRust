use std::collections::HashMap;

pub(crate) fn fibonacci(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
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
