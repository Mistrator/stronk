use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SavingThrowType {
    Fortitude,
    Reflex,
    Will,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SkillType {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatType {
    Perception,
    Skill(SkillType),
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
            StatType::Skill(kind) => match kind {
                SkillType::Acrobatics => "acrobatics",
                SkillType::Arcana => "arcana",
                SkillType::Athletics => "athletics",
                SkillType::Crafting => "crafting",
                SkillType::Deception => "deception",
                SkillType::Diplomacy => "diplomacy",
                SkillType::Intimidation => "intimidation",
                SkillType::Lore => "lore",
                SkillType::Medicine => "medicine",
                SkillType::Nature => "nature",
                SkillType::Occultism => "occultism",
                SkillType::Performance => "performance",
                SkillType::Religion => "religion",
                SkillType::Society => "society",
                SkillType::Stealth => "stealth",
                SkillType::Survival => "survival",
                SkillType::Thievery => "thievery",
            },
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

#[rustfmt::skip]
pub fn is_bonus(stat: StatType) -> bool {
    matches!(stat, StatType::Perception
        | StatType::Skill(_)
        | StatType::SavingThrow(_)
        | StatType::StrikeAttackBonus
        | StatType::SpellAttackBonus)
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
