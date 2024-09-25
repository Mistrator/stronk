use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatType {
    Perception,
    Skill,
    ArmorClass,
    SavingThrow,
    HitPoints,
    Resistance,
    Weakness,
    StrikeAttackBonus,
    StrikeDamage,
    SpellDC,
    SpellAttackBonus,
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatType::Perception => "perception",
            StatType::Skill => "skill",
            StatType::ArmorClass => "AC",
            StatType::SavingThrow => "save",
            StatType::HitPoints => "HP",
            StatType::Resistance => "resistance",
            StatType::Weakness => "weakness",
            StatType::StrikeAttackBonus => "attack",
            StatType::StrikeDamage => "damage",
            StatType::SpellDC => "spell-DC",
            StatType::SpellAttackBonus => "spell-attack",
        };

        write!(f, "{}", s)
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
