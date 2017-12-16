use Day;
use utils::read_file;

pub struct Day09 {}

impl Day<usize, usize> for Day09 {
    fn run1() -> usize {
        let input = read_file("data/day09");

        let mut garbage = false;
        let mut escaping = false;
        let mut nest_lvl = 0;
        let mut result = 0;

        for c in input.chars() {

            if escaping {
                escaping = false;
                continue;
            }

            if garbage {
                match c {
                    '>' => garbage = false,
                    '!' => escaping = true,
                    _ => (),
                }
                continue;
            }

            match c {
                '<' => garbage = true,
                '{' => nest_lvl += 1,
                '}' => {
                    result += nest_lvl;
                    nest_lvl -= 1;
                }
                _ => (),
            }

        }

        result
    }

    fn run2() -> usize {
        let input = read_file("data/day09");

        let mut garbage = false;
        let mut escaping = false;
        let mut result = 0;

        for c in input.chars() {

            if escaping {
                escaping = false;
                continue;
            }

            if garbage {
                match c {
                    '>' => garbage = false,
                    '!' => escaping = true,
                    _ => result += 1,
                }
                continue;
            }

            match c {
                '<' => garbage = true,
                _ => (),
            }

        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day09::run1(), 14421);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day09::run2(), 6817);
    }
}
