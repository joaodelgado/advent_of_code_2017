use utils::read_file;

fn read_input() -> Vec<isize> {
    read_file("data/day05")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect(&format!("Unparsable integer {}", s)))
        .collect()
}

fn run<F>(increment: F) -> usize
where
    F: Fn(isize) -> isize,
{
    let mut instructions = read_input();
    let mut curr_offset = 0isize;
    let mut iter = 0;

    loop {
        let next_offset;
        match instructions.get(curr_offset as usize) {
            Some(incr) => next_offset = curr_offset + incr,
            None => break,
        }
        instructions[curr_offset as usize] = increment(instructions[curr_offset as usize]);
        curr_offset = next_offset;
        iter += 1;
    }

    iter
}

pub fn run1() {
    let result = run(|n| n + 1);
    println!("Result: {}", result);
}

pub fn run2() {
    let result = run(|n| if n >= 3 { n - 1 } else { n + 1 });
    println!("Result: {}", result);
}
