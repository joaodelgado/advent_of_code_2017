use std::cmp::{max, min};
use Day;
use utils::read_file;

fn read_input() -> Vec<Vec<u64>> {
    read_file("data/day02")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split('\t')
                .map(|n| n.parse().expect("Unparsable number"))
                .collect()
        })
        .collect()
}

pub struct Day02 {}

impl Day<u64, u64> for Day02 {
    fn run1() -> u64 {
        read_input()
            .into_iter()
            .map(|line| {
                line.into_iter().fold(
                    (u64::max_value(), u64::min_value()),
                    |(curr_min, curr_max), x| {
                        (min(curr_min, x), max(curr_max, x))
                    },
                )
            })
            .map(|(min, max)| max - min)
            .sum()
    }

    fn run2() -> u64 {
        read_input()
            .into_iter()
            .map(|line| {
                for (i, a) in line.clone().into_iter().enumerate() {
                    for &b in line[i + 1..].into_iter() {
                        let max = max(a, b);
                        let min = min(a, b);
                        if max % min == 0 {
                            return max / min;
                        }
                    }
                }
                return 0;
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day02::run1(), 43074);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day02::run2(), 280);
    }
}
