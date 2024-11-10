use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SavingThrowType {
    Fortitude,
    Reflex,
    Will,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatType {
    Perception,
    Skill,
    ArmorClass,
    SavingThrow(SavingThrowType),
    HitPoints,
    Resistance,
    Weakness,
    StrikeAttackBonus,
    StrikeDamage,
    SpellDC,
    SpellAttackBonus,
    UnlimitedAreaDamage,
    LimitedAreaDamage,
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatType::Perception => "perception",
            StatType::Skill => "skill",
            StatType::ArmorClass => "AC",
            StatType::SavingThrow(kind) => match kind {
                SavingThrowType::Fortitude => "fortitude",
                SavingThrowType::Reflex => "reflex",
                SavingThrowType::Will => "will",
            },
            StatType::HitPoints => "HP",
            StatType::Resistance => "resistance",
            StatType::Weakness => "weakness",
            StatType::StrikeAttackBonus => "strike-attack",
            StatType::StrikeDamage => "strike-damage",
            StatType::SpellDC => "spell-DC",
            StatType::SpellAttackBonus => "spell-attack",
            StatType::UnlimitedAreaDamage => "unlimited-area-damage",
            StatType::LimitedAreaDamage => "limited-area-damage",
        };

        write!(f, "{}", s)
    }
}

pub fn is_bonus(stat: StatType) -> bool {
    match stat {
        StatType::Perception
            | StatType::Skill
            | StatType::SavingThrow(_)
            | StatType::StrikeAttackBonus
            | StatType::SpellAttackBonus => true,
        _ => false,
    }
}

#[derive(Clone, Copy)]
pub struct Statistic {
    pub kind: StatType,
    pub value: f64,
}

impl Statistic {
    pub fn new(kind: StatType, value: f64) -> Self {
        Self { kind, value }
    }
}
