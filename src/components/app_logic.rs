use crate::components::payout_table::SortBy;
use crate::logic;

pub type TableRow = (usize, String, u32, u32, f64);

pub fn prepare_rows(payouts: &[Vec<u32>]) -> Vec<TableRow> {
    payouts
        .iter()
        .enumerate()
        .map(|(i, vals)| {
            let avg = if vals.is_empty() {
                0.0
            } else {
                vals.iter().map(|&v| v as f64).sum::<f64>() / vals.len() as f64
            };
            let max = vals.iter().copied().max().unwrap_or(0);
            let max_count = vals.iter().filter(|&&v| v == max).count();
            let percent = if !vals.is_empty() && max > 0 {
                (max_count as f64 / vals.len() as f64) * 100.0
            } else {
                0.0
            };
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
            (i, line_label.to_string(), avg.floor() as u32, max, percent)
        })
        .collect()
}

pub fn sort_rows(mut rows: Vec<TableRow>, sort_by: SortBy) -> Vec<TableRow> {
    rows.sort_by(|a, b| {
        let ord = match sort_by {
            SortBy::Avg => a.2.cmp(&b.2),
            SortBy::Max => a.3.cmp(&b.3),
        };
        ord.reverse()
    });
    rows
}

pub fn get_best_line_cells(rows: &[TableRow], filled_count: usize) -> Option<[usize; 3]> {
    if filled_count == 4 {
        Some(logic::LINES[rows[0].0])
    } else {
        None
    }
}
