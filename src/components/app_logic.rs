use crate::components::payout_table::SortBy;

#[derive(Clone, PartialEq, Debug)]
pub struct TableRow {
    pub index: usize,
    pub line_label: String,
    pub avg: u32,
    pub max: u32,
    pub percent: f64,
}

pub fn prepare_rows(payouts: &[Vec<u32>]) -> Vec<TableRow> {
    payouts
        .iter()
        .enumerate()
        .map(|(i, vals)| {
            let avg = vals.iter().map(|&v| v as f64).sum::<f64>() / vals.len() as f64;
            let max = vals.iter().copied().max().unwrap_or(0);
            let max_count = vals.iter().filter(|&&v| v == max).count();
            let percent = (max_count as f64 / vals.len() as f64) * 100.0;
            let line_label = match i {
                0 => "Row 1",
                1 => "Row 2",
                2 => "Row 3",
                3 => "Col 1",
                4 => "Col 2",
                5 => "Col 3",
                6 => "Diag 1",
                7 => "Diag 2",
                _ => "",
            };
            TableRow {
                index: i,
                line_label: line_label.to_string(),
                avg: avg.floor() as u32,
                max,
                percent,
            }
        })
        .collect()
}

pub fn sort_rows(mut rows: Vec<TableRow>, sort_by: SortBy) -> Vec<TableRow> {
    rows.sort_by(|a, b| {
        let ord = match sort_by {
            SortBy::Avg => a.avg.cmp(&b.avg),
            SortBy::Max => a.max.cmp(&b.max),
        };
        ord.reverse()
    });
    rows
}
