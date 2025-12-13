//! Advent of Code 2025, day 3, part 1

use std::fs::File;

use day03::Bank;

fn main() {
    let solution = solve();
    println!("Solution: {solution}");
}

fn solve() -> u16 {
    let banks = Bank::read_from_file(&File::open("input.txt").unwrap());
    banks.iter().map(|bank| bank.max_joltage() as u16).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 17100);
    }
}
