#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Expected at least one argument representing the day")
        .parse::<isize>()
        .expect("This first argument must be a number representing the day");

    let part = env::args()
        .nth(2)
        .unwrap_or("1".to_string())
        .parse::<isize>()
        .expect(
            "This second argument must be a number representing the part of the puzzle",
        );

    match format!("{}{}", day, part).as_ref() {
        "11" => day01::run1(),
        "12" => day01::run2(),
        "21" => day02::run1(),
        "22" => day02::run2(),
        "31" => day03::run1(),
        "32" => day03::run2(),
        "41" => day04::run1(),
        "42" => day04::run2(),
        "51" => day05::run1(),
        "52" => day05::run2(),
        "61" => day06::run1(),
        "62" => day06::run2(),
        "71" => day07::run1(),
        "72" => day07::run2(),
        "81" => day08::run1(),
        "82" => day08::run2(),
        _ => panic!("Unsupported day {}", day),
    }
}
