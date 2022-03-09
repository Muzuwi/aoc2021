use std::borrow::{Borrow, BorrowMut};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::ExplodeState::{AddValues, Completed};

#[derive(Debug, Clone)]
enum NumberType {
    Literal(i64),
    Number(Box<NumberPair>),
}

#[derive(Debug, Clone)]
struct NumberPair {
    left: NumberType,
    right: NumberType,
}

#[derive(Debug, Clone)]
enum ExplodeState {
    None,
    Completed,
    AddValues(Option<i64>, Option<i64>),
}

impl NumberPair {
    pub fn from_string(str: &String) -> Option<Box<NumberPair>> {
        let mut types: VecDeque<NumberType> = VecDeque::new();

        for ch in str.chars() {
            match ch {
                '0'..='9' => {
                    let literal = ch.to_digit(10)? as i64;
                    types.push_back(NumberType::Literal(literal));
                }
                '[' => {
                    continue;
                }
                ',' => {
                    continue;
                }
                ']' => {
                    let first = types.pop_back()?;
                    let second = types.pop_back()?;

                    let number = NumberType::Number(Box::new(NumberPair {
                        left: second,
                        right: first,
                    }));
                    types.push_back(number);
                }
                _ => {
                    eprintln!("Invalid token '{}'", ch);
                    return None;
                }
            }
        }

        assert_eq!(types.len(), 1);
        let front = types.pop_front().unwrap();

        return match front {
            NumberType::Number(bx) => Some(bx),
            _ => None
        };
    }

    pub fn add(lhs: &Box<NumberPair>, rhs: &Box<NumberPair>) -> Box<NumberPair> {
        let mut number = Box::new(NumberPair {
            left: NumberType::Number(lhs.clone()),
            right: NumberType::Number(rhs.clone()),
        });

        number
    }

    fn find_explodable_pair(&mut self, depth: usize, count: &mut usize) -> Option<(usize, (i64, i64))> {
        match &mut self.left {
            NumberType::Number(branch) => {
                if depth + 1 >= 4 {
                    let values = match (&branch.left, &branch.right) {
                        (NumberType::Literal(left), NumberType::Literal(right)) => (left.clone(), right.clone()),
                        _ => panic!()
                    };

                    return Some((count.clone(), values));
                }

                let v = branch.find_explodable_pair(depth + 1, count.borrow_mut());
                if v.is_some() {
                    return v;
                }
            }
            NumberType::Literal(literal) => {
                *count += 1;
            }
        };
        match &mut self.right {
            NumberType::Number(branch) => {
                if depth + 1 >= 4 {
                    let values = match (&branch.left, &branch.right) {
                        (NumberType::Literal(left), NumberType::Literal(right)) => (left.clone(), right.clone()),
                        _ => panic!()
                    };

                    return Some((count.clone(), values));
                }

                let v = branch.find_explodable_pair(depth + 1, count.borrow_mut());
                if v.is_some() {
                    return v;
                }
            }
            NumberType::Literal(literal) => {
                *count += 1;
            }
        };

        None
    }

    fn add_to_literal(&mut self, depth: usize, count: &mut usize, target: usize, value: i64) {
        match &mut self.left {
            NumberType::Number(branch) => {
                branch.add_to_literal(depth + 1, count.borrow_mut(), target, value);
            }
            NumberType::Literal(literal) => {
                if *count == target {
                    *literal += value;
                }
                *count += 1;
            }
        };
        match &mut self.right {
            NumberType::Number(branch) => {
                branch.add_to_literal(depth + 1, count.borrow_mut(), target, value);
            }
            NumberType::Literal(literal) => {
                if *count == target {
                    *literal += value;
                }
                *count += 1;
            }
        };
    }

    fn explode_pair(&mut self, depth: usize) -> bool {
        match &mut self.left {
            NumberType::Number(branch) => {
                if depth + 1 >= 4 {
                    self.left = NumberType::Literal(0);
                    return true;
                }

                let v = branch.explode_pair(depth + 1);
                if v {
                    return true;
                }
            }
            _ => {}
        }
        match &mut self.right {
            NumberType::Number(branch) => {
                if depth + 1 >= 4 {
                    self.right = NumberType::Literal(0);
                    return true;
                }

                let v = branch.explode_pair(depth + 1);
                if v {
                    return true;
                }
            }
            _ => {}
        }

        return false;
    }

    fn explode(&mut self) -> bool {
        let mut tmp = 0;
        let explodable = self.find_explodable_pair(0, &mut tmp);
        if explodable.is_none() {
            return false;
        }

        let explodable = explodable.unwrap();
        let target = explodable.0;
        let left = explodable.1.0;
        let right = explodable.1.1;

        tmp = 0;
        if target > 0 {
            self.add_to_literal(0, &mut tmp, target - 1, left);
        }
        tmp = 0;
        self.add_to_literal(0, &mut tmp, target + 2, right);
        self.explode_pair(0);
        return true;
    }

    fn split(&mut self) -> bool {
        match &mut self.left {
            NumberType::Number(branch) => {
                if branch.split() {
                    return true;
                }
            }
            NumberType::Literal(literal) => {
                if *literal >= 10 {
                    self.left = NumberType::Number(Box::new(NumberPair{
                        left: NumberType::Literal(*literal / 2),
                        right: NumberType::Literal((*literal as f64 / 2.0f64).ceil() as i64),
                    }));
                    return true;
                }
            }
        }
        match &mut self.right {
            NumberType::Number(branch) => {
                if branch.split() {
                    return true;
                }
            }
            NumberType::Literal(literal) => {
                if *literal >= 10 {
                    self.right = NumberType::Number(Box::new(NumberPair{
                        left: NumberType::Literal(*literal / 2),
                        right: NumberType::Literal((*literal as f64 / 2.0f64).ceil() as i64),
                    }));
                    return true;
                }
            }
        }
        return false;
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn magnitude(&self) -> i64 {
        let left = match &self.left {
            NumberType::Literal(literal) => literal.clone(),
            NumberType::Number(branch) => branch.magnitude()
        };
        let right = match &self.right {
            NumberType::Literal(literal) => literal.clone(),
            NumberType::Number(branch) => branch.magnitude()
        };

        return 3 * left + 2 * right;
    }
}


fn load(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect()
}

fn main() {
    let input = load("input.txt");

    let mut numbers: Vec<Box<NumberPair>> = vec![];
    for line in input {
        let number = NumberPair::from_string(line.borrow());
        // dbg!(number.borrow());
        if number.is_some() {
            numbers.push(number.unwrap());
        }
    }

    // let mut current = numbers[0].clone();
    // for number in numbers.iter().skip(1) {
    //     let mut sum = NumberPair::add(&current, number);
    //     sum.reduce();
    //     current = sum;
    // }
    // println!("Magnitude: {}", current.magnitude());

    let mut max: Option<i64> = None;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            let first = &numbers[i];
            let second = &numbers[j];

            let mut sum = NumberPair::add(first, second);
            sum.reduce();

            let magnitude = sum.magnitude();
            if max.is_none() {
                max = Some(magnitude);
            } else {
                if magnitude > max.unwrap() {
                    max = Some(magnitude);
                }
            }
        }
    }
    println!("Largest magnitude: {}", max.unwrap());
}
