use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::{min, max};

#[derive(Debug)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn walk(&self, coord: &Coord) -> Coord {
        match *self {
            Direction::N  => Coord{ x: coord.x,     y: coord.y + 2 },
            Direction::NE => Coord{ x: coord.x + 1, y: coord.y + 1 },
            Direction::E  => Coord{ x: coord.x + 2, y: coord.y },
            Direction::SE => Coord{ x: coord.x + 1, y: coord.y - 1 },
            Direction::S  => Coord{ x: coord.x,     y: coord.y - 2 },
            Direction::SW => Coord{ x: coord.x - 1, y: coord.y - 1 },
            Direction::W  => Coord{ x: coord.x - 2, y: coord.y },
            Direction::NW => Coord{ x: coord.x - 1, y: coord.y + 1 },

        }
    }
}

impl<'a> From<&'a str> for Direction {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn from(s: &'a str) -> Direction {
        match s.to_uppercase().trim() {
            "N"  => Direction::N,
            "NE" => Direction::NE,
            "E"  => Direction::E,
            "SE" => Direction::SE,
            "S"  => Direction::S,
            "SW" => Direction::SW,
            "W"  => Direction::W,
            "NW" => Direction::NW,
            _ => panic!("Unsupported direction: {}", s),
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    /// Manhattan distance from the center
    pub fn dist(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    /// Manhattan distance from the center in a hex grid
    pub fn dist_hex(&self) -> usize {
        let x = self.x.abs() as usize;
        let y = self.y.abs() as usize;
        let closest = min(x, y);
        let furthest = max(x, y);

        closest + (furthest - closest + 1) / 2
    }
    /// Returns list of adjacent coordinates
    pub fn adj(&self) -> Vec<Coord> {
        vec![
            Coord {
                x: self.x - 1,
                y: self.y,
            },
            Coord {
                x: self.x,
                y: self.y - 1,
            },
            Coord {
                x: self.x,
                y: self.y + 1,
            },
            Coord {
                x: self.x + 1,
                y: self.y,
            },
        ]

    }

    /// Returns list of adjacent coordinates (including diagonals)
    pub fn adj_with_diagonals(&self) -> Vec<Coord> {
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

#[derive(PartialEq, Clone)]
pub struct Node<T> {
    pub id: T,
    pub children: Vec<T>,
}

pub struct Graph<K>
where
    K: Eq + Hash,
{
    pub nodes: HashMap<K, Node<K>>,
}

impl<K> Graph<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Graph<K> {
        Graph { nodes: HashMap::new() }
    }

    pub fn from_nodes(nodes: Vec<Node<K>>) -> Graph<K> {
        Graph {
            nodes: nodes
                .iter()
                .map(|node| (node.id.clone(), node.clone()))
                .collect(),
        }
    }

    pub fn extend(&mut self, other: &Graph<K>) {
        self.nodes.extend(other.nodes.clone());
    }

    pub fn insert(&mut self, node: Node<K>) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn contains(&self, id: &K) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn contains_node(&self, node: &Node<K>) -> bool {
        self.contains(&node.id)
    }

    pub fn get<'a>(&'a self, id: &K) -> Option<&'a Node<K>> {
        self.nodes.get(id)
    }

    pub fn groups(&self) -> Vec<Graph<K>> {
        let mut groups: Vec<Graph<K>> = vec![];

        for node in self.nodes.values() {
            if groups.iter().any(|g| g.contains_node(node)) {
                continue;
            } else {
                groups.push(self.group(&node.id));
            }
        }

        groups
    }

    pub fn group(&self, start: &K) -> Graph<K> {
        let mut group = Graph::new();

        let start = self.get(start).expect(
            "Can't get group starting in a non-existing node",
        );
        self._group(start, &mut group);

        group
    }

    fn _group(&self, curr: &Node<K>, group: &mut Graph<K>) {
        if group.contains_node(&curr) {
            return;
        }

        group.insert(curr.clone());

        for child in curr.children.iter().map(|c| self.get(c)) {
            match child {
                Some(c) => self._group(c, group),
                None => (),
            }
        }
    }
}
