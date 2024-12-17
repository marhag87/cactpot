use crate::models::Number;
use std::collections::HashMap;

pub(crate) fn row_gil(nums: Vec<Number>) -> u16 {
    match nums.into_iter().map(|n| n as u8).sum() {
        6 => 10000,
        7 => 36,
        8 => 720,
        9 => 360,
        10 => 80,
        11 => 252,
        12 => 108,
        13 => 72,
        14 => 54,
        15 => 180,
        16 => 72,
        17 => 180,
        18 => 119,
        19 => 36,
        20 => 306,
        21 => 1080,
        22 => 144,
        23 => 1800,
        24 => 3600,
        _ => unreachable!(),
    }
}

pub(crate) fn possible(nums: &[Option<Number>], remaining: &[Number]) -> HashMap<u16, u8> {
    let mut possible_solutions = HashMap::new();
    let known = nums.iter().filter_map(|n| *n).collect::<Vec<Number>>();
    match known.len() {
        3 => {
            possible_solutions.insert(row_gil(known), 1);
        }
        0 | 1 | 2 => remaining.iter().for_each(|remaining_number| {
            let mut nums = known
                .iter()
                .map(|num| Some(num.clone()))
                .collect::<Vec<Option<Number>>>();
            nums.push(Some(remaining_number.clone()));
            let remaining = remaining
                .iter()
                .filter(|num| *num != remaining_number)
                .cloned()
                .collect::<Vec<Number>>();
            let map = possible(&nums, &remaining);
            for (key, value) in map.into_iter() {
                *possible_solutions.entry(key).or_default() += value;
            }
        }),
        _ => unimplemented!(),
    };
    possible_solutions
}

#[test]
fn test_possible_all_known() {
    let possible_solutions = possible(
        &[Some(Number::One), Some(Number::Two), Some(Number::Three)],
        &[
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
        ],
    );
    let expected = HashMap::from([(10000, 1)]);
    assert_eq!(possible_solutions, expected);
}

#[test]
fn test_possible_one_unknown() {
    let possible_solutions = possible(
        &[Some(Number::One), Some(Number::Two)],
        &[
            Number::Three,
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
        ],
    );
    let expected = HashMap::from([
        (10000, 1),
        (36, 1),
        (720, 1),
        (360, 1),
        (80, 1),
        (252, 1),
        (108, 1),
    ]);
    assert_eq!(possible_solutions, expected);
}

#[test]
fn test_possible_two_unknown() {
    let possible_solutions = possible(
        &[Some(Number::One)],
        &[
            Number::Two,
            Number::Three,
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
        ],
    );
    let expected = HashMap::from([
        (10000, 2),
        (36, 2),
        (720, 4),
        (360, 4),
        (80, 6),
        (252, 6),
        (108, 8),
        (72, 10),
        (54, 6),
        (180, 6),
        (119, 2),
    ]);
    assert_eq!(possible_solutions, expected);
}

#[test]
fn test_possible_three_unknown() {
    let possible_solutions = possible(
        &[],
        &[
            Number::One,
            Number::Two,
            Number::Three,
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
        ],
    );
    let expected = HashMap::from([
        (1080, 18),
        (1800, 6),
        (3600, 6),
        (10000, 6),
        (36, 36),
        (54, 48),
        (72, 90),
        (80, 24),
        (108, 42),
        (119, 42),
        (144, 12),
        (180, 90),
        (252, 30),
        (306, 24),
        (360, 18),
        (720, 12),
    ]);
    assert_eq!(possible_solutions, expected);
}
