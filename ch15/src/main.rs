use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::fmt::Binary;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    risk: i64,
    visited: bool,
    edges: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Map {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Edge {
    score: i64,
    target: usize
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
            .then_with(|| self.target.cmp(&other.target))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    pub fn from_input(input: &(Vec<i64>, usize)) -> Map {
        let scores: &Vec<i64> = input.0.borrow();
        let width = input.1;
        let height = (scores.len() / width);
        let real_width = width * 5;
        let real_height = height * 5;

        let mut tiles: Vec<Node> = Vec::with_capacity(real_width * real_height);

        for m in 0..5 as usize {
            for y in 0..height {
                for n in 0..5 as usize {
                    for x in 0..width {
                        let idx = y * width + x;
                        let risk = 1 + (((scores[idx] - 1) as usize + m + n) % 9);

                        let real_x = n * width + x;
                        let real_y = m * height + y;

                        let mut edges: Vec<usize> = vec![];
                        if real_y > 0 {
                            edges.push((real_y - 1) * real_width + real_x);
                        }
                        if real_y < real_height - 1 {
                            edges.push((real_y + 1) * real_width + real_x);
                        }
                        if real_x > 0 {
                            edges.push(real_y * real_width + real_x - 1);
                        }
                        if real_x < real_width - 1 {
                            edges.push(real_y * real_width + real_x + 1);
                        }

                        let node = Node {
                            risk: risk as i64,
                            visited: false,
                            edges,
                        };
                        tiles.push(node);
                    }
                }
            }
        }

        Map {
            nodes: tiles,
            width: real_width,
            height: real_height,
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.nodes[y*self.width + x].borrow();
                print!("{}", v.risk);
            }
            println!();
        }
    }

    fn run(&mut self) {
        let mut shortest: Vec<i64> = vec![i64::MAX; self.nodes.len()];
        let mut queue: BinaryHeap<Edge> = BinaryHeap::new();

        shortest[0] = 0;
        queue.push(Edge{ score: 0, target: 0});

        loop {
            if queue.is_empty() {
                println!("No path found.");
                break;
            }

            let current = queue.pop().unwrap();

            if current.target == self.nodes.len()-1 {
                println!("Path found. Score: {}", current.score);
                break;
            }

            if current.score > shortest[current.target] {
                continue;
            }

            let node = &self.nodes[current.target];
            for edge in &node.edges {
                let other = &self.nodes[*edge];
                let cost_to_node = current.score + other.risk;

                if cost_to_node < shortest[*edge] {
                    queue.push(Edge { score: cost_to_node, target: *edge });
                    shortest[*edge] = cost_to_node;
                }
            }
        }
    }
}


fn load(filename: &str) -> (Vec<i64>, usize) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    let mut vec: Vec<i64> = vec![];
    for str in &lines {
        for ch in str.chars() {
            vec.push(i64::from_str(ch.to_string().as_str()).unwrap());
        }
    }
    let size = lines[0].len();

    (vec, size)
}

fn main() {
    let input = load("input.txt");
    // dbg!(input.borrow());

    let mut map = Map::from_input(input.borrow());
    // dbg!(map.borrow());

    map.print();
    map.run();
}
