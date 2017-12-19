use std::cmp::max;

use Day;
use utils::read_file;
use graph::{Coord, Direction};

fn read_input() -> Vec<Direction> {
    read_file("data/day11")
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect()
}

pub struct Day11 {}

impl Day<usize, usize> for Day11 {
    fn run1() -> usize {
        let directions = read_input();

        let mut coord = Coord { x: 0, y: 0 };
        for direction in directions {
            coord.walk_hex(&direction);
        }

        coord.dist_hex()
    }

    fn run2() -> usize {
        let directions = read_input();

        let mut coord = Coord { x: 0, y: 0 };
        let mut result = usize::min_value();

        for direction in directions {
            coord.walk_hex(&direction);
            result = max(result, coord.dist_hex());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day11::run1(), 796);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day11::run2(), 1585);
    }
}
