use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone)]
struct State {
    energy: Vec<i64>,
    flashed: Vec<bool>,
    queue: VecDeque<usize>,
    width: usize
}

impl State {
    fn flash_at(&mut self, idx: usize) {
        let y = idx / self.width;
        let x = idx % self.width;
        let height = self.energy.len() / self.width;

        self.flashed[idx] = true;
        for yoff in -1..=1 {
            for xoff in -1..=1 {
                let y = (y as i64) + yoff;
                let x = (x as i64) + xoff;
                if x < 0 || y < 0 || x >= self.width as i64 || y >= height as i64 {
                    continue;
                }

                let idx = (y * (self.width as i64) + x) as usize;
                self.energy[idx] += 1;
                if self.energy[idx] > 9 && !self.flashed[idx] && !self.queue.contains(&idx) {
                    // println!("Adding {}", idx);
                    self.queue.push_back(idx);
                }
            }
        }
    }

    fn clear_flashed(&mut self) {
        for i in 0..self.energy.len() {
            if self.flashed[i] {
                self.energy[i] = 0;
                self.flashed[i] = false;
            }
        }
    }

    fn step(&mut self) -> usize {
        for i in 0..self.energy.len() {
            self.energy[i] += 1;
        }

        for i in 0..self.energy.len() {
            if self.energy[i] > 9 {
                // println!("Initial: adding {}", i);
                self.queue.push_back(i);
            }
        }

        let mut flash_count = 0;
        loop {
            if self.queue.is_empty() {
                self.clear_flashed();
                break;
            }

            let idx = self.queue.pop_front().unwrap();
            // println!("Flashing {}", idx);
            if !self.flashed[idx] {
                flash_count += 1;
                self.flash_at(idx);
            }
        }

        flash_count
    }

    fn pretty_print(&self) {
        for y in 0..(self.energy.len() / self.width) {
            for x in 0..self.width {
                print!("{}", self.energy[y * self.width + x]);
            }
            println!();
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

fn run(input: &(Vec<i64>, usize)) {
    let mut state = State {
        energy: input.0.clone(),
        queue: VecDeque::new(),
        flashed: vec![false; input.0.len()],
        width: input.1.clone()
    };

    let mut total = 0;
    for i in 0..100 {
        println!(" ===== Step {} ===== ", i+1);
        let flashes = state.step();
        total += flashes;
        println!("Flashed {} times (total: {})", flashes, total);
        state.pretty_print();
    }
}

fn run_until_synchronized(input: &(Vec<i64>, usize)) {
    let mut state = State {
        energy: input.0.clone(),
        queue: VecDeque::new(),
        flashed: vec![false; input.0.len()],
        width: input.1.clone()
    };

    let mut step = 1;
    loop {
        let flashes = state.step();
        if flashes == input.0.len() {
            println!("Found synchronized flashing in step {}", step);
            state.pretty_print();
            break;
        }
        step += 1;
    }
}

fn main() {
    let input = load("input.txt");
    // run(input.borrow());
    run_until_synchronized(input.borrow());
}
