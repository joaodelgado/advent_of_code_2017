use Day;

struct Generator {
    last: u64,
    factor: u64,
}

impl Generator {
    fn new(start: u64, factor: u64) -> Generator {
        Generator {
            last: start,
            factor: factor,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = (self.last * self.factor) % 2147483647;

        self.last = new_next;

        Some(new_next)
    }
}

pub struct Day15 {}

impl Day<usize, usize> for Day15 {
    fn run1() -> usize {
        let gen_a = Generator::new(591, 16807);
        let gen_b = Generator::new(393, 48271);

        let mask = 0xffff;
        gen_a
            .zip(gen_b)
            .take(40_000_000)
            .filter(|&(a, b)| (a & mask) == (b & mask))
            .count()
    }

    fn run2() -> usize {
        let gen_a = Generator::new(591, 16807);
        let gen_b = Generator::new(393, 48271);

        let mask = 0xffff;
        gen_a
            .filter(|n| n % 4 == 0)
            .zip(gen_b.filter(|n| n % 8 == 0))
            .take(5_000_000)
            .filter(|&(a, b)| (a & mask) == (b & mask))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day15::run1(), 619);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day15::run2(), 290);
    }
}
