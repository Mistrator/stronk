#[derive(Clone, Copy)]
pub enum StatType {
    ArmorClass,
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
