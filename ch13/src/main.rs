use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Axis {
    X,
    Y
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug, Copy, Clone)]
struct Fold {
    axis: Axis,
    value: i64
}

#[derive(Debug, Clone)]
struct State {
    points: Vec<Point>,
    width: usize,
    height: usize
}

impl State {
    pub fn from_points(points: &Vec<Point>) -> State {
        let width = points
            .iter()
            .max_by_key(|f| {
                f.x
            })
            .unwrap()
            .x as usize + 1;
        let height = points
            .iter()
            .max_by_key(|f| {
                f.y
            })
            .unwrap()
            .y as usize + 1;

        State {
            points: points.clone(),
            width: width,
            height: height
        }
    }

    pub fn pretty_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x: x as i64, y: y as i64 };
                print!("{}", if !self.points.contains(point.borrow()) {
                    '.'
                } else {
                    '#'
                });
            }
            println!();
        }
    }

    fn merge_duplicates(&mut self) {
        let mut vec: Vec<Point> = Vec::with_capacity(self.points.len());
        for point in self.points.iter() {
            if !vec.contains(&point) {
                vec.push(point.clone());
            }
        }
        self.points = vec;
    }

    fn fold_horizontal(&mut self, coord: i64) {
        for mut point in &mut self.points {
            if point.x < coord {
                continue;
            }

            point.x = coord - (point.x - coord);
        }
        self.merge_duplicates();
        self.width = coord as usize;
    }

    fn fold_vertical(&mut self, coord: i64) {
        for mut point in &mut self.points {
            if point.y < coord {
                continue;
            }

            point.y = coord - (point.y - coord);
        }
        self.merge_duplicates();
        self.height = coord as usize;
    }

    pub fn fold(&mut self, axis: Axis, coord: i64) {
        if axis == Axis::X {
            self.fold_horizontal(coord);
        } else {
            self.fold_vertical(coord);
        }
    }

    pub fn count_visible(&self) -> usize {
        self.points.len()
    }
}


fn load(filename: &str) -> (Vec<Point>, Vec<Fold>) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    let mut points: Vec<Point> = vec![];
    let mut folds: Vec<Fold> = vec![];
    for line in lines {
        if line.starts_with("fold") {
            let command = &line[11..];
            let parts: Vec<&str> = command.split('=').collect();
            let axis = match parts[0].chars().nth(0).unwrap() {
                'x' => Axis::X,
                'y' => Axis::Y,
                _ => panic!()
            };
            let value = i64::from_str(parts[1]).unwrap();

            let fold = Fold {
                axis,
                value
            };
            folds.push(fold);

            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        let coords: Vec<i64> = line
            .split(',')
            .map(|f| {
                i64::from_str(f).unwrap()
            })
            .collect();

        let point = Point {
            x: coords[0],
            y: coords[1]
        };

        points.push(point);
    }

    (points, folds)
}

fn run(input: &(Vec<Point>, Vec<Fold>)) {
    let mut state = State::from_points(input.0.borrow());
    println!("Width: {}", state.width);
    println!("Height: {}", state.height);
    // println!("Initial state:");
    // state.pretty_print();

    for fold in input.1.iter() {
        state.fold(fold.axis, fold.value);
        println!("Visible dots: {}", state.count_visible());
    }
    state.pretty_print();
}


fn main() {
    let input = load("input.txt");
    run(input.borrow());
}
