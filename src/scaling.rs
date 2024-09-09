use crate::levels::{Levels, MIN_LEVEL};
use crate::logging::{self, LogLevel};
use crate::statistic::Statistic;
use crate::tables::{Proficiency, StatTable};
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
            ScaleMethod::Exact => "exact",
            ScaleMethod::Interpolated => "interpolated",
            ScaleMethod::Extrapolated => "extrapolated",
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

pub fn scale_by_table(levels: Levels, stat: Statistic, table: StatTable) -> ScaleResult {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
