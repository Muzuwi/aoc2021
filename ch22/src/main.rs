use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn from_i64(from: i64, to: i64) -> Range {
        Range {
            start: from,
            end: to,
        }
    }
}

#[derive(Debug)]
struct Input {
    state: bool,
    x: Range,
    y: Range,
    z: Range,
}

struct State {
    cube: [[[bool; 100 + 1]; 100 + 1]; 100 + 1]
}

impl State {
    pub fn new() -> State {
        State {
            cube: [[[false; 100 + 1]; 100 + 1]; 100 + 1]
        }
    }

    pub fn mark(&mut self, input: Input) {
        let xmin = i64::max(input.x.start, -50);
        let xmax = i64::min(input.x.end, 50);
        let ymin = i64::max(input.y.start, -50);
        let ymax = i64::min(input.y.end, 50);
        let zmin = i64::max(input.z.start, -50);
        let zmax = i64::min(input.z.end, 50);

        for x in xmin..=xmax {
            for y in ymin..=ymax {
                for z in zmin..=zmax {
                    self.cube[(50 + x) as usize][(50 + y) as usize][(50 + z) as usize] = input.state;
                }
            }
        }

    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        for x in -50..=50 {
            for y in -50..=50 {
                for z in -50..=50 {
                    if self.cube[(50 + x) as usize][(50 + y) as usize][(50 + z) as usize] == true {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}


fn load(filename: &str) -> Vec<Input> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    let mut output: Vec<Input> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        assert_eq!(parts.len(), 2);

        let state = match parts[0] {
            "on" => true,
            "off" => false,
            _ => panic!()
        };

        let coords: Vec<&str> = parts[1].split(',').collect();
        assert_eq!(coords.len(), 3);

        let x: Vec<i64> = coords[0][2..]
            .split("..")
            .map(|str| {
                i64::from_str(str).unwrap()
            })
            .collect();
        let y: Vec<i64> = coords[1][2..]
            .split("..")
            .map(|str| {
                i64::from_str(str).unwrap()
            })
            .collect();
        let z: Vec<i64> = coords[2][2..]
            .split("..")
            .map(|str| {
                i64::from_str(str).unwrap()
            })
            .collect();

        output.push(Input {
            x: Range { start: x[0], end: x[1] },
            y: Range { start: y[0], end: y[1] },
            z: Range { start: z[0], end: z[1] },
            state: state
        })

    }

    output
}

fn main() {
    let input = load("sample2.txt");
    dbg!(&input);

    let mut state = State::new();
    for line in input {
        state.mark(line);
    }

    let count = state.count();
    println!("Count: {}", count);
}
