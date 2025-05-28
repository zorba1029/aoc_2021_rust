use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[warn(dead_code)]
pub fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Cannot open file");
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}
