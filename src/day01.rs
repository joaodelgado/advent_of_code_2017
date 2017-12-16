use Day;
use utils::read_file;

fn captcha(input: Vec<char>, offset: usize) -> u32 {
    let input_copy = input.clone();

    input
        .into_iter()
        .enumerate()
        .map(|(i, a)| {
            let j = (i + offset) % input_copy.len();
            if a == input_copy[j] {

                a.to_digit(10).expect("Unparsable int")
            } else {
                0
            }
        })
        .sum()
}

pub struct Day01 {}

impl Day<u32, u32> for Day01 {
    fn run1() -> u32 {
        let input: Vec<char> = read_file("data/day01").trim().chars().collect();
        captcha(input, 1)
    }

    fn run2() -> u32 {
        let input: Vec<char> = read_file("data/day01").trim().chars().collect();
        let offset = input.len() / 2;
        captcha(input, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day01::run1(), 1393);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day01::run2(), 1292);
    }
}
