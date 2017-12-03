use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn from_number(n: usize) -> Coord {
        let mut x = 0isize;
        let mut y = 0isize;
        let mut dx = 0;
        let mut dy = -1;
        for _ in 1..n {
            if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
                let tmp = dx;
                dx = -dy;
                dy = tmp;
            }
            x += dx;
            y += dy;
        }

        Coord { x: x, y: y }
    }

    fn dist(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn adjacent(&self) -> Vec<Coord> {
        vec![
            Coord {
                x: self.x - 1,
                y: self.y - 1,
            },
            Coord {
                x: self.x - 1,
                y: self.y,
            },
            Coord {
                x: self.x - 1,
                y: self.y + 1,
            },

            Coord {
                x: self.x,
                y: self.y - 1,
            },
            Coord {
                x: self.x,
                y: self.y,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
            },

            Coord {
                x: self.x + 1,
                y: self.y - 1,
            },
            Coord {
                x: self.x + 1,
                y: self.y,
            },
            Coord {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

pub fn run1() {
    let input = 312051;
    let coord = Coord::from_number(input);
    let result = coord.dist();

    println!("Result: {}", result);
}

pub fn run2() {
    let input = 312051;

    let mut matrix: HashMap<Coord, usize> = HashMap::new();

    matrix.insert(Coord { x: 0, y: 0 }, 1);

    let mut n = 2;
    loop {
        let coord = Coord::from_number(n);
        let value: usize = coord
            .adjacent()
            .into_iter()
            .map(|c| match matrix.get(&c) {
                Some(&v) => v,
                None => 0,
            })
            .sum();

        if value > input {
            println!("Result: {}", value);
            break;
        }

        matrix.insert(coord, value);
        n += 1;

    }

}
