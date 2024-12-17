mod models;
mod solver;

use models::Number;
use solver::row_gil;

fn main() {
    println!("{}", row_gil(vec![Number::One, Number::Two, Number::Three]));
}
