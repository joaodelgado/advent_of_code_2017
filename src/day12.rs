use std::collections::{HashSet, HashMap};
use regex::Regex;
use utils::read_file;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    id: usize,
    connections: Vec<usize>,
}

impl Node {
    fn parse(input: &str) -> Node {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?P<id>\d+)\s+<->\s+(?P<connections>[\d,\s]+)"
            ).expect("Unparsable regex");
        }

        let cap = RE.captures(input).expect(&format!(
            "Input didn't match regex: '{}'",
            input
        ));

        let id = cap.name("id")
            .expect(&format!("Didn't capture id for input: '{}'", input))
            .as_str()
            .parse()
            .expect(&format!("Unparsable id for input: '{}'", input));
        let connections = cap.name("connections")
            .expect(&format!(
                "Didn't capture connections for input: '{}'",
                input
            ))
            .as_str()
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.parse().expect(&format!(
                    "Unparsable connection for input: '{}'",
                    input
                ))
            })
            .collect();

        Node {
            id: id,
            connections: connections,
        }
    }
}

struct Graph {
    nodes: HashMap<usize, Node>,
}

impl Graph {
    fn get(&self, id: usize) -> &Node {
        self.nodes.get(&id).expect(&format!(
            "Node with id {} does not exist",
            id
        ))
    }

    fn group(&self, start: usize) -> HashSet<usize> {
        let mut visited = HashSet::new();

        self._group(start, &mut visited);

        visited
    }

    fn _group(&self, curr: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&curr) {
            return;
        }

        visited.insert(curr);

        for &connection in self.get(curr).connections.iter() {
            self._group(connection, visited);
        }
    }
}

fn read_graph() -> Graph {
    Graph {
        nodes: read_file("data/day12")
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(Node::parse)
            .map(|node| (node.id, node))
            .collect(),
    }

}

pub fn run1() {
    let graph = read_graph();
    let result = graph.group(0).len();
    println!("Result: {}", result);
}

pub fn run2() {
    let graph = read_graph();

    let mut result = 0;
    let mut visited = HashSet::new();

    for node in graph.nodes.keys() {
        if visited.contains(node) {
            continue;
        }
        result += 1;
        visited.extend(graph.group(*node));
    }

    println!("Result: {}", result);
}
