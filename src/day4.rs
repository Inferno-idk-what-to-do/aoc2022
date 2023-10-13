use std::{ops::RangeInclusive, str::FromStr};
use std::string::ParseError;
use std::collections::HashSet;

use aoc_tools;

#[derive(Debug)]
struct Pair {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assignments = s
            .split(',')
            .map(|a| 
                a
                    .split('-')
                    .map(|b| 
                        b
                            .parse::<u32>()
                            .unwrap())
                    .collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();
        return Ok(Pair { first: (assignments[0][0]..=assignments[0][1]), second: (assignments[1][0]..=assignments[1][1]) });
    }
}

impl Default for Pair {
    fn default() -> Self {
        return Pair { first: (0..=0), second: (0..=0) };
    }
}

impl Pair {
    fn first_contains_second(&self) -> bool {
        let mut fst = HashSet::new();

        let f_range = self.first.clone();
        let s_range = self.second.clone();

        for i in f_range {
            fst.insert(i);
        }
        let fst_len = fst.len();

        for y in s_range {
            fst.insert(y);
        }

        return fst_len == fst.len()
    }

    fn overlaps(&self) -> bool {
        let f = self.first.clone().into_inner();
        let s = self.second.clone().into_inner();

        if f.0 < s.0 {
            return f.1 >= s.0;
        } else if s.0 < f.0 {
            return s.1 >= f.0;
        } else {
            return true;
        }
    }

    fn reverse(&self) -> Pair {
        let f = self.first.clone();
        let s = self.second.clone();
        return Pair { first: s, second: f };
    }
}

fn main() {
    let data = aoc_tools::read_lines::<Pair>("data/day4.txt").expect("couldn't read file");

    let day1 = data.iter()
        .map(|p| {
            p.first_contains_second() || p.reverse().first_contains_second()
        })
        .filter(|b| *b)
        .count();

    println!("{:?}", day1);

    let day2 = data.iter()
        .map(|p| { p.overlaps() })
        .filter(|b| *b)
        .count();

    println!("{:?}", day2);
}
