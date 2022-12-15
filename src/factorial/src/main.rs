mod cli;
use cli::parse_args;

fn calculate_factorial(mut n: u64) -> u64 {
    let mut factorial: u64 = 1;
    while n > 0 {
        factorial *= n;
        n -= 1;
    }
    factorial
}
fn main() {
    let args = parse_args();
    let n: u64 = args.number;
    let original_n = n;
    let factorial = calculate_factorial(n);
    println!("The factorial of {} is {}", original_n, factorial)
}

#[test]
fn calculate_factorial_with_5_returns_120() {
    let number: u64 = 5;
    let expected: u64 = 120;

    let result = calculate_factorial(number);

    assert_eq!(result, expected)
}

#[test]
fn calculate_factorial_with_0_returns_1() {
    let number: u64 = 0;
    let expected: u64 = 1;

    let result = calculate_factorial(number);

    assert_eq!(result, expected)
}

#[test]
fn calculate_factorial_with_1_returns_1() {
    let number: u64 = 1;
    let expected: u64 = 1;

    let result = calculate_factorial(number);

    assert_eq!(result, expected)
}
