// advent/day_18a.rs - part 1
#[allow(unused_imports)]
use log::{debug, info};
#[allow(unused_imports)]
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use std::rc::Rc;
#[allow(unused_imports)]
use std::str::Chars;

use crate::advent::day18a::tree_handler::{parse_tokens, TreeNode, TreeNodePtr, tree_to_list};
use crate::advent::day18a::tokenizer::{tokenize, Token};

fn handle_input(filename: &str) -> (Vec<String>, Vec<Vec<Token>>, Vec<Option<TreeNodePtr>>) {
    let file = File::open(filename).expect("Couldn't open input file.");
    let buf = BufReader::new(file);
    let input_lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    for (index, line) in input_lines.iter().enumerate() {
        info!("[{}] input_lines : {:#?}", index, line);
    }

    let tokens_matrix = input_lines.iter()
        .map(|line| tokenize(line))
        .collect::<Vec<Vec<Token>>>();

    let tree_list = tokens_matrix.iter()
        .map(|tokens| parse_tokens(tokens, &mut 0))
        .collect::<Vec<Option<TreeNodePtr>>>();

    for (index, tree) in tree_list.iter().enumerate() {
        debug!("[{}] tree : {:#?}", index, tree);
    }

    (input_lines, tokens_matrix, tree_list)
}

#[warn(dead_code)]
pub fn do_day_18a() {
    do_day_18a_part1();

}

pub fn do_day_18a_part1() {
    info!("===============================================");
    info!("--- Day 18: Snailfish, Part One ---, Nov 30, 2025 ");
    info!("===============================================");
    // let filename = "input/day_18-sample-1.txt";
    // let filename = "input/day_18-sample-2.txt";
    let filename = "input/day_18-input.txt";
    let (_input_lines, _tokens_matrix, tree_list) = handle_input(filename);

    // Add all numbers sequentially: first + second, then + third, etc.
    if tree_list.is_empty() {
        info!("No trees to process");
        return;
    }

    let mut result = tree_list[0].as_ref().unwrap().clone();
    debug!("tree[0] : {:#?}", result);
    info!("The 1st tree[0]: {:#?}", tree_to_list(&result));

    for i in 1..tree_list.len() {
        info!("Adding tree[{}]...", i);
        debug!("tree[{}] : {:#?}", i, tree_list[i].as_ref().unwrap());
        info!("    Left  [{}]: {:#?}", i, tree_to_list(&result));
        info!("    Right [{}]: {:#?}", i, tree_to_list(tree_list[i].as_ref().unwrap()));
        result = TreeNode::add(result, tree_list[i].as_ref().unwrap().clone());
        info!("    --> Result [{}]: {:#?}", i, tree_to_list(&result));
    }
    // info!("Final result: ---------------\n{:#?}", result);
    info!("Final result as list: {}", tree_to_list(&result));

    // Calculate and print magnitude
    let magnitude = result.borrow().magnitude();
    info!("Magnitude of final result: {}", magnitude);
}

pub fn do_day_18a_part2() {
    info!("===============================================");
    info!("--- Day 18: Snailfish, Part Two ---, ");
    info!("===============================================");
    // let filename = "input/day_18-sample-1.txt";
    // let filename = "input/day_18-sample-2.txt";
    let filename = "input/day_18-input.txt";
    let (_input_lines, _tokens_matrix, _tree_list) = handle_input(filename);

    
}