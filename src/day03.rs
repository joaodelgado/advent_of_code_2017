use std::collections::HashMap;

use Day;
use graph::Coord;

impl Coord {
    fn from_spiral(n: usize) -> Coord {
        let mut x = 0isize;
        let mut y = 0isize;
        let mut dx = 0;
        let mut dy = -1;
        for _ in 1..n {
            if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
                let tmp = dx;
                dx = -dy;
                dy = tmp;
            }
            x += dx;
            y += dy;
        }

        Coord { x: x, y: y }
    }
}

pub struct Day03 {}

impl Day<usize, usize> for Day03 {
    fn run1() -> usize {
        let input = 312051;
        let coord = Coord::from_spiral(input);
        coord.dist()
    }

    fn run2() -> usize {
        let input = 312051;

        let mut matrix: HashMap<Coord, usize> = HashMap::new();

        matrix.insert(Coord { x: 0, y: 0 }, 1);

        let mut n = 2;
        loop {
            let coord = Coord::from_spiral(n);
            let value = coord
                .adj_with_diagonals()
                .into_iter()
                .map(|c| match matrix.get(&c) {
                    Some(&v) => v,
                    None => 0,
                })
                .sum();

            if value > input {
                return value;
            }

            matrix.insert(coord, value);
            n += 1;

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day03::run1(), 430);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day03::run2(), 312453);
    }
}
