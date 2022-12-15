mod cli;
use cli::parse_args;

fn main() {
    let args = parse_args();
    let mut n: u64 = args.number;
    let original_n = n;
    let mut factorial: u64 = 1;
    while n > 0 {
        factorial *= n;
        n -= 1;
    }
    println!("The factorial of {} is {}", original_n, factorial)
}
