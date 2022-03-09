use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

enum Segment {
    A = 0x01,
    B = 0x02,
    C = 0x04,
    D = 0x08,
    E = 0x10,
    F = 0x20,
    G = 0x40,
}

fn load(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    lines
}

fn pattern_to_byte(pattern: &str) -> u8 {
    let mut byte: u8 = 0x0;

    for ch in pattern.chars() {
        let segtype = match ch {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => {
                panic!();
            }
        };

        byte |= segtype as u8;
    }

    byte
}

fn run_p1(input: &Vec<String>) {
    let mut count = 0;
    for line in input {
        let parts = line.split_once('|').unwrap();
        let patterns: Vec<&str> = parts.0
            .split_whitespace()
            .collect();
        let outputs: Vec<&str> = parts.1
            .split_whitespace()
            .collect();

        assert_eq!(patterns.len(), 10);
        assert_eq!(outputs.len(), 4);

        // dbg!(patterns.borrow());
        dbg!(<Vec<&str> as Borrow<[&str]>>::borrow(outputs.borrow()));

        let unique = outputs
            .iter()
            .filter(|p| {
                match p.len() {
                    2 => true,
                    4 => true,
                    3 => true,
                    7 => true,
                    _ => false
                }
            })
            .count();
        count += unique;
    }

    println!("Unique count: {}", count);
}

fn run_p2(input: &Vec<String>) {
    let mut sum = 0;

    for line in input {
        let parts = line.split_once('|').unwrap();
        let patterns: Vec<u8> = parts.0
            .split_whitespace()
            .map(pattern_to_byte)
            .collect();
        let outputs: Vec<u8> = parts.1
            .split_whitespace()
            .map(pattern_to_byte)
            .collect();

        assert_eq!(patterns.len(), 10);
        assert_eq!(outputs.len(), 4);

        // dbg!(patterns);
        // dbg!(outputs);

        let one_pattern: u8 = patterns
            .iter()
            .find(|p| p.count_ones() == 2)
            .unwrap()
            .clone();
        let four_pattern: u8 = patterns
            .iter()
            .find(|p| p.count_ones() == 4)
            .unwrap()
            .clone();
        let seven_pattern: u8 = patterns
            .iter()
            .find(|p| p.count_ones() == 3)
            .unwrap()
            .clone();
        let eight_pattern: u8 = patterns
            .iter()
            .find(|p| p.count_ones() == 7)
            .unwrap()
            .clone();

        println!("Pattern[1]: {:07b}", one_pattern);
        println!("Pattern[4]: {:07b}", four_pattern);
        println!("Pattern[7]: {:07b}", seven_pattern);
        println!("Pattern[8]: {:07b}", eight_pattern);

        let seg_a = seven_pattern & !one_pattern;
        assert_eq!(seg_a.count_ones(), 1);
        // let seg_bd = one_pattern ^ four_pattern;
        let seg_bd = four_pattern & !one_pattern;
        assert_eq!(seg_bd.count_ones(), 2);

        println!("Segment[a]: {:07b}", seg_a);
        println!("Segment[bd]: {:07b}", seg_bd);

        let nine_pattern = patterns
            .iter()
            .find(|p| {
                p.count_ones() == 6 &&
                    (*p ^ (four_pattern | seg_a)).count_ones() == 1
            })
            .unwrap()
            .clone();

        println!("Pattern[9]: {:07b}", nine_pattern);

        let seg_g = (four_pattern | seg_a) ^ nine_pattern;
        assert_eq!(seg_g.count_ones(), 1);
        println!("Segment[g]: {:07b}", seg_g);

        let seg_e = (nine_pattern ^ eight_pattern);
        assert_eq!(seg_e.count_ones(), 1);
        println!("Segment[e]: {:07b}", seg_e);

        let three_pattern = patterns
            .iter()
            .find(|p| {
                p.count_ones() == 5 && ((*p & one_pattern) == one_pattern)
            })
            .unwrap()
            .clone();
        println!("Pattern[3]: {:07b}", three_pattern);

        let seg_d = three_pattern & seg_bd;
        let seg_b = seg_bd & !seg_d;
        assert_eq!(seg_b.count_ones(), 1);
        assert_eq!(seg_d.count_ones(), 1);
        println!("Segment[b]: {:07b}", seg_b);
        println!("Segment[d]: {:07b}", seg_d);

        let six_pattern = patterns
            .iter()
            .find(|p| {
                p.count_ones() == 6 &&
                    ((*p & one_pattern) != one_pattern)
            })
            .unwrap()
            .clone();
        println!("Pattern[6]: {:07b}", six_pattern);

        let seg_f = six_pattern ^ (seg_a | seg_b | seg_d | seg_e | seg_g);
        assert_eq!(seg_f.count_ones(), 1);
        println!("Segment[f]: {:07b}", seg_f);

        let seg_c = one_pattern ^ seg_f;
        assert_eq!(seg_c.count_ones(), 1);
        println!("Segment[c]: {:07b}", seg_c);

        let zero_pattern = seg_a | seg_b | seg_c | seg_e | seg_f | seg_g;
        let two_pattern = seg_a | seg_c | seg_d | seg_e | seg_g;
        let five_pattern = seg_a | seg_b | seg_d | seg_f | seg_g;
        println!("Pattern[0]: {:07b}", zero_pattern);
        println!("Pattern[2]: {:07b}", two_pattern);
        println!("Pattern[5]: {:07b}", five_pattern);

        assert_eq!(zero_pattern.count_ones(), 6);
        assert_eq!(one_pattern.count_ones(), 2);
        assert_eq!(two_pattern.count_ones(), 5);
        assert_eq!(three_pattern.count_ones(), 5);
        assert_eq!(four_pattern.count_ones(), 4);
        assert_eq!(five_pattern.count_ones(), 5);
        assert_eq!(six_pattern.count_ones(), 6);
        assert_eq!(seven_pattern.count_ones(), 3);
        assert_eq!(eight_pattern.count_ones(), 7);
        assert_eq!(nine_pattern.count_ones(), 6);

        let mut number = 0;
        for o in outputs {
            println!("{:07b}", o);
            //  I know, this is horrible
            let digit = if o == zero_pattern {
                0
            } else if o == one_pattern {
                1
            } else if o == two_pattern {
                2
            } else if o == three_pattern {
                3
            } else if o == four_pattern {
                4
            } else if o == five_pattern {
                5
            } else if o == six_pattern {
                6
            } else if o == seven_pattern {
                7
            } else if o == eight_pattern {
                8
            } else if o == nine_pattern {
                9
            } else {
                panic!("everything is fucked");
            };

            number = number*10 + digit;
        }

        println!("=========> output number: {}", number);

        sum += number;

        // dbg!(patterns.borrow());
        // dbg!(<Vec<&str> as Borrow<[&str]>>::borrow(outputs.borrow()));
    }

    println!("Total sum: {}", sum);
}

fn main() {
    let input = load("input.txt");
    run_p2(input.borrow());
}
