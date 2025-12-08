//! Advent of Code 2025, day 1, part 2

use std::fs::File;

use day01::Dial;
use day01::TurnSpec;

fn main() {
    let solution = solve();
    println!("Solution: {solution}");
}

fn solve() -> u16 {
    let mut dial = Dial::new(100, 50);
    let specs = TurnSpec::read_from_file(&File::open("input.txt").unwrap());
    for spec in &specs {
        dial.turn(spec);
    }
    dial.paz_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 6819);
    }
}
