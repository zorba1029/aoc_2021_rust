// advent/day_18.rs - part 1 and part 2
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

use crate::advent::day18::tokenizer::Token;
use crate::advent::day18::tree_handler::{tree_to_list, parse_tokens, TreeNode, TreeNodePtr};

use super::tokenizer::tokenize;

fn read_input(filename: &str) -> (Vec<String>, Vec<Vec<Token>>, Vec<Option<TreeNodePtr>>) {
    let file = File::open(filename).expect("Couldn't open input file.");
    let buf = BufReader::new(file);
    let input_lines = buf
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<String>>();
    for (index, line) in input_lines.iter().enumerate() {
        info!("[{}] input_lines : {:#?}", index, line);
    }

    let tokens_matrix = input_lines
        .iter()
        .map(|line| tokenize(line))
        .collect::<Vec<Vec<Token>>>();
    let tree_list = tokens_matrix
        .iter()
        .map(|tokens| parse_tokens(tokens, &mut 0))
        .collect::<Vec<Option<TreeNodePtr>>>();
    (input_lines, tokens_matrix, tree_list)
}

fn handle_input(filename: &str) -> (Vec<String>, Vec<Vec<Token>>, Vec<Option<TreeNodePtr>>) {
    let (input_lines, tokens_matrix, tree_list) = read_input(filename);

    // for (index, tree) in tree_list.iter().enumerate() {
    //     info!("[{}] input_lines : {:#?}", index, input_lines[index]);
    //     debug!("[{}] tree: ---------------\n{:#?}", index, tree);
    // }

    (input_lines, tokens_matrix, tree_list)
}

#[allow(dead_code)]
fn print_tree(node: &TreeNode, depth: usize) {
    let indent = "  ".repeat(depth);
    
    if let Some(value) = node.value {
        debug!("{}Leaf: {}", indent, value);
    } else {
        debug!("{}Non-Leaf:", indent);
        if let Some(ref left) = node.left_child {
            print_tree(&left.borrow(), depth + 1);
        }
        if let Some(ref right) = node.right_child {
            print_tree(&right.borrow(), depth + 1);
        }
    }
}

#[warn(dead_code)]
pub fn do_day_18() {
    info!("===============================================");
    info!("--- Day 18: Snailfish, Part One ---, ");
    info!("===============================================");
    let filename = "input/day_18-sample-1.txt";
    // let filename = "input/day_18-sample-2.txt";
    // let filename = "input/day_18-input.txt";
    let (_, _, tree_list) = handle_input(filename);

    // Add all numbers sequentially: first + second, then + third, etc.
    if tree_list.is_empty() {
        info!("No trees to process");
        return;
    }

    let mut result = tree_list[0].as_ref().unwrap().clone();
    // debug!("tree[0] : {:#?}", result);
    // debug!("tree[0] : ---------------");
    // print_tree(&result.borrow(), 0);

    for i in 1..tree_list.len() {
        // info!("Adding tree[{}]...", i);
        // debug!("tree[{}] : {:#?}", i, tree_list[i].as_ref().unwrap());
        // debug!("tree[{}] : ---------------", i);
        // print_tree(&tree_list[i].as_ref().unwrap().borrow(), 0);
        result = TreeNode::add(result, tree_list[i].as_ref().unwrap().clone());
    }
    // info!("Final result: ---------------\n{:#?}", result);

    // Calculate and print magnitude
    let magnitude = result.borrow().magnitude();
    info!("Magnitude of final result: {}", magnitude);

    info!("Final result as list: {}", tree_to_list(&result));
}
