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

    pub fn max_joltage(&self) -> u8 {
        let batteries = &self.0;
        // The first battery is the first one with the largest joltage, excl. the last one
        // Unfortunately, the various `max` methods of the `Iterator` trait return the last item found,
        // so we need to do it ourselves
        let len = batteries.len();
        let mut max_joltage = 0u8;
        let mut max_index = 0usize;
        for (i, battery) in batteries.iter().enumerate().take(len - 1) {
            let joltage = battery.0;
            if joltage > max_joltage {
                max_joltage = joltage;
                max_index = i;
            }
        }
        assert!(max_joltage > 0);
        let first = max_joltage;
        // The second battery is the following one with the largest joltage
        max_joltage = 0;
        for battery in batteries.iter().skip(max_index + 1) {
            let joltage = battery.0;
            if joltage > max_joltage {
                max_joltage = joltage;
                // We don't need `max_index` this time
            }
        }
        assert!(max_joltage > 0);
        let second = max_joltage;
        first * 10 + second
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
    fn test_example() {
        assert_eq!(Bank::from_str("987654321111111").unwrap().max_joltage(), 98);
        assert_eq!(Bank::from_str("811111111111119").unwrap().max_joltage(), 89);
        assert_eq!(Bank::from_str("234234234234278").unwrap().max_joltage(), 78);
        assert_eq!(Bank::from_str("818181911112111").unwrap().max_joltage(), 92);
        let banks = Bank::read_from_file(&File::open("example.txt").unwrap());
        let max_joltage: u16 = banks.iter().map(|bank| bank.max_joltage() as u16).sum();
        assert_eq!(max_joltage, 357);
    }
}
