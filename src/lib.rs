use crate::levels::Levels;
use crate::scaling::ScaleResult;
use crate::statistic::Statistic;

pub mod damage;
pub mod levels;
pub mod logging;
pub mod scaling;
pub mod statistic;
pub mod tables;
pub mod utils;

pub fn scale_statistic(levels: Levels, stat: Statistic) -> ScaleResult {
    let table = tables::get_table_for_statistic(stat.kind);

    scaling::scale_by_table(levels, stat, table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scaling::ScaleMethod;
    use crate::statistic::StatType;
    use crate::tables::Proficiency;
    use crate::utils::float_eq;

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
}
