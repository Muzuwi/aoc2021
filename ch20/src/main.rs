use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Input {
    algorithm: Vec<bool>,
    image: Vec<bool>,
    width: usize,
    height: usize,
    outside_state: bool
}

impl Input {

    fn get_surround(&self, x: i64, y: i64) -> u64 {
        let mut output: u64 = 0;
        for yoff in -1..=1 {
            for xoff in -1..=1 {
                let x = x + xoff;
                let y = y + yoff;

                output <<= 1;
                let state = if x < 0 || y < 0 {
                    self.outside_state
                } else if x >= self.width as i64 || y >= self.height as i64 {
                    self.outside_state
                } else {
                    self.image[(y as usize) * self.width + x as usize]
                };
                output |= state as u64;
            }
        }

        output
    }

    fn step(&mut self) {
        let mut output: Vec<bool> = vec![];
        output.resize((self.width+2) * (self.height+2), false);

        for y in 0..self.height+2 {
            for x in 0..self.width+2 {
                let actual_x = (x as i64) - 1;
                let actual_y = (y as i64) - 1;
                let score = self.get_surround(actual_x, actual_y);
                output[y * (self.width+2) + x] = self.algorithm[score as usize];
            }
        }
        self.outside_state = if self.outside_state == true {
            self.algorithm[511]
        } else {
            self.algorithm[0]
        };
        self.width += 2;
        self.height += 2;
        self.image = output;
    }

    fn pretty_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.image[y * self.width + x] == true {
                    '#'
                } else {
                    '.'
                });
            }
            println!();
        }
    }

    fn count_lit(&self) -> usize {
        self.image
            .iter()
            .filter(|&v| *v == true)
            .count()
    }

}

fn load(filename: &str) -> Input {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    let algorithm: Vec<bool> = lines[0]
        .chars()
        .map(|ch| {
            if ch == '#' {
                true
            } else {
                false
            }
        })
        .collect();

    let mut image: Vec<bool> = vec![];
    for line in lines.iter().skip(2) {
        for ch in line.chars() {
            image.push(if ch == '#' {
                true
            } else {
                false
            });
        }
    }

    let width = lines[2].len();
    let height = image.len() / width;

    Input {
        algorithm,
        image,
        width,
        height,
        outside_state: false
    }

}

fn main() {
    let mut input = load("input.txt");

    for i in 0..50 {
        input.step();
        input.pretty_print();
        println!("Step {}: pixels lit: {}", i, input.count_lit());
    }
}
