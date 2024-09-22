use crate::damage;
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
        StatType::Perception => perception(),
        StatType::ArmorClass => armor_class(),
        StatType::StrikeAttackBonus => strike_attack_bonus(),
        StatType::StrikeDamage => strike_damage(),
        StatType::SpellAttackBonus => spell_attack_bonus(),
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

fn to_average_damage(table: Vec<Vec<&str>>) -> Vec<Vec<f64>> {
    let mut v: Vec<Vec<f64>> = Vec::new();

    for row in table {
        let mut r: Vec<f64> = Vec::new();
        for x in row {
            let average_damage = damage::parse_damage_expression(x)
                .expect("tables should only have valid damage expressions");
            r.push(average_damage);
        }
        v.push(r);
    }

    v
}

fn perception() -> StatTable {
    let values = vec![
        vec![0, 2, 5, 8, 9],
        vec![1, 3, 6, 9, 10],
        vec![2, 4, 7, 10, 11],
        vec![3, 5, 8, 11, 12],
        vec![4, 6, 9, 12, 14],
        vec![6, 8, 11, 14, 15],
        vec![7, 9, 12, 15, 17],
        vec![8, 11, 14, 17, 18],
        vec![10, 12, 15, 18, 20],
        vec![11, 13, 16, 19, 21],
        vec![12, 15, 18, 21, 23],
        vec![14, 16, 19, 22, 24],
        vec![15, 18, 21, 24, 26],
        vec![16, 19, 22, 25, 27],
        vec![18, 20, 23, 26, 29],
        vec![19, 22, 25, 28, 30],
        vec![20, 23, 26, 29, 32],
        vec![22, 25, 28, 30, 33],
        vec![23, 26, 29, 32, 35],
        vec![24, 27, 30, 33, 36],
        vec![26, 29, 32, 35, 38],
        vec![27, 30, 33, 36, 39],
        vec![28, 32, 35, 38, 41],
        vec![30, 33, 36, 39, 43],
        vec![31, 34, 37, 40, 44],
        vec![32, 36, 38, 42, 46],
    ];

    let values = to_float(values);

    let proficiencies = vec![
        Proficiency::Terrible,
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

fn strike_attack_bonus() -> StatTable {
    let values = vec![
        vec![4, 6, 8, 10],
        vec![4, 6, 8, 10],
        vec![5, 7, 9, 11],
        vec![7, 9, 11, 13],
        vec![8, 10, 12, 14],
        vec![9, 12, 14, 16],
        vec![11, 13, 15, 17],
        vec![12, 15, 17, 19],
        vec![13, 16, 18, 20],
        vec![15, 18, 20, 22],
        vec![16, 19, 21, 23],
        vec![17, 21, 23, 25],
        vec![19, 22, 24, 27],
        vec![20, 24, 26, 28],
        vec![21, 25, 27, 29],
        vec![23, 27, 29, 31],
        vec![24, 28, 30, 32],
        vec![25, 30, 32, 34],
        vec![27, 31, 33, 35],
        vec![28, 33, 35, 37],
        vec![29, 34, 36, 38],
        vec![31, 36, 38, 40],
        vec![32, 37, 39, 41],
        vec![33, 39, 41, 43],
        vec![35, 40, 42, 44],
        vec![36, 42, 44, 46],
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

fn strike_damage() -> StatTable {
    let values = vec![
        vec!["1d4", "1d4", "1d4+1", "1d6+1"],
        vec!["1d4+1", "1d4+2", "1d6+2", "1d6+3"],
        vec!["1d4+2", "1d6+2", "1d6+3", "1d8+4"],
        vec!["1d6+3", "1d8+4", "1d10+4", "1d12+4"],
        vec!["1d6+5", "1d8+6", "1d10+6", "1d12+8"],
        vec!["2d4+4", "2d6+5", "2d8+5", "2d10+7"],
        vec!["2d4+6", "2d6+6", "2d8+7", "2d12+7"],
        vec!["2d4+7", "2d6+8", "2d8+9", "2d12+10"],
        vec!["2d6+6", "2d8+8", "2d10+9", "2d12+12"],
        vec!["2d6+8", "2d8+9", "2d10+11", "2d12+15"],
        vec!["2d6+9", "2d8+11", "2d10+13", "2d12+17"],
        vec!["2d6+10", "2d10+11", "2d12+13", "2d12+20"],
        vec!["2d8+10", "2d10+12", "2d12+15", "2d12+22"],
        vec!["3d6+10", "3d8+12", "3d10+14", "3d12+19"],
        vec!["3d6+11", "3d8+14", "3d10+16", "3d12+21"],
        vec!["3d6+13", "3d8+15", "3d10+18", "3d12+24"],
        vec!["3d6+14", "3d10+14", "3d12+17", "3d12+26"],
        vec!["3d6+15", "3d10+15", "3d12+18", "3d12+29"],
        vec!["3d6+16", "3d10+16", "3d12+19", "3d12+31"],
        vec!["3d6+17", "3d10+17", "3d12+20", "3d12+34"],
        vec!["4d6+14", "4d8+17", "4d10+20", "4d12+29"],
        vec!["4d6+15", "4d8+19", "4d10+22", "4d12+32"],
        vec!["4d6+17", "4d8+20", "4d10+24", "4d12+34"],
        vec!["4d6+18", "4d8+22", "4d10+26", "4d12+37"],
        vec!["4d6+19", "4d10+20", "4d12+24", "4d12+39"],
        vec!["4d6+21", "4d10+22", "4d12+26", "4d12+42"],
    ];
    let values = to_average_damage(values);

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

fn spell_attack_bonus() -> StatTable {
    let values = vec![
        vec![5, 8, 11],
        vec![5, 8, 11],
        vec![6, 9, 12],
        vec![7, 10, 14],
        vec![9, 12, 15],
        vec![10, 13, 17],
        vec![11, 14, 18],
        vec![13, 16, 19],
        vec![14, 17, 21],
        vec![15, 18, 22],
        vec![17, 20, 24],
        vec![18, 21, 25],
        vec![19, 22, 26],
        vec![21, 24, 28],
        vec![22, 25, 29],
        vec![23, 26, 31],
        vec![25, 28, 32],
        vec![26, 29, 33],
        vec![27, 30, 35],
        vec![29, 32, 36],
        vec![30, 33, 38],
        vec![31, 34, 39],
        vec![33, 36, 40],
        vec![34, 37, 42],
        vec![35, 38, 43],
        vec![37, 40, 44],
    ];
    let values = to_float(values);

    let proficiencies = vec![
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

        // Values don't decrease when proficiency or level increases.
        for i in 0..levels::num_levels() - 1 {
            for j in 0..columns - 1 {
                assert!(table.values[i][j] <= table.values[i][j + 1]);
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
    fn validate_perception_table() {
        let table = perception();
        validate_table(table);
    }

    #[test]
    fn validate_armor_class_table() {
        let table = armor_class();
        validate_table(table);
    }

    #[test]
    fn validate_strike_attack_bonus_table() {
        let table = strike_attack_bonus();
        validate_table(table);
    }

    #[test]
    fn validate_strike_damage_table() {
        let table = strike_damage();
        validate_table(table);
    }

    #[test]
    fn validate_spell_attack_bonus_table() {
        let table = spell_attack_bonus();
        validate_table(table);
    }
}
