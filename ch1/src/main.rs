use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn count(input: &Vec<i64>) -> u64 {
    let mut count = 0;
    let mut last: Option<i64> = None;
    for depth in input {
        match last {
            Some(value) => {
                if depth > value.borrow() {
                    count = count + 1;
                }
            }
            None => {}
        }
        last = Some(depth.clone());
    }

    count
}

fn count_sliding(input: &Vec<i64>) -> u64 {
    let mut count = 0;
    let mut last: Option<i64> = None;

    let results = input.windows(3);
    for i in results {
        let sum = i[0] + i[1] + i[2];
        match last {
            Some(value) => {
                if sum > value {
                    count = count + 1;
                }
            }
            None => {}
        }
        last = Some(sum.clone());
    }

    count
}


fn load(filename: &str) -> Vec<i64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(
            |l|
            i64::from_str(l.unwrap().as_str()).unwrap()
        )
        .collect()
}

fn main() {
    let input = load("input.txt");
    // let output = count(input.borrow());
    let output = count_sliding(input.borrow());
    println!("Output: {}", output);
}
