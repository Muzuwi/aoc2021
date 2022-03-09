#![feature(iter_intersperse)]
use std::borrow::Borrow;
use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Inserter {
    polymer: String,
    patterns: Vec<(String, String)>,
    counts: HashMap<String, usize>,
    element_counts: HashMap<char, usize>
}

impl Inserter {
    pub fn from_input(input: &(String, Vec<(String, String)>)) -> Inserter {
        let chars: Vec<char> = input.0.chars().collect();
        let mut counts: HashMap<String, usize> = HashMap::new();

        for polymer in chars.windows(2) {
            let str = String::from_iter(polymer);
            if counts.contains_key(&str) {
                let v = counts.get_mut(&str).unwrap();
                *v += 1;
                continue;
            }
            counts.insert(str, 1);
        }

        let mut element_counts: HashMap<char, usize> = HashMap::new();
        for ch in chars {
            let v = element_counts.get_mut(&ch);
            if v.is_none() {
                element_counts.insert(ch, 1);
            } else {
                *v.unwrap() += 1;
            }
        }

        Inserter {
            polymer: input.0.clone(),
            patterns: input.1.clone(),
            counts: counts,
            element_counts: element_counts
        }
    }

    fn find_pattern(&self, pair: &str) -> Option<&(String, String)> {
        assert_eq!(pair.len(), 2);

        self.patterns
            .iter()
            .find(|item| item.0 == pair)
    }

    /*
    fn step(&mut self) {
        let chars: Vec<char> = self.polymer.chars().collect();
        let mut polymers: Vec<String> = chars
            .windows(2)
            .map(|f| String::from_iter(f.iter()))
            .collect();
        let mut new_polymer: String = String::with_capacity(self.polymer.len());

        for (idx, pair) in &mut polymers.iter_mut().enumerate() {
            let pattern = self.find_pattern(pair);
            if pattern.is_none() {
                new_polymer += pair;
                continue;
            }

            let pattern = pattern.unwrap();
            // println!("{}, {} matches pattern {},{}", idx, pair, pattern.0, pattern.1);

            if idx == 0 {
                new_polymer.write_char(pair.chars().nth(0).unwrap()).unwrap();
            }
            new_polymer.write_char(pattern.1.chars().nth(0).unwrap()).unwrap();
            new_polymer.write_char(pair.chars().nth(1).unwrap()).unwrap();
        }

        self.polymer = new_polymer;
    }
     */

    fn increment_char(&mut self, ch: char, n: usize) {
        let v = self.element_counts.get_mut(&ch);
        if v.is_none() {
            self.element_counts.insert(ch, n);
            return;
        }
        let v = v.unwrap();
        *v += n;
    }

    fn step_optimized(&mut self) {
        let current = self.counts.clone();
        for (k, v) in current.iter() {
            let pattern = self.find_pattern(k.as_str());
            if pattern.is_none() {
                continue;
            }
            let pattern = pattern.unwrap();

            let ch: Vec<char> = k
                .chars()
                .intersperse(pattern.1.chars().nth(0).unwrap())
                .collect();

            assert_eq!(ch.len(), 3);
            let left = String::from_iter(ch[0..=1].iter());
            let right = String::from_iter(ch[1..=2].iter());

            // dbg!(<String as Borrow<str>>::borrow(left.borrow()));
            // dbg!(<String as Borrow<str>>::borrow(right.borrow()));

            self.increment_char(pattern.1.chars().nth(0).unwrap(), v.clone());

            let center_key = self.counts.get_mut(k.as_str()).unwrap();
            *center_key -= v;
            if *center_key == 0 {
                self.counts.remove(k);
            }
            // dbg!(self.counts.borrow());

            let left_key = self.counts.get_mut(left.as_str());
            if left_key.is_none() {
                // println!("Creating left key '{}'", left);
                self.counts.insert(left, *v);
            } else {
                // println!("Updating left key '{}'", left);
                *left_key.unwrap() += *v;
            }
            // dbg!(self.counts.borrow());

            let right_key = self.counts.get_mut(right.as_str());
            if right_key.is_none() {
                // println!("Creating right key '{}'", right);
                self.counts.insert(right, *v);
            } else {
                // println!("Updating right key '{}'", right);
                *right_key.unwrap() += *v;
            }
            // dbg!(self.counts.borrow());

            // let v = self.element_counts.get(pattern.1.chars().nth(0).unwrap());
        }
    }

    /*
    pub fn count_occurences(&self) -> HashMap<char, usize> {
        let mut map: HashMap<char, usize> = HashMap::new();

        for char in self.polymer.chars() {
            if map.contains_key(&char) {
                let mut v = map.get_mut(&char).unwrap();
                *v += 1;
                continue;
            }

            map.insert(char, 1);
        }

        map
    }
     */
}

fn load(filename: &str) -> (String, Vec<(String, String)>) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    let start = lines[0].clone();
    let mut patterns: Vec<(String, String)> = vec![];

    for line in &lines[1..] {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<String> = line
            .split("->")
            .map(|f| f.to_string())
            .collect();
        assert_eq!(parts.len(), 2);

        let pattern = parts[0].clone().trim().to_string();
        let replacement = parts[1].clone().trim().to_string();

        patterns.push((pattern, replacement));
    }

    (start, patterns)
}

fn main() {
    let input = load("input.txt");
    let mut inserter = Inserter::from_input(input.borrow());
    dbg!(inserter.borrow());

    for i in 0..40 {
        inserter.step_optimized();
    }
    dbg!(inserter.counts.borrow());
    dbg!(inserter.element_counts.borrow());

    // let occurences = inserter.count_occurences();
    // for (k, v) in occurences {
    //     println!("Element '{}': {} times", k, v)
    // }
}
