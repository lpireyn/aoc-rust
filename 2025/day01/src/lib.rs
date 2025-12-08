//! Advent of Code 2025, day 1

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
pub struct Dial {
    size: u8,
    pointed: u8,
}

impl Dial {
    pub fn new(size: u8, pointed: u8) -> Self {
        Self { size, pointed }
    }

    pub fn pointed(&self) -> u8 {
        self.pointed
    }

    pub fn turn(&mut self, spec: &TurnSpec) {
        let size = self.size as i32;
        let pointed = self.pointed as i32;
        let TurnSpec(dir, clicks) = spec;
        let clicks = *clicks as i32;
        self.pointed = match dir {
            Direction::Left => {
                let m = (pointed - clicks) % size;
                if m < 0 { size + m } else { m }
            }
            Direction::Right => (pointed + clicks) % size,
        } as u8;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TurnSpec(Direction, u16);

impl TurnSpec {
    pub fn read_from_file(file: &File) -> Vec<TurnSpec> {
        BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.parse().unwrap())
            .collect()
    }
}

impl FromStr for TurnSpec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, clicks) = s.split_at(1);
        let dir: Direction = dir.parse().unwrap();
        let clicks: u16 = clicks.parse().unwrap();
        Ok(Self(dir, clicks))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointed() {
        let mut dial = Dial::new(100, 50);
        let specs = TurnSpec::read_from_file(&File::open("example.txt").unwrap());
        let mut count: u16 = 0;
        for spec in &specs {
            dial.turn(spec);
            if dial.pointed() == 0 {
                count += 1;
            }
        }
        assert_eq!(count, 3);
    }
}
