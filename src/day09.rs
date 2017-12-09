use utils::read_file;

pub fn run1() {
    let input = read_file("data/day09");

    let mut garbage = false;
    let mut escaping = false;
    let mut nest_lvl = 0;
    let mut result = 0;

    for c in input.chars() {

        if escaping {
            escaping = false;
            continue;
        }

        if garbage {
            match c {
                '>' => garbage = false,
                '!' => escaping = true,
                _ => (),
            }
            continue;
        }

        match c {
            '<' => garbage = true,
            '{' => nest_lvl += 1,
            '}' => {
                result += nest_lvl;
                nest_lvl -= 1;
            }
            _ => (),
        }

    }

    println!("Result: {}", result);
}

pub fn run2() {
    let input = read_file("data/day09");

    let mut garbage = false;
    let mut escaping = false;
    let mut result = 0;

    for c in input.chars() {

        if escaping {
            escaping = false;
            continue;
        }

        if garbage {
            match c {
                '>' => garbage = false,
                '!' => escaping = true,
                _ => result += 1,
            }
            continue;
        }

        match c {
            '<' => garbage = true,
            _ => (),
        }

    }

    println!("Result: {}", result);
}
