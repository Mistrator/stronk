use std::env;
use std::fs;
use std::io;
use std::process;
use stronk::color::{self, Color};
use stronk::damage::{self, Damage};
use stronk::levels::Levels;
use stronk::logging::{self, LogLevel};
use stronk::scaling::{self, ScaleMethod, ScaleResult};
use stronk::statistic::{self, StatType, Statistic};

struct Arguments {
    pub levels: Levels,
    pub input_file: Option<String>,
}

fn print_usage() {
    eprintln!("usage: stronk <current_level> <target_level> [input_file]");
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

fn parse_args(args: &Vec<&str>) -> Option<Arguments> {
    if !(args.len() == 3 || args.len() == 4) {
        print_usage();
        return None;
    }

    let current_level = parse_level(args[1]);
    if let Some(c) = current_level {
        let target_level = parse_level(args[2]);
        if let Some(t) = target_level {
            if let Some(levels) = Levels::new(c, t) {
                let input_file = if args.len() == 4 {
                    Some(String::from(args[3]))
                } else {
                    None
                };

                return Some(Arguments { levels, input_file });
            }
        }
    }

    None
}

fn parse_stat_kind(kind: &str) -> Option<StatType> {
    match kind {
        "perception" | "per" => Some(StatType::Perception),
        "skill" => Some(StatType::Skill),
        "ac" => Some(StatType::ArmorClass),
        "save" | "fortitude" | "fort" | "reflex" | "ref" | "will" => Some(StatType::SavingThrow),
        "hp" => Some(StatType::HitPoints),
        "resistance" => Some(StatType::Resistance),
        "weakness" => Some(StatType::Weakness),
        "strike-attack" | "att" => Some(StatType::StrikeAttackBonus),
        "strike-damage" | "dmg" => Some(StatType::StrikeDamage),
        "spell-dc" => Some(StatType::SpellDC),
        "spell-attack" => Some(StatType::SpellAttackBonus),
        "unlimited-area-damage" => Some(StatType::UnlimitedAreaDamage),
        "limited-area-damage" => Some(StatType::LimitedAreaDamage),
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
        StatType::StrikeDamage | StatType::UnlimitedAreaDamage | StatType::LimitedAreaDamage => {
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
        StatType::Perception
        | StatType::Skill
        | StatType::ArmorClass
        | StatType::SavingThrow
        | StatType::HitPoints
        | StatType::Resistance
        | StatType::Weakness
        | StatType::StrikeAttackBonus
        | StatType::SpellDC
        | StatType::SpellAttackBonus => {
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

fn print_scale_details(result: ScaleResult) {
    let colored_method = format!("{}", result.method);
    let color = match result.method {
        ScaleMethod::Exact => Color::Green,
        ScaleMethod::Interpolated => Color::Green,
        ScaleMethod::Extrapolated => Color::BrightYellow,
    };
    let colored_method = color::color_text(colored_method, color);

    println!("[{}] [{}]", result.proficiency, colored_method);
}

fn print_result(result: ScaleResult) {
    let stat_rounded = result.stat.value.floor();

    let colored_stat = if statistic::is_bonus(result.stat.kind) {
        format!("+{}", stat_rounded)
    } else {
        format!("{}", stat_rounded)
    };

    let colored_stat = color::color_text(colored_stat, Color::BrightCyan);

    print!("{} {} ", result.stat.kind, colored_stat);

    if result.method != ScaleMethod::Exact {
        print!("({:.2}) ", result.stat.value);
    }

    print_scale_details(result);
}

fn print_damage(damage: &Damage, result: ScaleResult) {
    print!("{} ", result.stat.kind);

    for (i, component) in damage.components.iter().enumerate() {
        #[rustfmt::skip]
        let damage_expression = damage::build_damage_expression(component.average_value, result.proficiency);

        let colored_damage_expression = color::color_text(damage_expression, Color::BrightCyan);
        print!(
            "{} ({:.2}) {} ",
            colored_damage_expression, component.average_value, component.damage_type
        );

        let n = damage.components.len();
        if n > 1 && i != n - 1 {
            print!("plus ");
        }
    }

    print_scale_details(result);
}

fn process_input_file(args: Arguments) -> bool {
    let input_file = args
        .input_file
        .expect("input file argument should have been checked to exist");

    let contents = match fs::read_to_string(input_file) {
        Ok(c) => c,
        Err(_) => {
            logging::log(LogLevel::Error, "failed to read input file");
            return false;
        }
    };

    for line in contents.lines() {
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            println!("{}", line);
            continue;
        }

        if handle_prompt(args.levels, line).is_none() {
            logging::log(LogLevel::Error, "failed to process input file");
            return false;
        }
    }

    true
}

fn start_interactive_prompt(args: Arguments) {
    loop {
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("failed to read prompt");

        handle_prompt(args.levels, &prompt);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // parse_args() is easier to test if we can pass string literals
    // instead of having to use String::from() everywhere.
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let parsed_args = match parse_args(&args) {
        Some(x) => x,
        None => {
            process::exit(1);
        }
    };

    if parsed_args.input_file.is_some() {
        process_input_file(parsed_args);
    } else {
        start_interactive_prompt(parsed_args);
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
        let args = parse_args(&vec!["", "1", "2"]).unwrap();
        assert_eq!(args.levels.current, 1);
        assert_eq!(args.levels.target, 2);

        assert!(parse_args(&vec!["", "2", "1"]).is_some());
        assert!(parse_args(&vec!["", "1", "1"]).is_some());
        assert!(parse_args(&vec!["", "-1", "24"]).is_some());
        assert!(parse_args(&vec!["", "24", "-1"]).is_some());
        assert!(parse_args(&vec!["something", "1", "2"]).is_some());

        assert!(parse_args(&vec!["", "1", "2", "input.txt"]).is_some());
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

        assert!(parse_args(&vec!["", "1", "2", "input.txt", "4"]).is_none());
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

        assert!(handle_prompt(levels, "strike-damage 2d12+11 bludgeoning").is_some());
        assert!(handle_prompt(levels, "strike-damage 3d10 + 17 slashing plus 2d6+6 cold plus 1d4 acid plus 2 vitality").is_some());
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
    fn accept_valid_input_file() {
        let args = Arguments {
            levels: Levels::new(9, 15).unwrap(),
            input_file: Some(String::from("testdata/valid_input_file.txt")),
        };

        assert!(process_input_file(args));
    }

    #[test]
    fn reject_invalid_input_file() {
        let args = Arguments {
            levels: Levels::new(9, 15).unwrap(),
            input_file: Some(String::from("testdata/invalid_input_file.txt")),
        };

        assert_eq!(process_input_file(args), false);
    }

    #[test]
    fn reject_nonexistent_input_file() {
        let args = Arguments {
            levels: Levels::new(9, 15).unwrap(),
            input_file: Some(String::from("testdata/nonexistent_input_file.txt")),
        };

        assert_eq!(process_input_file(args), false);
    }

    #[test]
    fn scale_perception() {
        let levels = Levels::new(19, 15).unwrap();

        let result = handle_prompt(levels, "perception +29").unwrap();
        assert_eq!(result.stat.kind, StatType::Perception);
        assert!(float_eq(result.stat.value, 23.0));
        assert_eq!(result.proficiency, Proficiency::Low);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_skill() {
        let levels = Levels::new(3, 4).unwrap();

        let result = handle_prompt(levels, "skill +13").unwrap();
        assert_eq!(result.stat.kind, StatType::Skill);
        assert!(float_eq(result.stat.value, 15.0));
        assert_eq!(result.proficiency, Proficiency::Extreme);
        assert_eq!(result.method, ScaleMethod::Exact);
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
    fn scale_saving_throw() {
        let levels = Levels::new(6, 0).unwrap();

        let result = handle_prompt(levels, "save +11").unwrap();
        assert_eq!(result.stat.kind, StatType::SavingThrow);
        assert!(float_eq(result.stat.value, 3.0));
        assert_eq!(result.proficiency, Proficiency::Low);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_hit_points() {
        let levels = Levels::new(24, 10).unwrap();

        let result = handle_prompt(levels, "hp 367").unwrap();
        assert_eq!(result.stat.kind, StatType::HitPoints);
        assert!(float_eq(result.stat.value, 127.0));
        assert_eq!(result.proficiency, Proficiency::Low);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_resistance() {
        let levels = Levels::new(7, 12).unwrap();

        let result = handle_prompt(levels, "resistance 10").unwrap();
        assert_eq!(result.stat.kind, StatType::Resistance);
        assert!(float_eq(result.stat.value, 15.0));
        assert_eq!(result.proficiency, Proficiency::High);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_weakness() {
        let levels = Levels::new(8, 23).unwrap();

        let result = handle_prompt(levels, "weakness 6").unwrap();
        assert_eq!(result.stat.kind, StatType::Weakness);
        assert!(float_eq(result.stat.value, 13.0));
        assert_eq!(result.proficiency, Proficiency::Low);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_strike_attack_bonus() {
        let levels = Levels::new(11, 19).unwrap();

        let result = handle_prompt(levels, "strike-attack +24").unwrap();
        assert_eq!(result.stat.kind, StatType::StrikeAttackBonus);
        assert!(float_eq(result.stat.value, 36.0));
        assert_eq!(result.proficiency, Proficiency::High);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_strike_damage() {
        let levels = Levels::new(7, 17).unwrap();

        let result = handle_prompt(levels, "strike-damage 2d12+12 piercing").unwrap();
        assert_eq!(result.stat.kind, StatType::StrikeDamage);
        assert!(float_eq(result.stat.value, 50.0));
        assert_eq!(result.proficiency, Proficiency::Extreme);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_spell_dc() {
        let levels = Levels::new(22, 20).unwrap();

        let result = handle_prompt(levels, "spell-dc 50").unwrap();
        assert_eq!(result.stat.kind, StatType::SpellDC);
        assert!(float_eq(result.stat.value, 47.0));
        assert_eq!(result.proficiency, Proficiency::Extreme);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_spell_attack_bonus() {
        let levels = Levels::new(12, 5).unwrap();

        let result = handle_prompt(levels, "spell-attack +21").unwrap();
        assert_eq!(result.stat.kind, StatType::SpellAttackBonus);
        assert!(float_eq(result.stat.value, 11.0));
        assert_eq!(result.proficiency, Proficiency::Moderate);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_unlimited_area_damage() {
        let levels = Levels::new(6, 14).unwrap();

        let result = handle_prompt(levels, "unlimited-area-damage 4d6 fire").unwrap();
        assert_eq!(result.stat.kind, StatType::UnlimitedAreaDamage);
        assert!(float_eq(result.stat.value, 26.0));
        assert_eq!(result.proficiency, Proficiency::Moderate);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn scale_limited_area_damage() {
        let levels = Levels::new(17, 12).unwrap();

        let result = handle_prompt(levels, "limited-area-damage 18d6 cold").unwrap();
        assert_eq!(result.stat.kind, StatType::LimitedAreaDamage);
        assert!(float_eq(result.stat.value, 46.0));
        assert_eq!(result.proficiency, Proficiency::Moderate);
        assert_eq!(result.method, ScaleMethod::Exact);
    }
}
