use aoc_tools;
use itertools::Itertools;

fn main() {
    let data: Vec<Vec<u32>> = aoc_tools::read_blocks::<u32>("data/day1.txt").expect("read failed");

    let part1 = data.iter()
                    .map(|block| block.iter().sum::<u32>())
                    .max();

    println!("{:?}", part1);

    let part2: u32 = data.iter()
                    .map(|block| block.iter().sum::<u32>())
                    .sorted()
                    .rev()
                    .take(3)
                    .sum();

    println!("{:?}", part2);
}
