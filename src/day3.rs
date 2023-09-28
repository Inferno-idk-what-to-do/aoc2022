use std::collections::HashSet;

use aoc_tools;

fn common_between_halves(line: String) -> char {
    let chars = line.as_bytes();
    let mid = line.len() / 2;
    let mut first_half = HashSet::new();

    for idx in 0..mid {
        first_half.insert(&chars[idx]);
    }

    for idx in mid..line.len() {
        let got = first_half.get(&chars[idx]);
        if let Some(_) = got {
            return **got.unwrap() as char;
        }
    }

    return ' ';
}

fn common_between_lines(lines: Vec<String>) -> char {
    let mut prev: HashSet<char> = HashSet::new();
    let mut current: HashSet<char> = HashSet::new();

    // init with first line 
    for c in lines[0].chars() {
        prev.insert(c.clone());
    }

    for line_idx in 1..(lines.len()-1) {
        let line = &lines[line_idx];
        for c in line.chars() {
            if let Some(_) = prev.get(&c) {
                current.insert(c.clone());
            }
        }

        prev = current;
        current = HashSet::new();
    }

    for c in lines[lines.len()-1].chars() {
        if let Some(_) = prev.get(&c) {
            return c;
        } 
    }

    return ' ';
}

fn value_of_char(c: char) -> u8 {
    let ascii = c as u8;

    return match ascii {
        65..=90 => ascii - 38,
        _ => ascii - 96,
    };
}

fn chunks(lines: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
    let mut ret: Vec<Vec<String>> = Vec::new();
    for chunk in lines.chunks(chunk_size) {
        ret.push(chunk.to_vec());
    }

    ret
}

fn main() {
    let data = aoc_tools::read_lines::<String>("data/day3.txt").expect("couldn't read file");
    let part1 = data
                    .iter()
                    .map(|l| common_between_halves(l.to_string()))
                    .fold(0, |acc, c| {
                        acc + u32::from(value_of_char(c))
                    });
    println!("{}", part1);

    let part2_chunks = chunks(data, 3);
    let part2 = part2_chunks.iter()
                            .map(|chunk| common_between_lines(chunk.to_vec()))
                            .map(|c| u32::from(value_of_char(c)))
                            .sum::<u32>();
    println!("{}", part2);
}
