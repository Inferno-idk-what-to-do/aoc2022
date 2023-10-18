use std::{str::FromStr, string::ParseError};

use aoc_tools;

#[derive(Debug)]
struct Move {
    count: i32,
    from: i32,
    to: i32,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words_vec = s.split(" ").collect::<Vec<&str>>();

        Ok(Move { count: FromStr::from_str(words_vec[1]).unwrap(), from: FromStr::from_str(words_vec[3]).unwrap(), to: FromStr::from_str(words_vec[5]).unwrap() })
    } 
}

impl Default for Move {
    fn default() -> Self {
        Move { count: 0, from: 0, to: 0 }
    }
}

fn main() {
    let moves = aoc_tools::read_lines::<Move>("data/day5.txt").unwrap();
    println!("{:?}", moves);
}
