use crate::statistic::StatType;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Proficiency {
    Terrible,
    Low,
    Moderate,
    High,
    Extreme,
}

impl fmt::Display for Proficiency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Proficiency::Terrible => "Terrible",
            Proficiency::Low => "Low",
            Proficiency::Moderate => "Moderate",
            Proficiency::High => "High",
            Proficiency::Extreme => "Extreme",
        };

        write!(f, "{}", s)
    }
}

pub struct StatTable {
    pub values: Vec<Vec<f64>>,
    pub proficiencies: Vec<Proficiency>,
}

pub fn get_table_for_statistic(stat: StatType) -> StatTable {
    match stat {
        StatType::ArmorClass => armor_class(),
    }
}

fn to_float(table: Vec<Vec<i32>>) -> Vec<Vec<f64>> {
    let mut v: Vec<Vec<f64>> = Vec::new();

    for row in table {
        let mut r: Vec<f64> = Vec::new();
        for x in row {
            r.push(x.into());
        }
        v.push(r);
    }

    v
}

fn armor_class() -> StatTable {
    let values = vec![
        vec![12, 14, 15, 18],
        vec![13, 15, 16, 19],
        vec![13, 15, 16, 19],
        vec![15, 17, 18, 21],
        vec![16, 18, 19, 22],
        vec![18, 20, 21, 24],
        vec![19, 21, 22, 25],
        vec![21, 23, 24, 27],
        vec![22, 24, 25, 28],
        vec![24, 26, 27, 30],
        vec![25, 27, 28, 31],
        vec![27, 29, 30, 33],
        vec![28, 30, 31, 34],
        vec![30, 32, 33, 36],
        vec![31, 33, 34, 37],
        vec![33, 35, 36, 39],
        vec![34, 36, 37, 40],
        vec![36, 38, 39, 42],
        vec![37, 39, 40, 43],
        vec![39, 41, 42, 45],
        vec![40, 42, 43, 46],
        vec![42, 44, 45, 48],
        vec![43, 45, 46, 49],
        vec![45, 47, 48, 51],
        vec![46, 48, 49, 52],
        vec![48, 50, 51, 54],
    ];
    let values = to_float(values);

    let proficiencies = vec![
        Proficiency::Low,
        Proficiency::Moderate,
        Proficiency::High,
        Proficiency::Extreme,
    ];

    StatTable {
        values,
        proficiencies,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::levels;

    fn validate_table(table: StatTable) {
        assert_eq!(table.values.len(), levels::num_levels());

        // All rows have the same number of columns.
        let columns = table.values[0].len();
        for row in &table.values {
            assert_eq!(row.len(), columns);
        }

        // Values increase when proficiency increases and don't decrease
        // when level increases.
        for i in 0..levels::num_levels() - 1 {
            for j in 0..columns - 1 {
                assert!(table.values[i][j] < table.values[i][j + 1]);
                assert!(table.values[i][j] <= table.values[i + 1][j]);
            }
        }

        assert_eq!(table.proficiencies.len(), columns);

        // Columns are in ascending proficiency order.
        for i in 0..columns - 1 {
            assert!(table.proficiencies[i] < table.proficiencies[i + 1]);
        }
    }

    #[test]
    fn validate_armor_class_table() {
        let ac = armor_class();
        validate_table(ac);
    }
}
