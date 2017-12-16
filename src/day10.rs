use Day;
use utils::{read_file, bytes_to_hex};
use hash::KnotHash;

pub struct Day10 {}

impl Day<usize, String> for Day10 {
    fn run1() -> usize {
        let lengths = read_file("data/day10")
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().expect(&format!("Invalid length {}", s)))
            .collect();

        let mut hash = KnotHash::new();

        hash.hash_one_round(&lengths);

        let result = hash.elements[..2].iter().map(|n| *n as usize).product();
        result
    }

    fn run2() -> String {
        let lengths: String = read_file("data/day10").trim().to_string();

        let mut hash = KnotHash::new();

        bytes_to_hex(&hash.hash_str(&lengths))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day10::run1(), 23874);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day10::run2(), "e1a65bfb5a5ce396025fab5528c25a87");
    }
}
