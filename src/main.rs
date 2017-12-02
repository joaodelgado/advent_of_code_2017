use std::env;

mod day01;
mod day02;

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
        _ => panic!("Unsupported day {}", day),
    }
}
