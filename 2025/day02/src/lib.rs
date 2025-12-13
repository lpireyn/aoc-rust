//! Advent of Code 2025, day 2

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
pub struct IdRange {
    pub first: u64,
    pub last: u64,
}

impl IdRange {
    pub fn new(first: u64, last: u64) -> Self {
        assert!(first <= last);
        Self { first, last }
    }

    pub fn read_from_file(file: &File) -> Vec<IdRange> {
        BufReader::new(file)
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|id_range| id_range.parse().unwrap())
            .collect()
    }

    pub fn invalid_ids_part1(&self) -> impl IntoIterator<Item = u64> {
        (self.first..=self.last).filter(|id| is_invalid_part1(*id))
    }

    pub fn invalid_ids_sum_part1(&self) -> u64 {
        self.invalid_ids_part1().into_iter().sum()
    }
}

impl FromStr for IdRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, last) = match s
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            &[first, last, ..] => (first, last),
            _ => panic!(),
        };
        Ok(Self { first, last })
    }
}

fn is_invalid_part1(id: u64) -> bool {
    let len = id.ilog10() + 1;
    if !len.is_multiple_of(2) {
        return false;
    }
    let m = 10u64.pow(len / 2);
    let second_half = id % m;
    id == second_half * m + second_half
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_part1() {
        assert!(!is_invalid_part1(10));
        assert!(is_invalid_part1(11));
        assert!(!is_invalid_part1(12));
        assert!(is_invalid_part1(1212));
        assert!(!is_invalid_part1(1213));
    }

    #[test]
    #[rustfmt::skip]
    fn test_example_part1() {
        assert_eq!(IdRange::new(11, 22).invalid_ids_sum_part1(), 11 + 22);
        assert_eq!(IdRange::new(95, 115).invalid_ids_sum_part1(), 99);
        assert_eq!(IdRange::new(998, 1012).invalid_ids_sum_part1(), 1010);
        assert_eq!(IdRange::new(1188511880, 1188511890).invalid_ids_sum_part1(), 1188511885);
        assert_eq!(IdRange::new(222220, 222224).invalid_ids_sum_part1(), 222222);
        assert_eq!(IdRange::new(1698522, 1698528).invalid_ids_sum_part1(), 0);
        assert_eq!(IdRange::new(446443, 446449).invalid_ids_sum_part1(), 446446);
        assert_eq!(IdRange::new(38593856, 38593862).invalid_ids_sum_part1(), 38593859);
        let id_ranges = IdRange::read_from_file(&File::open("example.txt").unwrap());
        let sum: u64 = id_ranges.iter()
            .map(|id_range| id_range.invalid_ids_sum_part1())
            .sum();
        assert_eq!(sum, 1227775554);
    }
}
