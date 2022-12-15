pub(crate) fn calculate_pi(iterations:usize) -> f64 {
    let mut pi = 0.0;
    let mut n = 0;

    while n < iterations {
        pi += (4.0 / (n * 2 + 1) as f64) * (if n % 2 == 0 { 1.0 } else { -1.0 });
        n += 1;
    }

    pi
}
