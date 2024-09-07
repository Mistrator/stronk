use std::env;
use std::io;
use std::process;
use stronk::levels::Levels;
use stronk::logging::{self, LogLevel};
use stronk::statistic::{StatType, Statistic};

fn print_usage() {
    eprintln!("usage: stronk <current_level> <target_level>");
}

fn parse_level(level: &String) -> i32 {
    match level.parse() {
        Ok(x) => x,
        Err(_) => {
            logging::log(
                LogLevel::Error,
                format!("level is not a valid integer: {}", level),
            );
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

fn parse_stat_kind(kind: &str) -> Option<StatType> {
    match kind {
        "ac" => Some(StatType::ArmorClass),
        _ => {
            logging::log(LogLevel::Error, format!("unknown statistic: {}", kind));
            None
        }
    }
}

// Ensure the input is an integer. However, we still want to store it as f64.
fn parse_stat_value_integer(value: &str) -> Option<f64> {
    let parsed: Result<i32, _> = value.parse();
    match parsed {
        Ok(p) => Some(p.into()),
        Err(_) => {
            logging::log(
                LogLevel::Error,
                format!("statistic value is not a valid integer: {}", value),
            );
            None
        }
    }
}

fn parse_stat_value(kind: StatType, value: &str) -> Option<f64> {
    match kind {
        StatType::ArmorClass => parse_stat_value_integer(value),
    }
}

fn parse_prompt(prompt: &str) -> Option<Statistic> {
    // We must assign String created by to_lowercase() to its own variable,
    // or it becomes a temporary that is then dropped too early.
    let prompt: String = prompt.trim().to_lowercase();
    let prompt: Vec<&str> = prompt.split_whitespace().collect();

    if prompt.len() != 2 {
        logging::log(LogLevel::Error, "invalid prompt");
        logging::log(LogLevel::Info, "usage: <statistic> <current_value>");
        return None;
    }

    let kind = parse_stat_kind(prompt[0]);
    if let Some(k) = kind {
        let value = parse_stat_value(k, prompt[1]);
        if let Some(v) = value {
            return Some(Statistic::new(k, v));
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let _levels = parse_args(&args);

    loop {
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("failed to read prompt");

        let _stat = parse_prompt(&prompt);
    }
}
