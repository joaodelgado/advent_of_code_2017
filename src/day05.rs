use Day;
use utils::read_file;

fn read_input() -> Vec<isize> {
    read_file("data/day05")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect(&format!("Unparsable integer {}", s)))
        .collect()
}

fn run<F>(increment: F) -> usize
where
    F: Fn(isize) -> isize,
{
    let mut instructions = read_input();
    let mut curr_offset = 0isize;
    let mut iter = 0;

    loop {
        let next_offset;
        match instructions.get(curr_offset as usize) {
            Some(incr) => next_offset = curr_offset + incr,
            None => break,
        }
        instructions[curr_offset as usize] = increment(instructions[curr_offset as usize]);
        curr_offset = next_offset;
        iter += 1;
    }

    iter
}

pub struct Day05 {}

impl Day<usize, usize> for Day05 {
    fn run1() -> usize {
        run(|n| n + 1)
    }

    fn run2() -> usize {
        run(|n| if n >= 3 { n - 1 } else { n + 1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day05::run1(), 375042);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day05::run2(), 28707598);
    }
}
