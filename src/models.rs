use crate::solver::possible;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Number {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

pub(crate) struct Grid {
    numbers: Vec<Option<Number>>,
}

impl Grid {
    fn max_value(&self, index: u8) -> u16 {
        let rows = self.rows(index);
        let max_keys = rows
            .iter()
            .filter_map(|row| {
                let remaining = self.remaining();
                let poss = possible(row, &remaining);
                let keys = poss.keys().cloned().collect::<Vec<u16>>();
                keys.iter().cloned().max()
            })
            .collect::<Vec<u16>>();
        max_keys.into_iter().max().unwrap_or_default()
    }

    fn remaining(&self) -> Vec<Number> {
        self.numbers.iter().filter_map(|r| *r).collect()
    }

    fn rows(&self, index: u8) -> Vec<Vec<Option<Number>>> {
        match index {
            0 => vec![self.row(0), self.column(0), self.backslash()],
            1 => vec![self.row(0), self.column(1)],
            2 => vec![self.row(0), self.column(2), self.slash()],
            3 => vec![self.row(1), self.column(0)],
            4 => vec![self.row(1), self.column(1), self.slash(), self.backslash()],
            5 => vec![self.row(1), self.column(2)],
            6 => vec![self.row(2), self.column(0), self.slash()],
            7 => vec![self.row(2), self.column(1)],
            8 => vec![self.row(2), self.column(2), self.backslash()],
            _ => unreachable!(),
        }
    }

    fn row(&self, index: u8) -> Vec<Option<Number>> {
        let row = (index * 3) as usize;
        vec![
            self.numbers[row],
            self.numbers[row + 1],
            self.numbers[row + 2],
        ]
    }

    fn column(&self, index: u8) -> Vec<Option<Number>> {
        let col = index as usize;
        vec![
            self.numbers[col],
            self.numbers[col + 3],
            self.numbers[col + 6],
        ]
    }

    fn slash(&self) -> Vec<Option<Number>> {
        vec![self.numbers[2], self.numbers[4], self.numbers[6]]
    }

    fn backslash(&self) -> Vec<Option<Number>> {
        vec![self.numbers[0], self.numbers[4], self.numbers[8]]
    }
}

#[test]
fn test_filled_grid_has_max_values() {
    let grid = Grid {
        numbers: vec![
            Some(Number::One),
            Some(Number::Two),
            Some(Number::Three),
            Some(Number::Four),
            Some(Number::Five),
            Some(Number::Six),
            Some(Number::Seven),
            Some(Number::Eight),
            Some(Number::Nine),
        ],
    };
    assert_eq!(grid.max_value(0), 10000);
    assert_eq!(grid.max_value(1), 10000);
    assert_eq!(grid.max_value(2), 10000);
    assert_eq!(grid.max_value(3), 180);
    assert_eq!(grid.max_value(4), 180);
    assert_eq!(grid.max_value(5), 180);
    assert_eq!(grid.max_value(6), 3600);
    assert_eq!(grid.max_value(6), 3600);
    assert_eq!(grid.max_value(6), 3600);
}
