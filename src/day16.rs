use Day;
use std::collections::HashMap;
use utils::read_file;

#[derive(Debug)]
enum Instr {
    Spin(u8),
    Exchange(u8, u8),
    Partner(char, char),
}

impl<'a> From<&'a str> for Instr {
    fn from(s: &'a str) -> Instr {
        let mut chars = s.chars();

        match chars.next() {
            Some('s') => Instr::Spin(chars.collect::<String>().parse().expect(
                "'s' operation must be followed by a number",
            )),
            Some('x') => {
                let indexes: Vec<u8> = chars
                    .collect::<String>()
                    .split('/')
                    .map(|s| s.parse().expect("Unparsable integer"))
                    .collect();
                Instr::Exchange(indexes[0], indexes[1])
            }
            Some('p') => {
                let first = chars.next().expect(
                    "'p' operation must be followed by a char",
                );

                // Consume the slash
                chars.next();

                let second = chars.next().expect("'p' operation must end with a char");
                Instr::Partner(first, second)
            }
            _ => panic!("Unsupported operation {}", s),
        }
    }
}

struct State {
    programs: Vec<char>,
}

impl State {
    fn new() -> State {
        State { programs: ('a' as u8..'p' as u8 + 1).map(|n| n as char).collect() }
    }

    fn execute(&mut self, instrs: &Vec<Instr>) {
        for instr in instrs.iter() {
            self.execute_single(instr);
        }
    }

    fn execute_single(&mut self, instr: &Instr) {
        use self::Instr::*;

        match *instr {
            Spin(n) => {
                let split_idx = self.programs.len() - n as usize;
                let mut tail = self.programs.split_off(split_idx);
                tail.append(&mut self.programs);
                self.programs = tail;
            }
            Exchange(first, second) => self.programs.swap(first as usize, second as usize),
            Partner(first, second) => {
                let first_idx = self.programs.iter().position(|&c| c == first).expect(
                    &format!(
                        "Cannot find program '{}'",
                        first
                    ),
                );
                let second_idx = self.programs.iter().position(|&c| c == second).expect(
                    &format!(
                        "Cannot find program '{}'",
                        second
                    ),
                );
                self.programs.swap(first_idx, second_idx);
            }
        }
    }

    fn state(&self) -> String {
        self.programs.iter().collect()
    }
}

fn read_instr() -> Vec<Instr> {
    read_file("data/day16")
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Instr::from)
        .collect()
}

pub struct Day16 {}

impl Day<String, String> for Day16 {
    fn run1() -> String {
        let instrs = read_instr();
        let mut state = State::new();

        state.execute(&instrs);

        state.state()
    }

    fn run2() -> String {
        let instrs = read_instr();
        let mut state = State::new();

        let mut known_states = HashMap::new();
        let mut period = 0;

        for i in 0.. {
            state.execute(&instrs);

            if known_states
                .values()
                .position(|s| *s == state.state())
                .is_some()
            {
                period = i;
                break;
            }

            known_states.insert(i, state.state());
        }

        let final_iteration = (1_000_000_000 % period) - 1;

        known_states[&final_iteration].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day16::run1(), "ionlbkfeajgdmphc");
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day16::run2(), "fdnphiegakolcmjb");
    }
}
