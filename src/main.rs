use std::env;
use std::process;
use stronk::levels::Levels;
use stronk::logging::{self, LogLevel};

fn print_usage() {
    eprintln!("usage: stronk <current_level> <target_level>");
}

fn parse_level(level: &String) -> i32 {
    match level.parse() {
        Ok(x) => x,
        Err(_) => {
            logging::log(LogLevel::Error, format!("{} is not a valid number", level));
            process::exit(1);
        }
    }
}

fn parse_args(args: &Vec<String>) -> Levels {
    if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let current_level = parse_level(&args[1]);
    let target_level = parse_level(&args[2]);

    match Levels::new(current_level, target_level) {
        Some(x) => x,
        None => {
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let _levels = parse_args(&args);
}
