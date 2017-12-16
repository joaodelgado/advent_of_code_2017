use Day;
use hash::KnotHash;
use graph::{Coord, Node, Graph};
use utils::bit_set;

impl From<Vec<Vec<u8>>> for Graph<Coord> {
    fn from(matrix: Vec<Vec<u8>>) -> Graph<Coord> {
        let mut graph = Graph::new();

        for (y, line) in matrix.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                for i in 0..8 {
                    let coord = Coord {
                        x: (x * 8 + i) as isize,
                        y: y as isize,
                    };
                    if bit_set(*cell, 7 - i as u8) {
                        let children = coord.adj();
                        graph.insert(Node {
                            id: coord,
                            children: children,
                        })
                    }
                }
            }
        }

        graph
    }
}

fn compute_hash_matrix(seed: &str) -> Vec<Vec<u8>> {
    let mut hashes = vec![];

    for i in 0..128 {
        let mut hash = KnotHash::new();
        let input = format!("{}-{}", seed, i);
        hashes.push(hash.hash_str(&input));
    }

    hashes
}

pub struct Day14 {}

impl Day<u32, usize> for Day14 {
    fn run1() -> u32 {
        let seed = "jzgqcdpd";

        let hashes = compute_hash_matrix(seed);

        hashes
            .iter()
            .flat_map(|hash| hash.iter())
            .map(|byte| byte.count_ones())
            .sum()
    }

    fn run2() -> usize {
        let seed = "jzgqcdpd";

        let hashes = compute_hash_matrix(seed);
        let groups = Graph::from(hashes).groups();

        groups.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day14::run1(), 8074);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day14::run2(), 1212);
    }
}
