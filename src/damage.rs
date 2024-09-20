use crate::logging::{self, LogLevel};
use crate::tables::Proficiency;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub struct DamageComponent {
    pub average_value: f64,
    pub damage_type: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Damage {
    pub components: Vec<DamageComponent>,
}

impl Damage {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn total_average_value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for component in &self.components {
            total += component.average_value;
        }

        total
    }
}

impl Default for Damage {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_dice_expression(dice: &str) -> Option<f64> {
    let parts: Vec<&str> = dice.split('d').collect();

    if parts.len() != 2 {
        logging::log(
            LogLevel::Error,
            format!("invalid dice expression: {}", dice),
        );
        return None;
    }

    let num_dice: i32 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            logging::log(
                LogLevel::Error,
                format!("number of dice is not a valid integer: {}", parts[0]),
            );
            return None;
        }
    };
    let num_dice: f64 = num_dice.into();

    let avg_die_damage: f64 = match parts[1] {
        "4" => 2.5,
        "6" => 3.5,
        "8" => 4.5,
        "10" => 5.5,
        "12" => 6.5,
        _ => {
            logging::log(LogLevel::Error, format!("unknown die size: {}", parts[1]));
            return None;
        }
    };

    Some(num_dice * avg_die_damage)
}

fn parse_flat_modifier(modifier: &str) -> Option<f64> {
    let parsed = match modifier.parse() {
        Ok(m) => m,
        Err(_) => {
            logging::log(
                LogLevel::Error,
                format!("flat modifier is not a valid integer: {}", modifier),
            );
            return None;
        }
    };

    // Reject negative modifiers entirely for now, our parser splits on "+"
    // and we would have to input 1d4 + -1 for negative flat modifiers.
    if parsed < 0.0 {
        return None;
    }

    Some(parsed)
}

pub fn parse_damage_expression(expression: &str) -> Option<f64> {
    let parts: Vec<&str> = expression.split('+').collect();

    let mut total_avg_damage = 0.0;

    for part in parts {
        let part = part.trim();

        let avg_damage = if part.contains('d') {
            parse_dice_expression(part)
        } else {
            parse_flat_modifier(part)
        };

        let avg_damage = match avg_damage {
            Some(d) => d,
            None => return None,
        };

        total_avg_damage += avg_damage;
    }

    Some(total_avg_damage)
}

fn parse_damage_component(component: &str) -> Option<DamageComponent> {
    let (damage, damage_type) = match component.rsplit_once(' ') {
        Some((d, dt)) => (d, dt),
        None => {
            logging::log(
                LogLevel::Error,
                format!(
                    "failed to parse damage component: expected <dice_expression> <damage_type>, got {}",
                    component
            ));
            return None;
        }
    };

    let average_damage = match parse_damage_expression(damage) {
        Some(d) => d,
        None => return None,
    };

    let result = DamageComponent {
        average_value: average_damage,
        damage_type: String::from(damage_type),
    };

    Some(result)
}

pub fn parse_damage(expression: &str) -> Option<Damage> {
    let expression: String = expression.trim().to_lowercase();
    let components: Vec<&str> = expression.split("plus").collect();

    let mut result = Damage::new();

    for component in components {
        let component = component.trim();
        let parsed = parse_damage_component(component);
        match parsed {
            Some(c) => result.components.push(c),
            None => return None,
        }
    }

    Some(result)
}

struct ExpressionCandidate {
    pub target_damage_delta: f64,
    pub dice_flat_mod_delta: f64,
    pub die_size_preference: usize,
    pub expression: String,
}

fn compare_expressions(a: &ExpressionCandidate, b: &ExpressionCandidate) -> Ordering {
    let target_dmg = a.target_damage_delta.total_cmp(&b.target_damage_delta);
    let dice_mod = a.dice_flat_mod_delta.total_cmp(&b.dice_flat_mod_delta);
    let die_pref = a.die_size_preference.cmp(&b.die_size_preference);

    match target_dmg {
        Ordering::Equal => match dice_mod {
            Ordering::Equal => die_pref,
            _ => dice_mod,
        },
        _ => target_dmg,
    }
}

fn get_damage_expression_candidates(
    average_damage: f64,
    available_dice: &[i32],
) -> Vec<ExpressionCandidate> {
    let mut solutions: Vec<ExpressionCandidate> = Vec::new();

    for (i, die_size) in available_dice.iter().enumerate() {
        for n_dice in 1..=4 {
            let ds: f64 = (*die_size).into();
            let nd: f64 = n_dice.into();

            assert_eq!(die_size % 2, 0);
            let dice_avg = nd * (ds / 2.0 + 0.5);

            if dice_avg > average_damage {
                continue;
            }

            let flat_modifier = (average_damage - dice_avg).floor();
            let target_damage_delta = average_damage - dice_avg - flat_modifier;
            let dice_flat_mod_delta = (flat_modifier - dice_avg).abs();

            // We never want to exceed the expected damage.
            assert!(dice_avg + flat_modifier <= average_damage);

            let expression: String = if flat_modifier > 0.0 {
                format!("{}d{}+{}", n_dice, die_size, flat_modifier)
            } else {
                format!("{}d{}", n_dice, die_size)
            };

            let candidate = ExpressionCandidate {
                target_damage_delta,
                dice_flat_mod_delta,
                die_size_preference: i,
                expression,
            };

            solutions.push(candidate);
        }
    }

    solutions
}

