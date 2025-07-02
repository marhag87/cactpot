use itertools::Itertools;
use std::collections::HashMap;
use std::sync::OnceLock;

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

/// Given the current board state, return all possible sums for each line.
pub fn possible_line_sums(board: &[Option<u8>; NUM_CELLS]) -> Vec<Vec<u8>> {
    let used: Vec<u8> = board.iter().filter_map(|&n| n).collect();
    let unused: Vec<u8> = (MIN_NUM..=MAX_NUM).filter(|n| !used.contains(n)).collect();
    LINES
        .iter()
        .map(|line| {
            let mut known = vec![];
            let mut unknown_indices = vec![];
            for &idx in line {
                match board[idx] {
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

/// Given the current board state, return all possible payouts for each line.
pub fn possible_line_payouts(board: &[Option<u8>; NUM_CELLS]) -> Vec<Vec<u32>> {
    possible_line_sums(board)
        .into_iter()
        .map(|sums| sums.into_iter().map(payout_for_sum).collect())
        .collect()
}
