use std::collections::HashSet;
use std::collections::HashMap;
use utils::read_file;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    banks: Vec<usize>,
}

impl State {
    fn find_idx_max_bank(&self) -> usize {
        let mut curr_max = usize::min_value();
        let mut curr_index = 0;
        // How to make this work without calling `clone` on `banks`?
        for (i, bank) in self.banks.clone().into_iter().enumerate() {
            if bank > curr_max {
                curr_index = i;
                curr_max = bank;
            }
        }

        curr_index
    }

    fn distribute(&mut self, start_idx: usize, blocks: usize) {
        let mut curr_index = start_idx;
        let mut remaining = blocks;
        while remaining > 0 {
            curr_index = (curr_index + 1) % self.banks.len();
            self.banks[curr_index] += 1;
            remaining -= 1;
        }
    }

    fn cycle(&mut self) {
        let max_idx = self.find_idx_max_bank();

        let blocks = self.banks[max_idx];
        self.banks[max_idx] = 0;

        self.distribute(max_idx, blocks);
    }
}

fn read_input() -> State {
    State {
        banks: read_file("data/day06")
            .split('\t')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.trim().parse().expect(
                    &format!("Unparsable integer {}", s),
                )
            })
            .collect(),
    }
}

pub fn run1() {
    let mut curr = read_input();
    let mut previous_states: HashSet<State> = HashSet::new();
    let mut iterations = 0;

    loop {
        if previous_states.contains(&curr) {
            break;
        }
        iterations += 1;
        previous_states.insert(curr.clone());

        curr.cycle();
    }

    println!("Result: {}", iterations);
}

pub fn run2() {
    let mut curr = read_input();
    let mut previous_states: HashMap<State, usize> = HashMap::new();
    let mut iterations = 0;

    loop {
        if previous_states.contains_key(&curr) {
            break;
        }
        iterations += 1;
        previous_states.insert(curr.clone(), iterations);

        curr.cycle();
    }

    let result = iterations - previous_states.get(&curr).unwrap() + 1;
    println!("Result: {}", result);
}
