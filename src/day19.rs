use std::ops::Index;

use Day;
use graph::{Direction, Coord};
use utils::read_file;

pub trait CoordIndex<T> {
    fn get_coord(&self, coord: &Coord) -> Option<&T>;
}

impl CoordIndex<char> for Vec<Vec<char>> {
    fn get_coord(&self, coord: &Coord) -> Option<&char> {
        match self.get(coord.y as usize) {
            Some(line) => line.get(coord.x as usize),
            _ => None,
        }
    }
}

impl<'a> Index<&'a Coord> for Vec<Vec<char>> {
    type Output = char;

    fn index(&self, coord: &Coord) -> &char {
        &self[coord.y as usize][coord.x as usize]
    }
}

pub struct Day19 {}

impl Day<String, usize> for Day19 {
    fn run1() -> String {
        let graph: Vec<Vec<char>> = read_file("data/day19")
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect())
            .collect();

        let mut dir = Direction::N;
        let mut coord = Coord {
            x: graph[0]
                .iter()
                .enumerate()
                .find(|&(_, c)| !c.is_whitespace())
                .unwrap()
                .0 as isize,
            y: 0,
        };
        let mut visited = vec![];

        loop {
            coord.walk(&dir);
            match graph.get_coord(&coord) {
                Some(&c) if c.is_alphabetic() => visited.push(c),
                Some(&'+') => {
                    let next_dir = [Direction::S, Direction::W, Direction::N, Direction::E]
                        .iter()
                        .filter(|d| **d != dir && **d != dir.backwards())
                        .find(|d| match graph.get_coord(&d.walk(&coord)) {
                            Some(c) => !c.is_whitespace(),
                            _ => false,
                        });

                    match next_dir {
                        Some(d) => {
                            dir = d.clone();
                        }
                        None => unreachable!(),
                    }
                }
                Some(c) if c.is_whitespace() => break,
                None => break,
                _ => {}
            }
        }

        visited.iter().collect()
    }

    fn run2() -> usize {
        let graph: Vec<Vec<char>> = read_file("data/day19")
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect())
            .collect();

        let mut dir = Direction::N;
        let mut coord = Coord {
            x: graph[0]
                .iter()
                .enumerate()
                .find(|&(_, c)| !c.is_whitespace())
                .unwrap()
                .0 as isize,
            y: 0,
        };
        let mut visited = vec![];
        let mut steps = 0;

        loop {
            steps += 1;
            coord.walk(&dir);
            match graph.get_coord(&coord) {
                Some(&c) if c.is_alphabetic() => visited.push(c),
                Some(&'+') => {
                    let next_dir = [Direction::S, Direction::W, Direction::N, Direction::E]
                        .iter()
                        .filter(|d| **d != dir && **d != dir.backwards())
                        .find(|d| match graph.get_coord(&d.walk(&coord)) {
                            Some(c) => !c.is_whitespace(),
                            _ => false,
                        });

                    match next_dir {
                        Some(d) => {
                            dir = d.clone();
                        }
                        None => unreachable!(),
                    }
                }
                Some(c) if c.is_whitespace() => break,
                None => break,
                _ => {}
            }
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day19::run1(), "GSXDIPWTU");
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day19::run2(), 16100);
    }
}
