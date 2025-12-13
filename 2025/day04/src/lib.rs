//! Advent of Code 2025, day 4

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid {
    width: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn read_from_file(file: &File) -> Self {
        let mut s = String::new();
        BufReader::new(file).read_to_string(&mut s).unwrap();
        s.parse().unwrap()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    pub fn cell(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x]
    }

    pub fn adjacent_count(&self, x: usize, y: usize) -> usize {
        let width = self.width;
        let height = self.height();
        let x1 = if x > 0 { x - 1 } else { x };
        let y1 = if y > 0 { y - 1 } else { y };
        let x2 = if x < width - 1 { x + 1 } else { x };
        let y2 = if y < height - 1 { y + 1 } else { y };
        let mut count = 0usize;
        for ax in x1..=x2 {
            for ay in y1..=y2 {
                if ax == x && ay == y {
                    continue;
                }
                if self.cell(ax, ay) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn accessible_count(&self, max_adjacent_count: usize) -> usize {
        let mut count = 0usize;
        for y in 0..self.height() {
            for x in 0..self.width {
                if self.cell(x, y) && self.adjacent_count(x, y) < max_adjacent_count {
                    count += 1;
                }
            }
        }
        count
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0usize;
        let cells = s
            .lines()
            .inspect(|line| {
                let len = line.len();
                if width == 0 {
                    width = len;
                } else if len != width {
                    panic!();
                }
            })
            .flat_map(|line| line.bytes().map(char_to_cell))
            .collect();
        let grid = Grid { width, cells };
        Ok(grid)
    }
}

fn char_to_cell(c: u8) -> bool {
    match c {
        b'@' => true,
        b'.' => false,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = Grid::read_from_file(&File::open("example.txt").unwrap());
        assert_eq!(grid.accessible_count(4), 13);
    }
}
