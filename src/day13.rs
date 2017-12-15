use utils::read_file;

#[derive(Clone)]
struct Layer {
    depth: usize,
    range: usize,
}

impl Layer {
    fn new(depth: usize, range: usize) -> Layer {
        Layer {
            depth: depth,
            range: range - 1,
        }
    }

    fn pos_at(&self, moment: usize) -> usize {
        let loops = moment / self.range;

        if loops % 2 == 0 {
            // going forward
            moment % self.range
        } else {
            // going backwards
            self.range - (moment % self.range)
        }
    }
}

// #[derive(Clone)]
struct Firewall {
    layers: Vec<Layer>,
}

impl Firewall {
    fn new(layers: Vec<Layer>) -> Firewall {
        Firewall { layers: layers }
    }

    fn simulate(&mut self, delay: usize) -> (bool, usize) {
        let mut detected = false;
        let mut severity = 0;

        for layer in self.layers.iter() {
            if layer.pos_at(layer.depth + delay) == 0 {
                detected = true;
                severity += (layer.depth * (layer.range + 1)) as usize;
            }
        }

        (detected, severity)
    }
}

fn read_firewall() -> Firewall {
    Firewall::new(
        read_file("data/day13")
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| {
                let parts: Vec<_> = s.split(':').take(2).collect();
                Layer::new(
                    parts[0].trim().parse().expect(&format!(
                        "Unparsable depth: {}",
                        parts[0]
                    )),
                    parts[1].trim().parse().expect(&format!(
                        "Unparsable range: {}",
                        parts[1]
                    )),
                )
            })
            .collect(),
    )
}

pub fn run1() {
    let mut firewall = read_firewall();

    let (_, severity) = firewall.simulate(0);

    println!("Result: {}", severity);
}

pub fn run2() {
    let mut firewall = read_firewall();

    let mut delay = 0;

    println!("Working. This one takes a few seconds");

    loop {
        let (detected, _) = firewall.simulate(delay);
        if !detected {
            break;
        }
        delay += 1;
    }

    println!("Result: {}", delay);
}
