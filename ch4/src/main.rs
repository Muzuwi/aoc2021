use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::BitOr;
use std::str::FromStr;
use bloom::{ASMS, BloomFilter};

struct Board {
    data: [u64; 25],
    filter: bloom::BloomFilter,
    marked: u64
}

impl Board {
    pub fn from_arr(arr: [u64;25]) -> Board {
        let mut board = Board {
            data: arr,
            filter: BloomFilter::with_rate(0.01, 100),
            marked: 0
        };
        for i in 0..25 {
            board.filter.insert(board.data[i].borrow());
        }

        board
    }

    pub fn contains(&self, number: &u64) -> bool {
        if !self.filter.contains(number.borrow()) {
            return false;
        }

        if !self.data.contains(number.borrow()) {
            return false;
        }

        return true;
    }

    pub fn check(&self) -> bool {
        let vertical = 0x108421u64;
        for i in 0..5 {
            if self.marked & (vertical<<i) == (vertical<<i) {
                return true;
            }
        }
        let horizontal = 0x1F;
        for i in 0..5 {
            if self.marked & (horizontal << i*5) == (horizontal << i*5) {
                return true;
            }
        }
        return false;
    }

    pub fn mark(&mut self, number: &u64) -> bool {
        if !self.contains(number) {
            return false;
        }

        let index = self.data.iter().position(|&v| v == number.clone()).unwrap();
        self.marked |= 1 << index;

        return self.check();
    }

    pub fn sum_unmarked(&self) -> u64 {
        let mut sum: u64 = 0;

        for i in 0..25 {
            if self.marked & (1 << i) != 0 {
                continue;
            }

            sum += self.data[i];
        }

        sum
    }
}

fn load(filename: &str) -> (Vec<u64>, Vec<Board>) {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect();

    let inputs: Vec<u64> = lines[0]
        .split(',')
        .map(
            |s| u64::from_str(s).unwrap()
        )
        .collect();

    let rows: Vec<Vec<u64>> = lines
        .iter()
        .skip(1)
        .filter(|v| !v.trim().is_empty())
        .map(|x| {
            let numbers = x.split_ascii_whitespace();
            let row: Vec<u64> = numbers
                .map(|n| {
                    u64::from_str(n.trim()).unwrap()
                })
                .collect();

            row
        })
        .collect();

    assert_eq!(rows.len() % 5, 0);
    let mut boards: Vec<Board> = vec![];
    for i in 0..(rows.len() / 5) {
        let mut arr: [u64; 25] = [0; 25];

        for x in 0..5 {
            for y in 0..5 {
                arr[y*5 + x] = rows[i*5 + y][x];
            }
        }

        boards.push(Board::from_arr(arr));
    }

    (inputs, boards)
}

fn run_part1(input: &mut (Vec<u64>, Vec<Board>)) {
    let mut winners: Vec<u64> = vec![];

    for number in &input.0 {
        for (x, board) in &mut input.1.iter_mut().enumerate() {
            if !board.contains(number) {
                continue;
            }

            if board.mark(number) {
                if !winners.contains(&((x + 1) as u64)) {
                    let unmarked = board.sum_unmarked();
                    println!("Board {} won with number {}!", x+1, number);
                    println!("Unmarked sum: {}", unmarked);
                    println!("Score: {}", unmarked * number);
                    winners.push((x+1) as u64);
                }
            }
        }
    }


}

fn main() {
    let mut data = load("input.txt");
    run_part1(data.borrow_mut());
}
