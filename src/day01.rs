use utils::read_file;

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
    let input: Vec<char> = read_file("data/day01").trim().chars().collect();
    println!("Result: {}", captcha(input, 1));
}

pub fn run2() {
    let input: Vec<char> = read_file("data/day01").trim().chars().collect();
    let offset = input.len() / 2;
    println!("Result: {}", captcha(input, offset));
}
