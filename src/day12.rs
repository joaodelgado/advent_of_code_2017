use regex::Regex;

use Day;
use utils::read_file;
use graph::{Graph, Node};

impl<'a> From<&'a str> for Node<usize> {
    fn from(input: &str) -> Node<usize> {
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
            children: connections,
        }
    }
}

fn read_graph() -> Graph<usize> {
    Graph::from_nodes(
        read_file("data/day12")
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(Node::from)
            .collect(),
    )

}

pub struct Day12 {}

impl Day<usize, usize> for Day12 {
    fn run1() -> usize {
        let graph = read_graph();
        let result = graph.group(&0).nodes.keys().len();

        result
    }

    fn run2() -> usize {
        let graph = read_graph();

        let mut result = 0;
        let mut visited = Graph::new();

        for node in graph.nodes.keys() {
            if visited.contains(node) {
                continue;
            }
            result += 1;
            visited.extend(&graph.group(node));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        assert_eq!(Day12::run1(), 378);
    }

    #[test]
    fn test_run2() {
        assert_eq!(Day12::run2(), 204);
    }
}
