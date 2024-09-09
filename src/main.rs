use std::env;
use std::io;
use std::process;
use stronk::levels::Levels;
use stronk::logging::{self, LogLevel};
use stronk::scaling::ScaleResult;
use stronk::statistic::{StatType, Statistic};

fn print_usage() {
    eprintln!("usage: stronk <current_level> <target_level>");
}

fn parse_level(level: &str) -> Option<i32> {
    match level.parse() {
        Ok(x) => Some(x),
        Err(_) => {
            logging::log(
                LogLevel::Error,
                format!("level is not a valid integer: {}", level),
            );
            None
        }
    }
}

fn parse_args(args: &Vec<&str>) -> Option<Levels> {
    if args.len() != 3 {
        print_usage();
        return None;
    }

    let current_level = parse_level(args[1]);
    if let Some(c) = current_level {
        let target_level = parse_level(args[2]);
        if let Some(t) = target_level {
            return Levels::new(c, t);
        }
    }

    None
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
fn parse_stat_value_integer(kind: StatType, value: &str) -> Option<f64> {
    let parsed: Result<i32, _> = value.parse();
    match parsed {
        Ok(p) => Some(p.into()),
        Err(_) => {
            logging::log(
                LogLevel::Error,
                format!("{} value is not a valid integer: {}", kind, value),
            );
            None
        }
    }
}

fn parse_stat_value(kind: StatType, value: &str) -> Option<f64> {
    match kind {
        StatType::ArmorClass => parse_stat_value_integer(kind, value),
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

fn print_result(result: ScaleResult) {
    println!(
        "{} {} ({}) ({})",
        result.stat.kind, result.stat.value, result.proficiency, result.method
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // parse_args() is easier to test if we can pass string literals
    // instead of having to use String::from() everywhere.
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let levels = match parse_args(&args) {
        Some(x) => x,
        None => {
            process::exit(1);
        }
    };

    loop {
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("failed to read prompt");

        let stat = parse_prompt(&prompt);
        if stat.is_none() {
            continue;
        }

        let scale_result = stronk::scale_statistic(levels, stat.unwrap());
        print_result(scale_result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_valid_args() {
        let levels = parse_args(&vec!["", "1", "2"]).unwrap();
        assert_eq!(levels.current, 1);
        assert_eq!(levels.target, 2);

        assert!(parse_args(&vec!["", "2", "1"]).is_some());
        assert!(parse_args(&vec!["", "1", "1"]).is_some());
        assert!(parse_args(&vec!["", "-1", "24"]).is_some());
        assert!(parse_args(&vec!["", "24", "-1"]).is_some());
        assert!(parse_args(&vec!["something", "1", "2"]).is_some());
    }

    #[test]
    fn reject_invalid_args() {
        assert!(parse_args(&vec![]).is_none());
        assert!(parse_args(&vec!["1", "2"]).is_none());
        assert!(parse_args(&vec!["", "1", "x"]).is_none());
        assert!(parse_args(&vec!["", "x", "1"]).is_none());
        assert!(parse_args(&vec!["", "x", "x"]).is_none());
        assert!(parse_args(&vec!["", "-2", "2"]).is_none());
        assert!(parse_args(&vec!["", "1", "25"]).is_none());
        assert!(parse_args(&vec!["", "1", "2.345"]).is_none());
        assert!(parse_args(&vec!["", "1", "2", "3"]).is_none());
    }

    #[test]
    fn accept_valid_prompt() {
        let statistic = parse_prompt("ac 12").unwrap();
        assert!(statistic.kind == StatType::ArmorClass);
        assert!(statistic.value == 12.0);

        assert!(parse_prompt("AC 12").is_some());
        assert!(parse_prompt("   ac   12    ").is_some());
        assert!(parse_prompt("ac 0").is_some());
        assert!(parse_prompt("ac -1").is_some());
        assert!(parse_prompt("ac -34").is_some());
    }

    #[test]
    fn reject_invalid_prompt() {
        assert!(parse_prompt("").is_none());
        assert!(parse_prompt("ac").is_none());
        assert!(parse_prompt("invalid").is_none());
        assert!(parse_prompt("ac x").is_none());
        assert!(parse_prompt("invalid 12").is_none());
        assert!(parse_prompt("invalid x").is_none());
        assert!(parse_prompt("ac 12 34").is_none());
        assert!(parse_prompt("ac 12.34").is_none());
    }
}
