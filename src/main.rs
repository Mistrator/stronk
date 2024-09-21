use std::env;
use std::io;
use std::process;
use stronk::damage::{self, Damage};
use stronk::levels::Levels;
use stronk::logging::{self, LogLevel};
use stronk::scaling::{self, ScaleResult};
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
        "attack" | "att" => Some(StatType::StrikeAttackBonus),
        "damage" | "dmg" => Some(StatType::StrikeDamage),
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

fn handle_prompt(levels: Levels, prompt: &str) -> Option<ScaleResult> {
    // We must assign the String created by to_lowercase() to its own variable,
    // or it becomes a temporary that is then dropped too early.
    let prompt: String = prompt.trim().to_lowercase();
    let prompt = match prompt.split_once(' ') {
        Some(p) => p,
        None => {
            logging::log(LogLevel::Error, "invalid prompt");
            logging::log(LogLevel::Info, "usage: <statistic> <current_value>");
            return None;
        }
    };
    let prompt_kind = prompt.0.trim();
    let prompt_value = prompt.1.trim();

    let stat_kind = match parse_stat_kind(prompt_kind) {
        Some(k) => k,
        None => {
            logging::log(
                LogLevel::Error,
                format!("invalid prompt: unknown statistic: {}", prompt_kind),
            );
            return None;
        }
    };

    match stat_kind {
        StatType::StrikeDamage => {
            let damage = match damage::parse_damage(prompt_value) {
                Some(d) => d,
                None => {
                    return None;
                }
            };

            let total_damage = Statistic::new(stat_kind, damage.total_average_value());
            let scale_result = scaling::scale_statistic(levels, total_damage);
            let scaled_damage = scaling::scale_damage_components(&damage, scale_result.stat.value);

            print_damage(&scaled_damage, scale_result);

            Some(scale_result)
        }
        StatType::ArmorClass | StatType::StrikeAttackBonus => {
            let stat_value = match parse_stat_value_integer(stat_kind, prompt_value) {
                Some(s) => s,
                None => {
                    return None;
                }
            };

            let scaled = scaling::scale_statistic(levels, Statistic::new(stat_kind, stat_value));

            print_result(scaled);

            Some(scaled)
        }
    }
}

fn print_result(result: ScaleResult) {
    println!(
        "{} {} [{}, {}]",
        result.stat.kind, result.stat.value, result.proficiency, result.method
    );
}

fn print_damage(damage: &Damage, result: ScaleResult) {
    print!("{} ", result.stat.kind);

    for (i, component) in damage.components.iter().enumerate() {
        #[rustfmt::skip]
        let damage_expression = damage::build_damage_expression(component.average_value, result.proficiency);

        print!(
            "{} ({:.1}) {} ",
            damage_expression, component.average_value, component.damage_type
        );

        let n = damage.components.len();
        if n > 1 && i != n - 1 {
            print!("plus ");
        }
    }

    println!("[{}, {}]", result.proficiency, result.method);
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

        handle_prompt(levels, &prompt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stronk::scaling::ScaleMethod;
    use stronk::tables::Proficiency;
    use stronk::utils::float_eq;

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

    #[rustfmt::skip]
    #[test]
    fn accept_valid_prompt_syntax() {
        let levels = Levels::new(1, 2).unwrap();

        assert!(handle_prompt(levels, "ac 12").is_some());
        assert!(handle_prompt(levels, "AC 12").is_some());
        assert!(handle_prompt(levels, "   ac   12    ").is_some());
        assert!(handle_prompt(levels, "AC 120").is_some());
        assert!(handle_prompt(levels, "ac 0").is_some());
        assert!(handle_prompt(levels, "ac -1").is_some());
        assert!(handle_prompt(levels, "ac -34").is_some());

        assert!(handle_prompt(levels, "damage 2d12+11 bludgeoning").is_some());
        assert!(handle_prompt(levels, "damage 3d10 + 17 slashing plus 2d6+6 cold plus 1d4 acid plus 2 vitality").is_some());
    }

    #[test]
    fn reject_invalid_prompt_syntax() {
        let levels = Levels::new(1, 2).unwrap();

        assert!(handle_prompt(levels, "").is_none());
        assert!(handle_prompt(levels, "ac").is_none());
        assert!(handle_prompt(levels, "invalid").is_none());
        assert!(handle_prompt(levels, "ac x").is_none());
        assert!(handle_prompt(levels, "invalid 12").is_none());
        assert!(handle_prompt(levels, "invalid x").is_none());
        assert!(handle_prompt(levels, "ac 12 34").is_none());
        assert!(handle_prompt(levels, "ac 12.34").is_none());

        assert!(handle_prompt(levels, "ac 2d6+1 fire").is_none());
        assert!(handle_prompt(levels, "damage 1d4+1").is_none());
        assert!(handle_prompt(levels, "1d6+2").is_none());
    }

    #[test]
    fn scale_armor_class() {
        let levels = Levels::new(3, 14).unwrap();

        let result = handle_prompt(levels, "ac 18").unwrap();
        assert_eq!(result.stat.kind, StatType::ArmorClass);
        assert!(float_eq(result.stat.value, 35.0));
        assert_eq!(result.proficiency, Proficiency::Moderate);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_strike_attack_bonus() {
        let levels = Levels::new(11, 19).unwrap();

        let result = handle_prompt(levels, "attack +24").unwrap();
        assert_eq!(result.stat.kind, StatType::StrikeAttackBonus);
        assert!(float_eq(result.stat.value, 36.0));
        assert_eq!(result.proficiency, Proficiency::High);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_strike_damage() {
        let levels = Levels::new(7, 17).unwrap();

        let result = handle_prompt(levels, "damage 2d12+12 piercing").unwrap();
        assert_eq!(result.stat.kind, StatType::StrikeDamage);
        assert!(float_eq(result.stat.value, 50.5));
        assert_eq!(result.proficiency, Proficiency::Extreme);
        assert_eq!(result.method, ScaleMethod::Exact);
    }
}