pub fn build_damage_expression(average_damage: f64, proficiency: Proficiency) -> String {
    assert!(average_damage > 0.0);

    // The dice most suitable for a given proficiency. Use the first one if possible,
    // but if a less preferred one gives a more accurate damage expression, use it.
    let dice_preference = match proficiency {
        Proficiency::Extreme => vec![12, 10],
        Proficiency::High => vec![10, 12, 8],
        Proficiency::Moderate => vec![8, 10, 6],
        Proficiency::Low => vec![4, 6],
        Proficiency::Terrible => panic!("no terrible strike proficiency"),
    };

    let mut solutions = get_damage_expression_candidates(average_damage, &dice_preference);

    // No solutions with preferred dice, try all dice options. This happens if
    // our target damage is smaller than the average value of the preferred dice.
    if solutions.is_empty() {
        let all_dice = vec![12, 10, 8, 6, 4];
        solutions = get_damage_expression_candidates(average_damage, &all_dice);
    }

    // No solutions with any dice, return a flat damage number. This happens if
    // our target damage is smaller than the average of d4.
    if solutions.is_empty() {
        let constant_dmg = average_damage.floor();
        return format!("{}", constant_dmg);
    }

    solutions.sort_by(compare_expressions);

    solutions[0].expression.clone()
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::float_eq;

    #[test]
    fn accept_valid_flat_modifier() {
        assert!(float_eq(parse_flat_modifier("1").unwrap(), 1.0));
        assert!(float_eq(parse_flat_modifier("1234").unwrap(), 1234.0));
    }

    #[test]
    fn reject_invalid_flat_modifier() {
        assert_eq!(parse_flat_modifier(""), None);
        assert_eq!(parse_flat_modifier("d"), None);
        assert_eq!(parse_flat_modifier("1d"), None);
        assert_eq!(parse_flat_modifier("d4"), None);
        assert_eq!(parse_flat_modifier("1d4"), None);
        assert_eq!(parse_flat_modifier("x"), None);

        assert_eq!(parse_flat_modifier("-1"), None);
    }

    #[test]
    fn accept_valid_dice_expression() {
        assert!(float_eq(parse_dice_expression("1d4").unwrap(), 2.5));
        assert!(float_eq(parse_dice_expression("1d6").unwrap(), 3.5));
        assert!(float_eq(parse_dice_expression("1d8").unwrap(), 4.5));
        assert!(float_eq(parse_dice_expression("1d10").unwrap(), 5.5));
        assert!(float_eq(parse_dice_expression("1d12").unwrap(), 6.5));

        assert!(float_eq(parse_dice_expression("3d4").unwrap(), 7.5));
        assert!(float_eq(parse_dice_expression("7d6").unwrap(), 24.5));
        assert!(float_eq(parse_dice_expression("12d8").unwrap(), 54.0));
        assert!(float_eq(parse_dice_expression("10d10").unwrap(), 55.0));
        assert!(float_eq(parse_dice_expression("1234d12").unwrap(), 8021.0));
    }

    #[test]
    fn reject_invalid_dice_expression() {
        assert_eq!(parse_dice_expression(""), None);
        assert_eq!(parse_dice_expression("4"), None);
        assert_eq!(parse_dice_expression("d"), None);
        assert_eq!(parse_dice_expression("4d"), None);
        assert_eq!(parse_dice_expression("d4"), None);
        assert_eq!(parse_dice_expression("dd"), None);
        assert_eq!(parse_dice_expression("4dd4"), None);
        assert_eq!(parse_dice_expression("4d4d4"), None);

        assert_eq!(parse_dice_expression("1d5"), None);
        assert_eq!(parse_dice_expression("1d20"), None);

        assert_eq!(parse_dice_expression("1x4"), None);
        assert_eq!(parse_dice_expression("xd4"), None);
        assert_eq!(parse_dice_expression("4dx"), None);
        assert_eq!(parse_dice_expression("xdy"), None);

        assert_eq!(parse_dice_expression("1 d 4"), None);
        assert_eq!(parse_dice_expression("1 d4"), None);
        assert_eq!(parse_dice_expression("1d 4"), None);
        assert_eq!(parse_dice_expression("1 4"), None);
    }

    #[test]
    fn accept_valid_damage_expression() {
        assert!(float_eq(parse_damage_expression("2").unwrap(), 2.0));
        assert!(float_eq(parse_damage_expression("1d4").unwrap(), 2.5));
        assert!(float_eq(parse_damage_expression("2d6+7").unwrap(), 14.0));
        assert!(float_eq(parse_damage_expression("12d10+64").unwrap(), 130.0));
        assert!(float_eq(parse_damage_expression("1d4 + 2").unwrap(), 4.5));
        assert!(float_eq(parse_damage_expression("3d8 + 6 + 2").unwrap(), 21.5));
        assert!(float_eq(parse_damage_expression("3d8 + 1d4 + 15").unwrap(), 31.0));
        assert!(float_eq(parse_damage_expression("2+1d4").unwrap(), 4.5));
    }

    #[test]
    fn reject_invalid_damage_expression() {
        assert_eq!(parse_damage_expression("2d6+"), None);
        assert_eq!(parse_damage_expression("+2d6"), None);
        assert_eq!(parse_damage_expression("1d6++1d4"), None);
        assert_eq!(parse_damage_expression("1d6 fire"), None);

        assert_eq!(parse_damage_expression("1d8 - 3"), None);
    }

    #[test]
    fn accept_valid_damage_component() {
        let component = parse_damage_component("2d6 + 7 fire").unwrap();
        assert!(float_eq(component.average_value, 14.0));
        assert_eq!(component.damage_type, "fire");

        let component = parse_damage_component("1d4+1 untyped").unwrap();
        assert!(float_eq(component.average_value, 3.5));
        assert_eq!(component.damage_type, "untyped");
    }

    #[test]
    fn reject_invalid_damage_component() {
        assert_eq!(parse_damage_component("1d4 + 1"), None);
        assert_eq!(parse_damage_component("cold"), None);
        assert_eq!(parse_damage_component("1d4 + 1 fire cold"), None);
        assert_eq!(parse_damage_component("fire 1d4 + 1"), None);
        assert_eq!(parse_damage_component("fire 1d4 + 1 cold"), None);

        assert_eq!(parse_damage_component("3d6 + 2 persistent fire"), None);
    }

    #[test]
    fn accept_valid_damage() {
        let damage = parse_damage("3d12+20 bludgeoning").unwrap();
        assert_eq!(damage.components.len(), 1);
        assert!(float_eq(damage.components[0].average_value, 39.5));
        assert_eq!(damage.components[0].damage_type, "bludgeoning");
        assert_eq!(damage.total_average_value(), 39.5);

        let damage = parse_damage("2d8+11 piercing plus 1d6 fire").unwrap();
        assert_eq!(damage.components.len(), 2);
        assert!(float_eq(damage.components[0].average_value, 20.0));
        assert_eq!(damage.components[0].damage_type, "piercing");
        assert!(float_eq(damage.components[1].average_value, 3.5));
        assert_eq!(damage.components[1].damage_type, "fire");
        assert_eq!(damage.total_average_value(), 23.5);

        let damage = parse_damage("3d10+15 slashing plus 2d6 + 1 cold plus 1d4 electricity plus 1 void").unwrap();
        assert_eq!(damage.components.len(), 4);
        assert!(float_eq(damage.components[0].average_value, 31.5));
        assert_eq!(damage.components[0].damage_type, "slashing");
        assert!(float_eq(damage.components[1].average_value, 8.0));
        assert_eq!(damage.components[1].damage_type, "cold");
        assert!(float_eq(damage.components[2].average_value, 2.5));
        assert_eq!(damage.components[2].damage_type, "electricity");
        assert!(float_eq(damage.components[3].average_value, 1.0));
        assert_eq!(damage.components[3].damage_type, "void");
        assert_eq!(damage.total_average_value(), 43.0);
    }

    #[test]
    fn reject_invalid_damage() {
        assert_eq!(parse_damage("1d4 piercing plus"), None);
        assert_eq!(parse_damage("plus 1d4 piercing"), None);
        assert_eq!(parse_damage("2d6+5 piercing plus 1d6 persistent fire"), None);
        assert_eq!(parse_damage("2d8+9 piercing + 1d10 cold"), None);
    }

    #[test]
    fn test_damage_expression_builder_correctness() {
        let proficiencies = vec![
            Proficiency::Low,
            Proficiency::Moderate,
            Proficiency::High,
            Proficiency::Extreme,
        ];

        for prof in &proficiencies {
            let mut average_damage = 1.0;
            while average_damage <= 75.0 {
                let expression = build_damage_expression(average_damage, *prof);
                let parsed = parse_damage_expression(&expression)
                    .expect("the generated expression should always be valid");

                assert!((parsed - average_damage).abs() < 1.0 - 1e-6);

                average_damage += 0.25;
            }
        }
    }
}
