use crate::damage::{Damage, DamageComponent};
use crate::levels::{Levels, MIN_LEVEL};
use crate::logging::{self, LogLevel};
use crate::statistic::Statistic;
use crate::tables::{self, Proficiency, StatTable};
use crate::utils::float_eq;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScaleMethod {
    Exact,
    Interpolated,
    Extrapolated,
}

impl fmt::Display for ScaleMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ScaleMethod::Exact => "Exact",
            ScaleMethod::Interpolated => "Interpolated",
            ScaleMethod::Extrapolated => "Extrapolated",
        };

        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy)]
pub struct ScaleResult {
    pub stat: Statistic,
    pub proficiency: Proficiency,
    pub method: ScaleMethod,
}

// Map a value from ]al, ar[ to [bl, br].
// a-interval can't be a point, b-interval can.
fn interpolate(al: f64, ar: f64, bl: f64, br: f64, val: f64) -> f64 {
    assert!(al < ar);
    assert!(bl <= br);
    assert!(al < val && val < ar);

    let ratio = (val - al) / (ar - al);
    let result = bl + ratio * (br - bl);

    assert!(bl <= result && result <= br);

    result
}

// Map a value that is outside current min/max by x to outside
// target min/max by the same x.
fn extrapolate(cur_edge: f64, tgt_edge: f64, value: f64) -> f64 {
    assert!(!float_eq(cur_edge, value));

    if value <= cur_edge {
        let diff = cur_edge - value;
        tgt_edge - diff
    } else {
        let diff = value - cur_edge;
        tgt_edge + diff
    }
}

fn scale_by_table(levels: Levels, stat: Statistic, table: StatTable) -> ScaleResult {
    let cur_row_i: usize = (levels.current - MIN_LEVEL)
        .try_into()
        .expect("levels should be in range");

    let tgt_row_i: usize = (levels.target - MIN_LEVEL)
        .try_into()
        .expect("levels should be in range");

    let cur_row = &table.values[cur_row_i];
    let tgt_row = &table.values[tgt_row_i];

    let cur_min = *cur_row.first().unwrap();
    let cur_max = *cur_row.last().unwrap();

    if stat.value < cur_min || stat.value > cur_max {
        if stat.value < cur_min {
            logging::log(
                LogLevel::Warning,
                format!(
                    "{} {} is too low for a level {} creature: minimum {}",
                    stat.kind, stat.value, levels.current, cur_min
                ),
            );
        } else {
            logging::log(
                LogLevel::Warning,
                format!(
                    "{} {} is too high for a level {} creature: maximum {}",
                    stat.kind, stat.value, levels.current, cur_max
                ),
            );
        }

        let edge = if stat.value < cur_min {
            0
        } else {
            cur_row.len() - 1
        };

        let scaled = extrapolate(cur_row[edge], tgt_row[edge], stat.value);
        return ScaleResult {
            stat: Statistic::new(stat.kind, scaled),
            proficiency: table.proficiencies[edge],
            method: ScaleMethod::Extrapolated,
        };
    }

    for i in 0..cur_row.len() {
        if float_eq(cur_row[i], stat.value) {
            let scaled = tgt_row[i];
            return ScaleResult {
                stat: Statistic::new(stat.kind, scaled),
                proficiency: table.proficiencies[i],
                method: ScaleMethod::Exact,
            };
        }

        if i < cur_row.len() - 1 && cur_row[i] < stat.value && cur_row[i + 1] > stat.value {
            let scaled = interpolate(
                cur_row[i],
                cur_row[i + 1],
                tgt_row[i],
                tgt_row[i + 1],
                stat.value,
            );

            return ScaleResult {
                stat: Statistic::new(stat.kind, scaled),
                proficiency: table.proficiencies[i],
                method: ScaleMethod::Interpolated,
            };
        }
    }

    unreachable!("we should always either get an exact result, interpolate or extrapolate");
}

fn scale_all_damage_components(damage: &Damage, scaled_total: f64) -> Damage {
    assert!(scaled_total > 0.0);
    assert!(!damage.components.is_empty());

    let current_total = damage.total_average_value();

    let mut scaled_damage = Damage::new();

    let ratios: Vec<f64> = damage
        .components
        .iter()
        .map(|c| c.average_value / current_total)
        .collect();

    #[allow(clippy::needless_range_loop)]
    for i in 0..damage.components.len() {
        let scaled_average_damage = ratios[i] * scaled_total;
        let new_component = DamageComponent {
            average_value: scaled_average_damage,
            damage_type: damage.components[i].damage_type.clone(),
        };
        scaled_damage.components.push(new_component);
    }

    scaled_damage
}

