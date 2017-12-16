#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fmt::Display;

mod utils;
mod graph;
mod hash;

mod day01;
use day01::Day01;

mod day02;
use day02::Day02;

mod day03;
use day03::Day03;

mod day04;
use day04::Day04;

mod day05;
use day05::Day05;

mod day06;
use day06::Day06;

mod day07;
use day07::Day07;

mod day08;
use day08::Day08;

mod day09;
use day09::Day09;

mod day10;
use day10::Day10;

mod day11;
use day11::Day11;

mod day12;
use day12::Day12;

mod day13;
use day13::Day13;

mod day14;
use day14::Day14;

mod day15;
use day15::Day15;

mod day16;
use day16::Day16;

pub trait Day<T1, T2>
where
    T1: Display,
    T2: Display,
{
    fn run(part: usize) {
        match part {
            1 => println!("Result: {}", Self::run1()),
            2 => println!("Result: {}", Self::run2()),
            _ => panic!("Unsupported part {}", part),
        };
    }

    fn run1() -> T1;

    fn run2() -> T2;
}

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Expected at least one argument representing the day")
        .parse::<usize>()
        .expect("This first argument must be a number representing the day");

    let part = env::args()
        .nth(2)
        .unwrap_or("1".to_string())
        .parse::<usize>()
        .expect(
            "This second argument must be a number representing the part of the puzzle",
        );

    match day {
        01 => Day01::run(part),
        02 => Day02::run(part),
        03 => Day03::run(part),
        04 => Day04::run(part),
        05 => Day05::run(part),
        06 => Day06::run(part),
        07 => Day07::run(part),
        08 => Day08::run(part),
        09 => Day09::run(part),
        10 => Day10::run(part),
        11 => Day11::run(part),
        12 => Day12::run(part),
        13 => Day13::run(part),
        14 => Day14::run(part),
        15 => Day15::run(part),
        16 => Day16::run(part),
        _ => panic!("Unsupported day {}", day),
    };

}
