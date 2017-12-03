use utils::read_file;

macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    }
}

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    }
}

fn read_input() -> Vec<Vec<u64>> {
    read_file("data/day02")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split('\t')
                .map(|n| n.parse().expect("Unparsable number"))
                .collect()
        })
        .collect()
}

pub fn run1() {

    let input = read_input();

    let result: u64 = input
        .into_iter()
        .map(|line| {
            line.into_iter().fold(
                (u64::max_value(), u64::min_value()),
                |(min, max), x| (min!(min, x), max!(max, x)),
            )
        })
        .map(|(min, max)| max - min)
        .sum();

    println!("Result: {}", result);

}

pub fn run2() {

    let input = read_input();

    let result: u64 = input
        .into_iter()
        .map(|line| {
            for (i, a) in line.clone().into_iter().enumerate() {
                for &b in line[i + 1..].into_iter() {
                    let max = max!(a, b);
                    let min = min!(a, b);
                    if max % min == 0 {
                        return max / min;
                    }
                }
            }
            return 0;
        })
        .sum();

    println!("Result: {}", result);

}
