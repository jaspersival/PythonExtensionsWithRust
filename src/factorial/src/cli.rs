use std::env;
use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - Compute the factorial of a positive number",
        "factorial".green()
    );
    eprintln!("Usage: factorial <number>")
}

#[derive(Debug)]
pub(crate) struct Arguments {
    pub(crate) number: u64,
}

pub(crate) fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 1, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    let number = args[0].clone().parse::<u64>().unwrap();
    Arguments { number }
}
