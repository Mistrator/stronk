use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatType {
    ArmorClass,
    StrikeDamage,
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StatType::ArmorClass => "AC",
            StatType::StrikeDamage => "damage",
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
