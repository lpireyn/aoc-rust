//! Advent of Code 2025, day 4, part 1

use std::fs::File;

use day04::Grid;

fn main() {
    let solution = solve();
    println!("Solution: {solution}");
}

fn solve() -> usize {
    let grid = Grid::read_from_file(&File::open("input.txt").unwrap());
    grid.accessible_count(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 1553);
    }
}
