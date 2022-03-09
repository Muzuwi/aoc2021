use std::borrow::{Borrow, BorrowMut};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn load(filename: &str) -> Vec<i8> {
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
            i8::from_str(f).unwrap()
        })
        .collect()
}

fn run_naive(input: &mut Vec<i8>, days: usize) {
    for i in 0..days {
        let count = input.len();
        for j in 0..count {
            input[j] -= 1;

            if input[j] < 0 {
                input[j] = 6;
                input.push(8);
            }
        }
    }
}

fn run_boxed(input: &Vec<i8>, days: usize) -> i64 {
    //  Each element in the deque represents the amount of fish
    //  with a timer value equal to the index, i.e.
    //  counts[0] = count of fish with timer=0
    let mut counts: VecDeque<i64> = VecDeque::new();
    counts.resize(9, 0);
    for time in input {
        counts[time.clone() as usize] += 1;
    }

    for _ in 0..days {
        //  "Rotate" the counts deque to the left
        //  Also remember the first element, which gets added to
        //  the counts for timer value 6
        let born = counts.pop_front().unwrap();
        counts.push_back(born);
        counts[6] += born;
    }

    let mut sum = 0;
    for fish in counts {
        sum += fish;
    }

    sum
}


fn main() {
    let mut input = load("input.txt");
    let output = run_boxed(input.borrow(), 256);
    println!("Fish count: {}", output);
}
