use crate::levels::Levels;
use crate::scaling::ScaleResult;
use crate::statistic::Statistic;

pub mod levels;
pub mod logging;
pub mod scaling;
pub mod statistic;
pub mod tables;

pub fn scale_statistic(levels: Levels, stat: Statistic) -> ScaleResult {
    let table = tables::get_table_for_statistic(stat.kind);

    scaling::scale_by_table(levels, stat, table)
}
