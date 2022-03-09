use std::borrow::{Borrow, BorrowMut};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn load(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(
            |l| l.unwrap()
        )
        .collect()
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Scope {
    Brace,
    Square,
    Squiggly,
    Triangle
}

#[derive(Eq, PartialEq)]
enum Token {
    OpenBlock(Scope),
    CloseBlock(Scope),
}

enum ParseError {
    InvalidChar(usize),
    Unexpected(usize, char),
    EOF
}


struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            tokens: VecDeque::new()
        }
    }

    fn autocomplete(&self) -> String {
        let mut output: String = String::with_capacity(self.tokens.len());
        for token in self.tokens.iter().rev() {
            match token {
                Token::OpenBlock(scope) => {
                    let char = match scope {
                        Scope::Brace => ')',
                        Scope::Square => ']',
                        Scope::Squiggly => '}',
                        Scope::Triangle => '>',
                    };
                    output.push(char);
                }
                _ => panic!()
            }
        }

        output
    }

    fn parse(&mut self, line: &String) -> Option<ParseError> {
        for (idx,ch) in line.chars().enumerate() {
            let scope = match ch {
                '('|')' => Some(Scope::Brace),
                '['|']' => Some(Scope::Square),
                '{'|'}' => Some(Scope::Squiggly),
                '<'|'>' => Some(Scope::Triangle),
                _ => None,
            };
            if scope.is_none() {
                return Some(ParseError::InvalidChar(idx));
            }
            let scope = scope.unwrap();


            let token = match ch {
                '('|'['|'{'|'<' => Some(Token::OpenBlock(scope)),
                ')'|']'|'}'|'>' => Some(Token::CloseBlock(scope)),
                _ => None
            };
            if token.is_none() {
                return Some(ParseError::InvalidChar(idx));
            }
            let token = token.unwrap();


            match token {
                Token::OpenBlock(scope) => {
                    self.tokens.push_back(token);
                }

                Token::CloseBlock(scope) => {
                    let current = self.tokens.pop_back();
                    if current.is_none() {
                        return Some(ParseError::Unexpected(idx, ch));
                    }

                    let current = current.unwrap();
                    if current != Token::OpenBlock(scope) {
                        return Some(ParseError::Unexpected(idx, ch));
                    }
                }
            }
        }

        if !self.tokens.is_empty() {
            return Some(ParseError::EOF);
        }

        return None;
    }
}

fn autocomplete_score(str: &String) -> usize {
    let mut score = 0;
    for ch in str.chars() {
        score *= 5;
        score += match ch {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        };
    }

    score
}


fn run(input: &Vec<String>) {
    let mut score = 0;
    let mut autocompletion_scores: Vec<usize> = vec![];

    for (idx,line) in input.iter().enumerate() {
        let mut parser = Parser::new();

        let result = parser.parse(line.borrow());
        if result.is_none() {
            println!("Line {}: OK", idx);
            continue;
        }

        let result = result.unwrap();
        match result {
            ParseError::InvalidChar(char_idx) => {
                println!("Line {}: syntax error at character {}: invalid character", idx, char_idx);
            }
            ParseError::Unexpected(char_idx, ch) => {
                println!("Line {}: syntax error at character {}: unexpected character '{}'", idx, char_idx, ch);
                match ch {
                    ')' => {
                        score += 3
                    }
                    ']' => {
                        score += 57
                    }
                    '}' => {
                        score += 1197
                    }
                    '>' => {
                        score += 25137
                    }
                    _ => {}
                }
            }
            ParseError::EOF => {
                println!("Line {}: unexpected EOF", idx);
                let autocompleted = parser.autocomplete();
                let score = autocomplete_score(autocompleted.borrow());
                println!("Autocomplete result: '{}' [score: {}]", autocompleted, score);
                autocompletion_scores.push(score);
            }
        }
    }

    println!("Final parse score: {}", score);

    autocompletion_scores.sort_unstable();
    assert_eq!(autocompletion_scores.len() % 2, 1);
    println!("Autocompletion median score: {}", autocompletion_scores[autocompletion_scores.len() / 2]);
}

fn main() {
    let input = load("input.txt");
    run(input.borrow());
}
