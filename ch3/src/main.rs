use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::BitAnd;

fn load(filename: &str) -> (usize, Vec<u64>) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut size: usize = 0;
    let values = buf.lines()
        .map(
            |l| {
                let line = l.unwrap();

                size = line.len();
                u64::from_str_radix(line.as_str(), 2).unwrap()
            }
        )
        .collect();

    (size, values)
}

fn load_raw(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect()
}

fn find_rates(values: &(usize, Vec<u64>)) -> (u64, u64) {
    let mut counts: Vec<i64> = vec![];
    counts.resize(values.0, 0);

    for v in &values.1 {
        for i in 0..counts.len() {
            let scalar: i64 = if (v & (1 << (counts.len() - i - 1))) != 0 {
                1
            } else {
                -1
            };
            counts[i] += scalar;
        }
    }

    let mut gamma = 0;
    for v in &counts {
        gamma <<= 1;
        if v > &0i64 {
            gamma |= 0b1;
        }
    }

    (gamma, !gamma & ((1 << counts.len()) - 1))
}

fn find_rating(input: &Vec<String>, inverted_logic: bool) -> u64 {
    let ones_filter = if inverted_logic { '0' } else { '1' };
    let zeros_filter = if inverted_logic { '1' } else { '0' };

    let mut current = input.clone();
    let mut position: usize = 0;
    loop {
        if current.len() == 1 {
            return u64::from_str_radix(current[0].borrow(), 2).unwrap();
        }

        let ones = current
            .iter()
            .filter(|x| x.chars().nth(position).unwrap() == '1')
            .count();
        let zeroes = current.len() - ones;

        let filter = if ones >= zeroes { ones_filter } else { zeros_filter };
        let new_values: Vec<String> = current
            .into_iter()
            .filter(|x| x.chars().nth(position).unwrap() == filter)
            .collect();
        current = new_values;
        position += 1;
    }
}

fn main() {
    // let value = load("input.txt");
    // let results = calc(value.borrow());
    // println!("Gamma: {}", results.0);
    // println!("Epsilon: {}", results.1);
    // println!("Out: {}", results.0 * results.1);

    let input = load_raw("input.txt");
    let generator = find_rating(input.borrow(), false);
    let scrubber = find_rating(input.borrow(), true);
    println!("Generator rating: {}", generator);
    println!("Scrubber rating: {}", scrubber);
    println!("Life support rating: {}", generator * scrubber);
}
