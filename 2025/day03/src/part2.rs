//! Advent of Code 2025, day 3, part 2

use std::fs::File;

use day03::Bank;

fn main() {
    let solution = solve();
    println!("Solution: {solution}");
}

fn solve() -> u64 {
    let banks = Bank::read_from_file(&File::open("input.txt").unwrap());
    banks.iter().map(|bank| bank.max_joltage(12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 170418192256861);
    }
}
