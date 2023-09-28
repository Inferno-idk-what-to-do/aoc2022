use std::str::FromStr;
use std::string::ParseError;

use aoc_tools;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    them: Choice,
    me: Choice,
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let them_char = s.chars().nth(0).unwrap();
        let mut them = Choice::Rock;

        match them_char {
            'A' => them = Choice::Rock,
            'B' => them = Choice::Paper,
            'C' => them = Choice::Scissors,
            _ => (),
        }

        let me_char = s.chars().nth(2).unwrap();
        let mut me = Choice::Rock;

        match me_char {
            'X' => me = Choice::Rock,
            'Y' => me = Choice::Paper,
            'Z' => me = Choice::Scissors,
            _ => (),
        }

        return Ok(Self { them, me });
    }
}

impl Default for Round {
    fn default() -> Self {
        Self { them: Choice::Rock, me: Choice::Rock }
    }
}

fn score_round(round: &Round) -> u32 {
    let score = match round.me {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    if round.me == Choice::Rock {
        return match round.them {
            Choice::Paper => score,
            Choice::Rock => score + 3,
            Choice::Scissors => score + 6,
        };
    } else if round.me == Choice::Paper {
        return match round.them {
            Choice::Scissors => score,
            Choice::Paper => score + 3,
            Choice::Rock => score + 6,
        };
    } else {
        return match round.them {
            Choice::Rock => score,
            Choice::Scissors => score + 3,
            Choice::Paper => score + 6,
        };
    }
}

fn convert_data(round: &Round) -> Round {

    // i need to lose to them
    if round.me == Choice::Rock {
        return match round.them {
            Choice::Rock => Round { them: Choice::Rock, me: Choice::Scissors },
            Choice::Paper => Round { them: Choice::Paper, me: Choice::Rock },
            Choice::Scissors => Round { them: Choice::Scissors, me: Choice::Paper },
        };
    // i need to tie them
    } else if round.me == Choice::Paper {
        return match round.them {
            Choice::Rock => Round { them: Choice::Rock, me: Choice::Rock },
            Choice::Paper => Round { them: Choice::Paper, me: Choice::Paper },
            Choice::Scissors => Round { them: Choice::Scissors, me: Choice::Scissors },
        }
    // i need to beat them
    } else {
        return match round.them {
            Choice::Rock => Round { them: Choice::Rock, me: Choice::Paper },
            Choice::Paper => Round { them: Choice::Paper, me: Choice::Scissors },
            Choice::Scissors => Round { them: Choice::Scissors, me: Choice::Rock },
        };
    }

}

fn main() {
    let data: Vec<Round> = aoc_tools::read_lines::<Round>("data/day2.txt").expect("couldn't read file");
    let part1: u32 = data.iter().map(|r| score_round(r)).sum();
    println!("{:?}", part1);

    let part2: u32 = data.iter().map(|r| convert_data(r)).map(|s| score_round(&s)).sum();
    println!("{:?}", part2);
}
