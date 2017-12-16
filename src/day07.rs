use std::collections::HashMap;
use regex::Regex;

use Day;
use utils::read_file;

#[derive(Debug)]
struct Program {
    name: String,
    weigth: usize,
    children: Vec<String>,
}

impl Program {
    fn parse(input: &str) -> Program {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?P<name>\w+)\s+\((?P<weigth>\d+)\)\s*(?:-> (?P<children>[\w\s,]+))?"
            ).expect("Unparsable regex");
        }

        let cap = RE.captures(input).expect(&format!(
            "Input didn't match regex: '{}'",
            input
        ));

        let name = cap.name("name")
            .expect(&format!("Didn't capture name for input: '{}'", input))
            .as_str()
            .to_string();
        let weigth = cap.name("weigth")
            .expect(&format!("Didn't capture weigth for input: '{}'", input))
            .as_str()
            .parse()
            .expect(&format!("Unparsable weigth for input: '{}'", input));

        let children = match cap.name("children") {
            Some(m) => {
                m.as_str()
                    .split(",")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect()
            }
            None => Vec::new(),
        };

        Program {
            name: name,
            weigth: weigth,
            children: children,
        }
    }
}

struct Tower {
    programs: HashMap<String, Program>,
}

impl Tower {
    fn root(&self) -> &Program {
        let mut curr = self.programs.values().next().expect(
            "At least one program is required",
        );

        while let Some(p) = self.programs.values().find(
            |p| p.children.contains(&curr.name),
        )
        {
            curr = p;
        }

        curr
    }

    fn unbalanced_child(&self, p: &Program) -> Option<(&Program, isize)> {
        if p.children.is_empty() {
            return None;
        }

        let children = self.children(p);
        let mut children_weigths = HashMap::new();

        for mut child in children.iter() {
            let weigth = self.weigth(&mut child);
            let count: &mut usize = children_weigths.entry(weigth).or_insert(0);
            *count += 1;
        }

        if children_weigths.len() > 1 {
            let expected_weigth = children_weigths
                .iter()
                .find(|&(_, count)| *count > 1)
                .map(|(weight, _)| weight)
                .unwrap();
            let unbalanced_child = children_weigths
                .iter()
                .find(|&(_, count)| *count == 1)
                .map(|(weigth, _)| {
                    children.iter().find(|c| self.weigth(c) == *weigth).unwrap()
                })
                .unwrap();
            let diff = *expected_weigth as isize - self.weigth(unbalanced_child) as isize;
            return Some((
                unbalanced_child,
                (unbalanced_child.weigth as isize + diff),
            ));
        } else {
            None
        }
    }

    fn weigth(&self, p: &Program) -> usize {
        p.weigth +
            self.children(p)
                .iter()
                .map(|mut c| self.weigth(&mut c))
                .sum::<usize>()
    }

    fn children(&self, p: &Program) -> Vec<&Program> {
        let mut children: Vec<&Program> = Vec::new();

        for child in &p.children {
            if let Some(child) = self.programs.get(child) {
                children.push(child);
            }
        }

        children
    }
}

fn read_input() -> Tower {
    Tower {
        programs: read_file("data/day07")
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(Program::parse)
            .map(|p| (p.name.clone(), p))
            .collect(),
    }
}

pub struct Day07 {}

impl Day<String, isize> for Day07 {
    fn run1() -> String {

        let tower = read_input();
        let root = tower.root();
        root.name.to_string()
    }

    fn run2() -> isize {
        let tower = read_input();
        let mut curr = tower.root();
        let mut diff = 0;

        while let Some((p, d)) = tower.unbalanced_child(curr) {
            curr = p;
            diff = d;
        }

        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day07::run1(), "airlri");
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day07::run2(), 1206);
    }
}
