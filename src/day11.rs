use std::cmp::{min, max};
use utils::read_file;

struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn distance(&self) -> usize {
        let x = self.x.abs() as usize;
        let y = self.y.abs() as usize;
        let closest = min(x, y);
        let furthest = max(x, y);

        closest + (furthest - closest + 1) / 2
    }
}

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn walk(&self, coord: &Coord) -> Coord {
        match *self {
            Direction::N  => Coord{ x: coord.x,     y: coord.y + 2 },
            Direction::NE => Coord{ x: coord.x + 1, y: coord.y + 1 },
            Direction::E  => Coord{ x: coord.x + 2, y: coord.y },
            Direction::SE => Coord{ x: coord.x + 1, y: coord.y - 1 },
            Direction::S  => Coord{ x: coord.x,     y: coord.y - 2 },
            Direction::SW => Coord{ x: coord.x - 1, y: coord.y - 1 },
            Direction::W  => Coord{ x: coord.x - 2, y: coord.y },
            Direction::NW => Coord{ x: coord.x - 1, y: coord.y + 1 },

        }
    }
}

impl<'a> From<&'a str> for Direction {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn from(s: &'a str) -> Direction {
        match s.to_uppercase().trim() {
            "N"  => Direction::N,
            "NE" => Direction::NE,
            "E"  => Direction::E,
            "SE" => Direction::SE,
            "S"  => Direction::S,
            "SW" => Direction::SW,
            "W"  => Direction::W,
            "NW" => Direction::NW,
            _ => panic!("Unsupported direction: {}", s),
        }
    }
}

fn read_input() -> Vec<Direction> {
    read_file("data/day11")
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect()
}

pub fn run1() {
    let directions = read_input();

    let mut coord = Coord { x: 0, y: 0 };
    for direction in directions {
        coord = direction.walk(&coord);
    }

    let result = coord.distance();
    println!("Result: {}", result);
}

pub fn run2() {
    let directions = read_input();

    let mut coord = Coord { x: 0, y: 0 };
    let mut result = usize::min_value();

    for direction in directions {
        coord = direction.walk(&coord);
        result = max(result, coord.distance());
    }

    println!("Result: {}", result);
}
