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
        StatType::Skill => skills(),
        StatType::ArmorClass => armor_class(),
        StatType::SavingThrow(_) => saving_throws(),
        StatType::HitPoints => hit_points(),
        StatType::Resistance => resistance_weakness(),
        StatType::Weakness => resistance_weakness(),
        StatType::StrikeAttackBonus => strike_attack_bonus(),
        StatType::StrikeDamage => strike_damage(),
        StatType::SpellDC => spell_dc(),
        StatType::SpellAttackBonus => spell_attack_bonus(),
        StatType::UnlimitedAreaDamage => unlimited_area_damage(),
        StatType::LimitedAreaDamage => limited_area_damage(),
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

fn skills() -> StatTable {
    let values = vec![
        vec![1, 2, 4, 5, 8],
        vec![2, 3, 5, 6, 9],
        vec![3, 4, 6, 7, 10],
        vec![4, 5, 7, 8, 11],
        vec![5, 7, 9, 10, 13],
        vec![7, 8, 10, 12, 15],
        vec![8, 10, 12, 13, 16],
        vec![9, 11, 13, 15, 18],
        vec![11, 13, 15, 17, 20],
        vec![12, 14, 16, 18, 21],
        vec![13, 16, 18, 20, 23],
        vec![15, 17, 19, 22, 25],
        vec![16, 19, 21, 23, 26],
        vec![17, 20, 22, 25, 28],
        vec![19, 22, 24, 27, 30],
        vec![20, 23, 25, 28, 31],
        vec![21, 25, 27, 30, 33],
        vec![23, 26, 28, 32, 35],
        vec![24, 28, 30, 33, 36],
        vec![25, 29, 31, 35, 38],
        vec![27, 31, 33, 37, 40],
        vec![28, 32, 34, 38, 41],
        vec![29, 34, 36, 40, 43],
        vec![31, 35, 37, 42, 45],
        vec![32, 36, 38, 43, 46],
        vec![33, 38, 40, 45, 48],
    ];

    let values = to_float(values);

    let proficiencies = vec![
        Proficiency::Low,
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

fn saving_throws() -> StatTable {
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

fn hit_points() -> StatTable {
    let values = vec![
        vec![5, 6, 7, 8, 9, 9],
        vec![11, 13, 14, 16, 17, 20],
        vec![14, 16, 19, 21, 24, 26],
        vec![21, 25, 28, 32, 36, 40],
        vec![31, 37, 42, 48, 53, 59],
        vec![42, 48, 57, 63, 72, 78],
        vec![53, 59, 72, 78, 91, 97],
        vec![67, 75, 91, 99, 115, 123],
        vec![82, 90, 111, 119, 140, 148],
        vec![97, 105, 131, 139, 165, 173],
        vec![112, 120, 151, 159, 190, 198],
        vec![127, 135, 171, 179, 215, 223],
        vec![142, 150, 191, 199, 240, 248],
        vec![157, 165, 211, 219, 265, 273],
        vec![172, 180, 231, 239, 290, 298],
        vec![187, 195, 251, 259, 315, 323],
        vec![202, 210, 271, 279, 340, 348],
        vec![217, 225, 291, 299, 365, 373],
        vec![232, 240, 311, 319, 390, 398],
        vec![247, 255, 331, 339, 415, 423],
        vec![262, 270, 351, 359, 440, 448],
        vec![277, 285, 371, 379, 465, 473],
        vec![295, 305, 395, 405, 495, 505],
        vec![317, 329, 424, 436, 532, 544],
        vec![339, 351, 454, 466, 569, 581],
        vec![367, 383, 492, 508, 617, 633],
    ];
    let values = to_float(values);

    let proficiencies = vec![
        Proficiency::Low,
        Proficiency::Low,
        Proficiency::Moderate,
        Proficiency::Moderate,
        Proficiency::High,
        Proficiency::High,
    ];

    StatTable {
        values,
        proficiencies,
    }
}

fn resistance_weakness() -> StatTable {
    let values = vec![
        vec![1, 1],
        vec![1, 3],
        vec![2, 3],
        vec![2, 5],
        vec![3, 6],
        vec![4, 7],
        vec![4, 8],
        vec![5, 9],
        vec![5, 10],
        vec![6, 11],
        vec![6, 12],
        vec![7, 13],
        vec![7, 14],
        vec![8, 15],
        vec![8, 16],
        vec![9, 17],
        vec![9, 18],
        vec![9, 19],
        vec![10, 19],
        vec![10, 20],
        vec![11, 21],
        vec![11, 22],
        vec![12, 23],
        vec![12, 24],
        vec![13, 25],
        vec![13, 26],
    ];
    let values = to_float(values);

    let proficiencies = vec![Proficiency::Low, Proficiency::High];

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
        vec![2, 3, 3, 4],
        vec![3, 4, 5, 6],
        vec![4, 5, 6, 8],
        vec![6, 8, 9, 11],
        vec![8, 10, 12, 15],
        vec![9, 12, 14, 18],
        vec![11, 13, 16, 20],
        vec![12, 15, 18, 23],
        vec![13, 17, 20, 25],
        vec![15, 18, 22, 28],
        vec![16, 20, 24, 30],
        vec![17, 22, 26, 33],
        vec![19, 23, 28, 35],
        vec![20, 25, 30, 38],
        vec![21, 27, 32, 40],
        vec![23, 28, 34, 43],
        vec![24, 30, 36, 45],
        vec![25, 31, 37, 48],
        vec![26, 32, 38, 50],
        vec![27, 33, 40, 53],
        vec![28, 35, 42, 55],
        vec![29, 37, 44, 58],
        vec![31, 38, 46, 60],
        vec![32, 40, 48, 63],
        vec![33, 42, 50, 65],
        vec![35, 44, 52, 68],
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

fn spell_dc() -> StatTable {
    let values = vec![
        vec![13, 16, 19],
        vec![13, 16, 19],
        vec![14, 17, 20],
        vec![15, 18, 22],
        vec![17, 20, 23],
        vec![18, 21, 25],
        vec![19, 22, 26],
        vec![21, 24, 27],
        vec![22, 25, 29],
        vec![23, 26, 30],
        vec![25, 28, 32],
        vec![26, 29, 33],
        vec![27, 30, 34],
        vec![29, 32, 36],
        vec![30, 33, 37],
        vec![31, 34, 39],
        vec![33, 36, 40],
        vec![34, 37, 41],
        vec![35, 38, 43],
        vec![37, 40, 44],
        vec![38, 41, 46],
        vec![39, 42, 47],
        vec![41, 44, 48],
        vec![42, 45, 50],
        vec![43, 46, 51],
        vec![45, 48, 52],
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

fn unlimited_area_damage() -> StatTable {
    let values = vec![
        vec![2],
        vec![4],
        vec![5],
        vec![7],
        vec![9],
        vec![11],
        vec![12],
        vec![14],
        vec![15],
        vec![17],
        vec![18],
        vec![20],
        vec![21],
        vec![23],
        vec![24],
        vec![26],
        vec![27],
        vec![28],
        vec![29],
        vec![30],
        vec![32],
        vec![33],
        vec![35],
        vec![36],
        vec![38],
        vec![39],
    ];
    let values = to_float(values);

    let proficiencies = vec![Proficiency::Moderate];

    StatTable {
        values,
        proficiencies,
    }
}

fn limited_area_damage() -> StatTable {
    let values = vec![
        vec![4],
        vec![6],
        vec![7],
        vec![11],
        vec![14],
        vec![18],
        vec![21],
        vec![25],
        vec![28],
        vec![32],
        vec![35],
        vec![39],
        vec![42],
        vec![46],
        vec![49],
        vec![53],
        vec![56],
        vec![60],
        vec![63],
        vec![67],
        vec![70],
        vec![74],
        vec![77],
        vec![81],
        vec![84],
        vec![88],
    ];
    let values = to_float(values);

    let proficiencies = vec![Proficiency::Moderate];

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

        // Columns are in non-descending proficiency order.
        for i in 0..columns - 1 {
            assert!(table.proficiencies[i] <= table.proficiencies[i + 1]);
        }
    }

    #[test]
    fn validate_perception_table() {
        let table = perception();
        validate_table(table);
    }

    #[test]
    fn validate_skills_table() {
        let table = skills();
        validate_table(table);
    }

    #[test]
    fn validate_armor_class_table() {
        let table = armor_class();
        validate_table(table);
    }

    #[test]
    fn validate_saving_throws_table() {
        let table = saving_throws();
        validate_table(table);
    }

    #[test]
    fn validate_hit_points_table() {
        let table = hit_points();
        validate_table(table);
    }

    #[test]
    fn validate_resistance_weakness_table() {
        let table = resistance_weakness();
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
    fn validate_spell_dc_table() {
        let table = spell_dc();
        validate_table(table);
    }

    #[test]
    fn validate_spell_attack_bonus_table() {
        let table = spell_attack_bonus();
        validate_table(table);
    }

    #[test]
    fn validate_unlimited_area_damage_table() {
        let table = unlimited_area_damage();
        validate_table(table);
    }

    #[test]
    fn validate_limited_area_damage_table() {
        let table = limited_area_damage();
        validate_table(table);
    }
}
