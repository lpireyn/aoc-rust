//! Advent of Code 2025, day 3

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

/// Battery with its joltage.
#[derive(Debug)]
pub struct Battery(u8);

/// Bank of batteries.
#[derive(Debug)]
pub struct Bank(Vec<Battery>);

impl Bank {
    pub fn read_from_file(file: &File) -> Vec<Bank> {
        BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.parse().unwrap())
            .collect()
    }

    pub fn max_joltage(&self, n: usize) -> u64 {
        let batteries = &self.0;
        let len = batteries.len();
        let mut total_joltage = 0u64;
        let mut first_index = 0usize;
        let mut last_index = len - (n - 1);
        for _ in 0..n {
            let mut max_joltage = 0u8;
            let mut max_index = 0usize;
            for (i, battery) in batteries
                .iter()
                .enumerate()
                .skip_while(|(index, _)| *index < first_index)
                .take_while(|(index, _)| *index < last_index)
            {
                let joltage = battery.0;
                if joltage > max_joltage {
                    max_joltage = joltage;
                    max_index = i;
                }
            }
            assert!(max_joltage > 0);
            total_joltage = total_joltage * 10 + max_joltage as u64;
            first_index = max_index + 1;
            last_index += 1;
        }
        total_joltage
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries: Vec<Battery> = s.bytes().map(|c| c - b'0').map(Battery).collect();
        Ok(Self(batteries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_example_n2() {
        assert_eq!(Bank::from_str("987654321111111").unwrap().max_joltage(2), 98);
        assert_eq!(Bank::from_str("811111111111119").unwrap().max_joltage(2), 89);
        assert_eq!(Bank::from_str("234234234234278").unwrap().max_joltage(2), 78);
        assert_eq!(Bank::from_str("818181911112111").unwrap().max_joltage(2), 92);
        let banks = Bank::read_from_file(&File::open("example.txt").unwrap());
        let max_joltage: u16 = banks.iter().map(|bank| bank.max_joltage(2) as u16).sum();
        assert_eq!(max_joltage, 357);
    }

    #[test]
    #[rustfmt::skip]
    fn test_example_n12() {
        assert_eq!(Bank::from_str("987654321111111").unwrap().max_joltage(12), 987654321111);
        assert_eq!(Bank::from_str("811111111111119").unwrap().max_joltage(12), 811111111119);
        assert_eq!(Bank::from_str("234234234234278").unwrap().max_joltage(12), 434234234278);
        assert_eq!(Bank::from_str("818181911112111").unwrap().max_joltage(12), 888911112111);
        let banks = Bank::read_from_file(&File::open("example.txt").unwrap());
        let max_joltage: u64 = banks.iter().map(|bank| bank.max_joltage(12)).sum();
        assert_eq!(max_joltage, 3121910778619);
    }
}
