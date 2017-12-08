use std::cmp::max;
use std::collections::HashMap;
use regex::Regex;

use utils::read_file;


#[derive(Default)]
struct Cpu {
    registers: HashMap<String, isize>,
}

impl Cpu {
    fn execute(&mut self, instr: &str) {
        //
        // Parse
        //

        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?P<target>\w+)\s+(?P<op>\w+)\s+(?P<amount>-?\d+)\s+if\s+(?P<cond_reg>\w+)\s+(?P<cond>[=><!]+)\s+(?P<cond_val>-?\d+)"
            ).expect("Unparsable regex");
        }

        let cap = RE.captures(instr).expect(&format!(
            "Instr didn't match regex: '{}'",
            instr
        ));

        let target = cap.name("target")
            .expect(&format!("Didn't capture target for instr: '{}'", instr))
            .as_str()
            .to_string();
        let op = cap.name("op")
            .expect(&format!("Didn't capture op for instr: '{}'", instr))
            .as_str();
        let amount: isize = cap.name("amount")
            .expect(&format!("Didn't capture amount for instr: '{}'", instr))
            .as_str()
            .parse()
            .expect(&format!("Unparsable amount for instr: '{}'", instr));
        let cond_reg = cap.name("cond_reg")
            .expect(&format!("Didn't capture cond_reg for instr: '{}'", instr))
            .as_str()
            .to_string();
        let cond = cap.name("cond")
            .expect(&format!("Didn't capture cond for instr: '{}'", instr))
            .as_str();
        let cond_val: isize = cap.name("cond_val")
            .expect(&format!("Didn't capture cond_val for instr: '{}'", instr))
            .as_str()
            .parse()
            .expect(&format!("Unparsable cond_val for instr: '{}'", instr));

        //
        // Evaluate condition
        //

        let cond_reg_val = *self.registers.entry(cond_reg).or_insert(0);
        let cond_passed = match cond {
            ">" => cond_reg_val > cond_val,
            "<" => cond_reg_val < cond_val,
            ">=" => cond_reg_val >= cond_val,
            "<=" => cond_reg_val <= cond_val,
            "==" => cond_reg_val == cond_val,
            "!=" => cond_reg_val != cond_val,
            _ => panic!("Unsupported condition operator {}", cond),
        };

        if cond_passed {
            let curr_val: &mut isize = self.registers.entry(target).or_insert(0);
            match op {
                "inc" => *curr_val += amount,
                "dec" => *curr_val -= amount,
                _ => panic!("Unsupported operator {}", cond),
            }
        }
    }
}

fn read_input() -> Vec<String> {
    read_file("data/day08")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn run1() {
    let mut cpu = Cpu::default();
    let instructions = read_input();

    for instr in instructions {
        cpu.execute(&instr);
    }

    let result = cpu.registers.values().max().unwrap();
    println!("Result: {}", result);
}

pub fn run2() {
    let mut cpu = Cpu::default();
    let instructions = read_input();
    let mut result = isize::min_value();

    for instr in instructions {
        cpu.execute(&instr);
        result = max(result, *cpu.registers.values().max().unwrap());
    }

    println!("Result: {}", result);
}
