use itertools::Itertools;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::components::{
    app_logic::{TableRow, prepare_rows, sort_rows},
    payout_table::SortBy,
};

pub const MIN_NUM: u8 = 1;
pub const MAX_NUM: u8 = 9;
pub const GRID_SIZE: usize = 3;
pub const NUM_CELLS: usize = GRID_SIZE * GRID_SIZE;

// Indices for all lines (rows, columns, diagonals)
pub const LINES: [[usize; 3]; 8] = [
    // Rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    // Columns
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    // Diagonals
    [0, 4, 8],
    [2, 4, 6],
];

// Mini Cactpot payout table as a static HashMap
pub static PAYOUTS: OnceLock<HashMap<u8, u32>> = OnceLock::new();

fn init_payouts() -> HashMap<u8, u32> {
    [
        (6, 10000),
        (7, 36),
        (8, 720),
        (9, 360),
        (10, 80),
        (11, 252),
        (12, 108),
        (13, 72),
        (14, 54),
        (15, 180),
        (16, 72),
        (17, 180),
        (18, 119),
        (19, 36),
        (20, 306),
        (21, 1080),
        (22, 144),
        (23, 1800),
        (24, 3600),
    ]
    .iter()
    .copied()
    .collect()
}

pub fn payout_for_sum(sum: u8) -> u32 {
    let payouts = PAYOUTS.get_or_init(init_payouts);
    *payouts.get(&sum).unwrap_or(&0)
}

#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    pub cells: [Option<u8>; NUM_CELLS],
}

impl Default for Board {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl Board {
    fn new(nums: Vec<Option<u8>>) -> Self {
        let mut cells = [None; NUM_CELLS];
        for (i, v) in nums.into_iter().enumerate().take(NUM_CELLS) {
            cells[i] = v;
        }
        Self { cells }
    }

    pub fn used(&self) -> Vec<u8> {
        self.cells.iter().filter_map(|&n| n).collect()
    }

    fn filled_count(&self) -> usize {
        self.cells.iter().filter(|&n| n.is_some()).count()
    }

    pub fn max_inputs_reached(&self) -> bool {
        self.filled_count() >= 4
    }

    fn possible_line_payouts(&self) -> Vec<Vec<u32>> {
        possible_line_sums(self)
            .into_iter()
            .map(|sums| sums.into_iter().map(payout_for_sum).collect())
            .collect()
    }

    pub fn rows(&self, sort_by: SortBy) -> Vec<TableRow> {
        let payouts = self.possible_line_payouts();
        sort_rows(prepare_rows(&payouts), sort_by)
    }

    pub fn best_line_cells(&self, sort_by: SortBy) -> Option<[usize; 3]> {
        let rows = self.rows(sort_by);
        if self.max_inputs_reached() {
            Some(LINES[rows[0].index])
        } else {
            None
        }
    }
}

/// Given the current board state, return all possible sums for each line.
pub fn possible_line_sums(board: &Board) -> Vec<Vec<u8>> {
    let used = board.used();
    let unused = (MIN_NUM..=MAX_NUM)
        .filter(|n| !used.contains(n))
        .collect::<Vec<u8>>();
    LINES
        .iter()
        .map(|line| {
            let mut known = vec![];
            let mut unknown_indices = vec![];
            for &idx in line {
                match board.cells[idx] {
                    Some(n) => known.push(n),
                    None => unknown_indices.push(idx),
                }
            }
            if unknown_indices.is_empty() {
                // All known
                return vec![known.iter().sum()];
            }
            // For each permutation of unused numbers of length unknown_indices.len(),
            // assign them to the unknowns and compute the sum
            let mut sums = vec![];
            for combo in unused
                .iter()
                .copied()
                .permutations(unknown_indices.len())
                .unique()
            {
                let sum: u8 = known.iter().copied().chain(combo.into_iter()).sum();
                sums.push(sum);
            }
            sums.sort_unstable();
            sums.dedup();
            sums
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payout_for_sum_known() {
        assert_eq!(payout_for_sum(6), 10000);
        assert_eq!(payout_for_sum(7), 36);
        assert_eq!(payout_for_sum(24), 3600);
    }

    #[test]
    fn test_payout_for_sum_unknown() {
        assert_eq!(payout_for_sum(5), 0);
        assert_eq!(payout_for_sum(25), 0);
    }

    #[test]
    fn test_possible_line_sums_all_known() {
        let board = Board::new(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
        ]);
        // Row 0: 1+2+3=6, Row 1: 4+5+6=15, Row 2: 7+8+9=24
        let sums = possible_line_sums(&board);
        assert_eq!(sums[0], vec![6]);
        assert_eq!(sums[1], vec![15]);
        assert_eq!(sums[2], vec![24]);
    }

    #[test]
    fn test_possible_line_sums_with_unknowns() {
        let board = Board::new(vec![
            Some(1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ]);
        // Row 0: 1 + two unknowns (choose from 2..=9, no repeats)
        let sums = possible_line_sums(&board);
        assert_eq!(sums[0].len(), 13); // unique sums
        // All sums should be between 1+2+3=6 and 1+8+9=18
        assert!(sums[0].iter().all(|&s| s >= 6 && s <= 18));
    }
}
