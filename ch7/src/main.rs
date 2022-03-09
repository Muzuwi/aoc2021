use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn load(filename: &str) -> Vec<i64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    lines[0]
        .split(',')
        .map(|f| {
            i64::from_str(f).unwrap()
        })
        .collect()
}

fn run(input: &Vec<i64>) {



}

fn main() {
    let input = load("sample.txt");
    run(input.borrow());
}
