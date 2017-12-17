use Day;

#[derive(Debug)]
struct Spinlock {
    buffer: Vec<usize>,
    pos: usize,
    step: usize,
}

impl Spinlock {
    fn new(step: usize) -> Spinlock {
        Spinlock {
            buffer: vec![0],
            pos: 1,
            step: step,
        }
    }

    fn tick(&mut self) {
        let value_to_insert = self.buffer.len();

        self.pos = (self.pos + self.step) % self.buffer.len() + 1;

        self.buffer.insert(self.pos, value_to_insert);
    }
}

pub struct Day17 {}

const INPUT: usize = 367;

impl Day<usize, usize> for Day17 {
    fn run1() -> usize {
        let mut spinlock = Spinlock::new(INPUT);

        for _ in 0..2017 {
            spinlock.tick();
        }

        spinlock.buffer[(spinlock.pos + 1) % spinlock.buffer.len()]
    }

    fn run2() -> usize {
        let mut last = 0;
        let mut last_idx = 0;
        for i in 1..50_000_001 {
            last_idx = (last_idx + INPUT) % i + 1;
            if last_idx == 1 {
                last = i
            }

        }

        last
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day17::run1(), 1487);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day17::run2(), 25674054);
    }
}
