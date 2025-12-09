//! Advent of Code 2025, day 2, part 1

use std::fs::File;

use day02::IdRange;

fn main() {
    let solution = solve();
    println!("Solution: {solution}");
}

fn solve() -> u64 {
    let id_ranges = IdRange::read_from_file(&File::open("input.txt").unwrap());
    id_ranges
        .iter()
        .map(|id_range| id_range.invalid_ids_sum())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 12850231731);
    }
}
