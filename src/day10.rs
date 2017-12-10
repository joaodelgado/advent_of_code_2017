use utils::{read_file, bytes_to_hex};

struct KnotHash {
    elements: Vec<u8>,
    pos: usize,
    skip: usize,
}

impl KnotHash {
    fn new() -> KnotHash {
        KnotHash {
            elements: (0..256).map(|n| n as u8).collect(),
            pos: 0,
            skip: 0,
        }
    }

    fn _reverse(&mut self, length: usize) {
        let should_wrap = self.elements.len() < self.pos + length;

        if should_wrap {
            let wrap_size = (self.pos + length) % self.elements.len();

            let mut intact = self.elements[wrap_size..self.pos].to_owned();
            let mut to_reverse = [&self.elements[self.pos..], &self.elements[..wrap_size]].concat();
            to_reverse.reverse();

            let mut reverse_tail = to_reverse[..length - wrap_size].to_owned();
            let reverse_head = to_reverse[length - wrap_size..].to_owned();

            self.elements = reverse_head;
            self.elements.append(&mut intact);
            self.elements.append(&mut reverse_tail);
        } else {
            let mut to_reverse = self.elements.split_off(self.pos);
            let mut tail = to_reverse.split_off(length);

            to_reverse.reverse();

            self.elements.append(&mut to_reverse);
            self.elements.append(&mut tail);
        }

    }

    fn _hash_single(&mut self, length: usize) {
        self._reverse(length);
        self.pos = (self.pos + length + self.skip) % self.elements.len();
        self.skip += 1;
    }

    fn hash(&mut self, lengths: &Vec<usize>) {
        for &length in lengths {
            self._hash_single(length);
        }
    }

    fn dense_hash(&self) -> Vec<u8> {
        self.elements
            .chunks(16)
            .map(|c| c[1..].iter().fold(c[0], |acc, &n| acc ^ n))
            .collect()
    }
}

pub fn run1() {
    let lengths = read_file("data/day10")
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect(&format!("Invalid length {}", s)))
        .collect();

    let mut hash = KnotHash::new();

    hash.hash(&lengths);

    let result: usize = hash.elements[..2].iter().map(|n| *n as usize).product();
    println!("Result: {}", result);
}

pub fn run2() {
    let mut lengths: Vec<usize> = read_file("data/day10")
        .trim()
        .chars()
        .map(|c| c as usize)
        .collect();

    lengths.extend([17, 31, 73, 47, 23].iter());

    let mut hash = KnotHash::new();

    for _ in 0..64 {
        hash.hash(&lengths);
    }

    let result = bytes_to_hex(&hash.dense_hash());
    println!("Result: {}", result);
}
