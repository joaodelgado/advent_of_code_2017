use std::fs::File;
use std::io::prelude::*;

fn read_input() -> String {
    let mut input = String::new();
    {
        let mut f = File::open("data/day01").expect("input file not found");
        String::new();
        f.read_to_string(&mut input).expect(
            "something went wrong while reading the input file",
        );
    }

    input
}

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

pub fn run1() {
    let input: Vec<char> = read_input().trim().chars().collect();
    println!("Result: {}", captcha(input, 1));
}

pub fn run2() {
    let input: Vec<char> = read_input().trim().chars().collect();
    let offset = input.len() / 2;
    println!("Result: {}", captcha(input, offset));
}
