use std::sync::mpsc;
use std::collections::HashMap;

use Day;
use utils::{read_file, ALPHABET};

#[derive(Debug, PartialEq)]
enum State {
    Running,
    Receiving,
    Stopped,
}

#[derive(Debug)]
struct Cpu<'a> {
    id: i64,
    program: &'a Vec<String>,
    registers: HashMap<char, i64>,
    sender: mpsc::Sender<i64>,
    receiver: mpsc::Receiver<i64>,
    state: State,
    pc: i64,
    values_sent: usize,
    conditional_receiving: bool,
}

impl<'a> Cpu<'a> {
    fn new(
        id: i64,
        program: &'a Vec<String>,
        sender: mpsc::Sender<i64>,
        receiver: mpsc::Receiver<i64>,
        conditional_receiving: bool,
    ) -> Cpu<'a> {
        let mut cpu = Cpu {
            id: id,
            program: program,
            registers: ALPHABET.chars().map(|c| (c, 0)).collect(),
            sender: sender,
            receiver: receiver,
            state: State::Running,
            pc: 0,
            values_sent: 0,
            conditional_receiving: conditional_receiving,
        };

        cpu.registers.insert('p', id);

        cpu
    }

    fn tick(&mut self) {
        let instr = match self.program.get(self.pc as usize) {
            Some(instr) => instr,
            None => {
                self.state = State::Stopped;
                return;
            }
        };
        println!("[{}] Running {}", self.id, instr);
        let mut words = instr.split_whitespace();

        match words.next() {
            Some("set") => {
                let r = words.next().unwrap().chars().nth(0).unwrap();
                let v = self.get_value(words.next().unwrap());
                self.registers.insert(r, v);
                self.pc += 1;
            }
            Some("add") => {
                let r = words.next().unwrap().chars().nth(0).unwrap();
                let v = self.get_value(words.next().unwrap());
                let sum = self.registers[&r] + v;
                self.registers.insert(r, sum);
                self.pc += 1;
            }
            Some("mul") => {
                let r = words.next().unwrap().chars().nth(0).unwrap();
                let v = self.get_value(words.next().unwrap());
                let sum = self.registers[&r] * v;
                self.registers.insert(r, sum);
                self.pc += 1;
            }
            Some("mod") => {
                let r = words.next().unwrap().chars().nth(0).unwrap();
                let v = self.get_value(words.next().unwrap());
                let sum = self.registers[&r] % v;
                self.registers.insert(r, sum);
                self.pc += 1;
            }
            Some("jgz") => {
                let x = self.get_value(words.next().unwrap());
                let y = self.get_value(words.next().unwrap());
                if x > 0 {
                    self.pc += y;
                } else {
                    self.pc += 1;
                }
            }
            Some("snd") => {
                let x = self.get_value(words.next().unwrap());
                self.sender.send(x).expect("Error sending data");
                self.values_sent += 1;
                self.pc += 1;
            }
            Some("rcv") => {
                let r = words.next().unwrap().chars().nth(0).unwrap();
                if !self.conditional_receiving || self.registers[&r] != 0 {
                    match self.receiver.try_recv() {
                        Ok(v) => {
                            self.registers.insert(r, v);
                            self.state = State::Running;
                        }
                        Err(_) => {
                            self.state = State::Receiving;
                            self.pc -= 1;
                        }
                    }
                }

                self.pc += 1;
            }
            _ => panic!("Unsupported operation {}", instr),
        }
    }

    fn get_value(&self, input: &str) -> i64 {
        input.parse().unwrap_or_else(|_| {
            self.registers[&input.chars().nth(0).unwrap()]
        })
    }

    fn is_running(&self) -> bool {
        self.state == State::Running
    }
}

fn read_program() -> Vec<String> {
    read_file("data/day18")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub struct Day18 {}

impl Day<i64, usize> for Day18 {
    fn run1() -> i64 {
        let program = read_program();
        let (p1_tx, p1_rx) = mpsc::channel();
        let (_, p2_rx) = mpsc::channel();

        let mut cpu = Cpu::new(0, &program, p1_tx, p2_rx, true);

        while cpu.is_running() {
            cpu.tick();
        }

        let mut result = 0;
        while let Ok(value) = p1_rx.try_recv() {
            result = value;
        }

        result
    }

    fn run2() -> usize {
        let program = read_program();
        let (p0_tx, p0_rx) = mpsc::channel();
        let (p1_tx, p1_rx) = mpsc::channel();

        let mut p0 = Cpu::new(0, &program, p0_tx, p1_rx, false);
        let mut p1 = Cpu::new(1, &program, p1_tx, p0_rx, false);

        while p0.is_running() || p1.is_running() {
            p0.tick();
            p1.tick();
        }

        p1.values_sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day18::run1(), 3423);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day18::run2(), 7493);
    }
}
