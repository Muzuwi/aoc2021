use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::id;
use std::str::FromStr;

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

fn pretty_print_queue(queue: &VecDeque<usize>, dims: (usize, usize)) {
    for y in 0..dims.1 {
        for x in 0..dims.0 {
            print!("{}", if queue.contains(&(y * dims.0 + x)) {'X'} else {'.'});
        }
        println!();
    }
}

fn find_basin_size(input: &(Vec<i64>, usize), point: usize) -> usize {
    let data: &Vec<i64> = input.0.borrow();
    let width = input.1;
    let height = data.len() / width;

    let mut explored: VecDeque<usize> = VecDeque::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(point);

    loop {
        if queue.is_empty() {
            break;
        }

        let idx = queue.pop_front().unwrap();
        let x = idx % width;
        let y = idx / width;
        let current = data[idx];
        explored.push_back(idx);

        assert_ne!(current, 9);

        // pretty_print_queue(explored.borrow(), (width, height));

        //  Left
        if x > 0 {
            let other_idx = y * width + x - 1;
            let other = data[other_idx];
            let diff = other - current;
            if other != 9 && !explored.contains(&other_idx) {
                if !queue.contains(&other_idx) {
                    queue.push_back(other_idx);
                }
            }
        }
        //  Right
        if x < width - 1 {
            let other_idx = y * width + x + 1;
            let other = data[other_idx];
            let diff = other - current;
            if other != 9 && !explored.contains(&other_idx) {
                if !queue.contains(&other_idx) {
                    queue.push_back(other_idx);
                }
            }
        }
        //  Up
        if y > 0 {
            let other_idx = (y - 1) * width + x;
            let other = data[other_idx];
            let diff = other - current;
            if other != 9 && !explored.contains(&other_idx) {
                if !queue.contains(&other_idx) {
                    queue.push_back(other_idx);
                }
            }
        }
        //  Down
        if y < height - 1 {
            let other_idx = (y + 1) * width + x;
            let other = data[other_idx];
            let diff = other - current;
            if other != 9 && !explored.contains(&other_idx) {
                if !queue.contains(&other_idx) {
                    queue.push_back(other_idx);
                }
            }
        }
    }

    explored.len()
}

fn run_part1(input: &(Vec<i64>, usize)) {
    let data: &Vec<i64> = input.0.borrow();
    let width = input.1;
    let height = data.len() / width;

    let mut risk_sum = 0;
    let mut basins: Vec<usize> = vec![];
    for i in 0..data.len() {
        let x = i % width;
        let y = i / width;
        let current = data[i];

        let mut neighbours_higher = true;
        //  Left
        if x > 0 {
            neighbours_higher &= data[y * width + x - 1] > current;
        }
        //  Right
        if x < width - 1 {
            neighbours_higher &= data[y * width + x + 1] > current;
        }
        //  Up
        if y > 0 {
            neighbours_higher &= data[(y - 1) * width + x] > current;
        }
        //  Down
        if y < height - 1 {
            neighbours_higher &= data[(y + 1) * width + x] > current;
        }

        if neighbours_higher {
            let risk = current + 1;
            risk_sum += risk;
            let basin_size = find_basin_size(input.borrow(), i);
            basins.push(basin_size);

            println!("Low point: {}x{} [risk={}] [basin={}]", x, y, risk, basin_size);
        }
    }

    println!("Total risk: {}", risk_sum);
    basins.sort_unstable();
    let score = basins
        .into_iter()
        .rev()
        .take(3)
        .reduce(|acc, f| {
            acc * f
        })
        .unwrap();
    println!("Score: {}", score);
}

fn main() {
    let input = load("input.txt");
    run_part1(input.borrow());
}