fn scale_first_damage_component(damage: &Damage, scaled_total: f64) -> Damage {
    assert!(scaled_total > 0.0);
    assert!(!damage.components.is_empty());

    let current_total = damage.total_average_value();
    let delta = scaled_total - current_total;

    assert!(damage.components[0].average_value + delta > 0.0);

    let mut scaled_damage = damage.clone();
    scaled_damage.components[0].average_value += delta;

    scaled_damage
}

// Only scale the first component if possible and leave the others untouched.
// Typically this means scaling the main physical damage and not touching the extra
// elemental damage. If we scale down so much that the first component goes to zero,
// scale every component proportionally instead.
pub fn scale_damage_components(damage: &Damage, scaled_total: f64) -> Damage {
    let current_total = damage.total_average_value();
    let delta = scaled_total - current_total;

    if damage.components.len() >= 2 && damage.components[0].average_value + delta <= 0.0 {
        logging::log(
            LogLevel::Info,
            "damage was greatly decreased: scaling all damage components proportionally",
        );

        scale_all_damage_components(damage, scaled_total)
    } else {
        scale_first_damage_component(damage, scaled_total)
    }
}

pub fn scale_statistic(levels: Levels, stat: Statistic) -> ScaleResult {
    let table = tables::get_table_for_statistic(stat.kind);

    scale_by_table(levels, stat, table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scaling::ScaleMethod;
    use crate::statistic::StatType;
    use crate::tables::Proficiency;
    use crate::utils::float_eq;

    #[test]
    fn test_interpolate() {
        // identity map
        assert!(float_eq(interpolate(0.0, 1.0, 0.0, 1.0, 0.5), 0.5));
        assert!(float_eq(interpolate(1.0, 2.0, 1.0, 2.0, 1.5), 1.5));
        assert!(float_eq(interpolate(2.0, 4.0, 2.0, 4.0, 3.0), 3.0));

        // equal interval up
        assert!(float_eq(interpolate(2.0, 5.0, 7.0, 10.0, 3.0), 8.0));

        // equal interval down
        assert!(float_eq(interpolate(12.0, 16.0, 4.0, 8.0, 15.0), 7.0));

        // increasing interval
        assert!(float_eq(interpolate(5.0, 8.0, 10.0, 16.0, 7.0), 14.0));
        assert!(float_eq(interpolate(1.0, 3.0, 1.0, 2.0, 2.0), 1.5));

        // decreasing interval
        assert!(float_eq(interpolate(3.0, 6.0, 1.0, 2.0, 4.0), 4.0 / 3.0));
        assert!(float_eq(interpolate(3.0, 6.0, 1.0, 2.0, 5.0), 5.0 / 3.0));

        // negative intervals
        assert!(float_eq(interpolate(-4.0, -2.0, -10.0, -6.0, -3.0), -8.0));
        assert!(float_eq(interpolate(-4.0, -2.0, 6.0, 10.0, -3.0), 8.0));
        assert!(float_eq(interpolate(2.0, 4.0, -10.0, -6.0, 3.0), -8.0));

        // map to point-b
        assert!(float_eq(interpolate(1.0, 5.0, 4.0, 4.0, 2.0), 4.0));
        assert!(float_eq(interpolate(1.0, 5.0, 7.0, 7.0, 4.0), 7.0));
        assert!(float_eq(interpolate(1.0, 5.0, 0.0, 0.0, 4.0), 0.0));
    }

    #[test]
    fn test_extrapolate() {
        assert!(float_eq(extrapolate(0.0, 1.0, 2.0), 3.0));
        assert!(float_eq(extrapolate(0.0, 1.0, -2.0), -1.0));

        assert!(float_eq(extrapolate(9.0, 3.0, 14.0), 8.0));
        assert!(float_eq(extrapolate(9.0, 3.0, 2.0), -4.0));

        assert!(float_eq(extrapolate(-5.0, -2.0, -3.0), 0.0));
        assert!(float_eq(extrapolate(-5.0, -2.0, -7.0), -4.0));

        assert!(float_eq(extrapolate(-4.0, -8.0, -1.0), -5.0));
        assert!(float_eq(extrapolate(-4.0, -8.0, -7.0), -11.0));
    }

    #[test]
    fn test_scale_all_damage_components() {
        let mut damage = Damage::new();

        let first = DamageComponent {
            average_value: 15.0,
            damage_type: String::from("bludgeoning"),
        };
        damage.components.push(first);

        let target_total = 12.0;
        let scaled = scale_all_damage_components(&damage, target_total);
        assert!(float_eq(scaled.total_average_value(), target_total));
        assert!(float_eq(scaled.components[0].average_value, 12.0));

        let second = DamageComponent {
            average_value: 5.0,
            damage_type: String::from("fire"),
        };
        damage.components.push(second);

        let third = DamageComponent {
            average_value: 1.0,
            damage_type: String::from("void"),
        };
        damage.components.push(third);

        let scaled = scale_all_damage_components(&damage, target_total);
        assert!(float_eq(scaled.total_average_value(), target_total));
        assert!(float_eq(scaled.components[0].average_value, 8.5714285));
        assert!(float_eq(scaled.components[1].average_value, 2.8571428));
        assert!(float_eq(scaled.components[2].average_value, 0.5714285));
    }

    #[test]
    fn test_scale_first_damage_component() {
        let mut damage = Damage::new();

        let first = DamageComponent {
            average_value: 15.0,
            damage_type: String::from("bludgeoning"),
        };
        damage.components.push(first);

        let target_total = 12.0;
        let scaled = scale_first_damage_component(&damage, target_total);
        assert!(float_eq(scaled.total_average_value(), target_total));
        assert!(float_eq(scaled.components[0].average_value, 12.0));

        let second = DamageComponent {
            average_value: 5.0,
            damage_type: String::from("fire"),
        };
        damage.components.push(second);

        let third = DamageComponent {
            average_value: 1.0,
            damage_type: String::from("void"),
        };
        damage.components.push(third);

        let scaled = scale_first_damage_component(&damage, target_total);
        assert!(float_eq(scaled.total_average_value(), target_total));
        assert!(float_eq(scaled.components[0].average_value, 6.0));
        assert!(float_eq(scaled.components[1].average_value, 5.0));
        assert!(float_eq(scaled.components[2].average_value, 1.0));
    }

    #[test]
    fn armor_class_exact_scale() {
        let levels = Levels::new(4, 17).unwrap();
        let stat = Statistic::new(StatType::ArmorClass, 21.0);

        let result = scale_statistic(levels, stat);

        assert_eq!(result.stat.kind, StatType::ArmorClass);
        assert!(float_eq(result.stat.value, 40.0));
        assert_eq!(result.proficiency, Proficiency::High);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn armor_class_interpolate() {
        let levels = Levels::new(11, 2).unwrap();
        let stat = Statistic::new(StatType::ArmorClass, 32.0);

        let result = scale_statistic(levels, stat);

        assert_eq!(result.stat.kind, StatType::ArmorClass);
        assert!(float_eq(result.stat.value, 19.0));
        assert_eq!(result.proficiency, Proficiency::High);
        assert_eq!(result.method, ScaleMethod::Interpolated);

        let stat = Statistic::new(StatType::ArmorClass, 33.0);
        let result = scale_statistic(levels, stat);

        assert_eq!(result.stat.kind, StatType::ArmorClass);
        assert!(float_eq(result.stat.value, 20.0));
        assert_eq!(result.proficiency, Proficiency::High);
        assert_eq!(result.method, ScaleMethod::Interpolated);
    }

    #[test]
    fn armor_class_extrapolate() {
        let levels = Levels::new(5, 9).unwrap();
        let stat = Statistic::new(StatType::ArmorClass, 17.0);

        let result = scale_statistic(levels, stat);

        assert_eq!(result.stat.kind, StatType::ArmorClass);
        assert!(float_eq(result.stat.value, 23.0));
        assert_eq!(result.proficiency, Proficiency::Low);
        assert_eq!(result.method, ScaleMethod::Extrapolated);
    }

    #[test]
    fn strike_damage_exact_scale() {
        let levels = Levels::new(8, 13).unwrap();
        let stat = Statistic::new(StatType::StrikeDamage, 22.0);

        let result = scale_statistic(levels, stat);

        assert_eq!(result.stat.kind, StatType::StrikeDamage);
        assert!(float_eq(result.stat.value, 32.0));
        assert_eq!(result.proficiency, Proficiency::High);
        assert_eq!(result.method, ScaleMethod::Exact);
    }

    #[test]
    fn strike_damage_interpolate() {
        let levels = Levels::new(14, 6).unwrap();
        let stat = Statistic::new(StatType::StrikeDamage, 30.0);

        let result = scale_statistic(levels, stat);

        assert_eq!(result.stat.kind, StatType::StrikeDamage);
        assert!(float_eq(result.stat.value, 16.0));
        assert_eq!(result.proficiency, Proficiency::Moderate);
        assert_eq!(result.method, ScaleMethod::Interpolated);
    }

    #[test]
    fn strike_damage_extrapolate() {
        let levels = Levels::new(18, 24).unwrap();
        let stat = Statistic::new(StatType::StrikeDamage, 57.0);

        let result = scale_statistic(levels, stat);

        assert_eq!(result.stat.kind, StatType::StrikeDamage);
        assert!(float_eq(result.stat.value, 72.0));
        assert_eq!(result.proficiency, Proficiency::Extreme);
        assert_eq!(result.method, ScaleMethod::Extrapolated);
    }
}
