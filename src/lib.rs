use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::string::ParseError;

pub fn prin() {
    println!("it works!")
}

pub fn read_blocks<T: FromStr + Default>(filename: &str) -> Result<Vec<Vec<T>>, ParseError> {
    let file = File::open(filename).expect("couldn't open file");
    let reader = BufReader::new(file);
    let mut whole: Vec<Vec<T>> = Vec::from(Vec::new());
    let mut current: Vec<T> = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            if &l == "" {
                whole.push(current);
                current = vec![];
            } else {
                current.push(l.clone().parse::<T>().unwrap_or_default());
            }
        } else {
            return Ok(vec![vec![]]);
        }
    }

    return Ok(whole);
}

pub fn read_lines<T: FromStr + Default>(filename: &str) -> Result<Vec<T>, ParseError> {
    let file = File::open(filename).expect("couldn't open file");
    let reader = BufReader::new(file);

    Ok(reader.lines()
        .map(|l| l.expect("couldn't read line").parse::<T>().unwrap_or_default())
        .collect::<Vec<T>>())
}
