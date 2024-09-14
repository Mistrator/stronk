use crate::logging::{self, LogLevel};

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
}
