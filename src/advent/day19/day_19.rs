#[allow(unused_imports)]
use log::{debug, info};
use std::fs::File;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use std::str::Chars;

#[allow(dead_code)]
fn handle_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Couldn't open input file.");
    let buf = BufReader::new(file);
    let input_lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    
    // let tokens_matrix = input_lines.iter()
    //     .map(|line| tokenize(line))
    //     .collect::<Vec<Vec<Token>>>();

    // let tree_list = tokens_matrix.iter()
    //     .map(|tokens| parse_tokens(tokens, &mut 0))
    //     .collect::<Vec<Option<TreeNodePtr>>>();

    // for (index, line) in input_lines.iter().enumerate() {
    //     debug!("[{}] input_lines : {:#?}", index, line);
    // }
    // for (index, tree) in tree_list.iter().enumerate() {
    //     debug!("[{}] tree : {:#?}", index, tree);
    // }

    // (input_lines, tokens_matrix, tree_list)
    input_lines
}

#[warn(dead_code)]
pub fn do_day_19() {
    do_day_19_part1();
    do_day_19_part2();
}

pub fn do_day_19_part1() {
    info!("===================================================");
    info!("--- Day 19: Beacon Scanner, Part One - Dec 04, 2025");
    info!("===================================================");
}

pub fn do_day_19_part2() {
    info!("===================================================");
    info!("--- Day 19: Beacon Scanner, Part Two - Dec 04, 2025");
    info!("===================================================");
}