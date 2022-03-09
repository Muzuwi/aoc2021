use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone)]
struct Position {
    depth: i64,
    horizontal: i64
}

fn load(filename: &str) -> Vec<(String, i64)> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(
            |l| {
                let line = l.unwrap();
                let substr: Vec<&str> = line.splitn(2, ' ').collect();

                (substr[0].to_string(), i64::from_str(substr[1]).unwrap())
            }
        )
        .collect()
}

fn run(input: &Vec<(String, i64)>) -> Position {
    let mut horizontal = 0i64;
    let mut depth = 0i64;
    let mut aim = 0i64;

    for cmd in input {
        match cmd.0.as_str() {
            "forward" => {
                horizontal += cmd.1;
                depth += aim * cmd.1;
            }

            "up" => {
                aim -= cmd.1;
                // depth -= cmd.1;
            }

            "down" => {
                aim += cmd.1;
                // depth += cmd.1;
            }

            _ => {
                panic!();
            }
        }
    }

    Position {
        depth,
        horizontal
    }
}

fn main() {
    let input = load("input.txt");
    let output = run(input.borrow());

    println!("Horizontal: {}", output.horizontal);
    println!("Depth: {}", output.depth);
}
