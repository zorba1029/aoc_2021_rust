// advent/day_18.rs - part 1 and part 2
use log::{debug, info};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::Chars;

use crate::advent::day18::tokenizer::Token;
use crate::advent::day18::tree_handler::{parse_tokens, reduction, TreeNode, TreeNodePtr};

use super::tokenizer::tokenize;

fn read_input(filename: &str) -> (Vec<String>, Vec<Vec<Token>>, Vec<Option<TreeNodePtr>>) {
    let file = File::open(filename).expect("Couldn't open input file.");
    let buf = BufReader::new(file);
    let input_lines = buf
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<String>>();
    let tokens_matrix = input_lines
        .iter()
        .map(|line| tokenize(&line))
        .collect::<Vec<Vec<Token>>>();
    let tree_list = tokens_matrix
        .iter()
        .map(|tokens| parse_tokens(tokens, &mut 0))
        .collect::<Vec<Option<TreeNodePtr>>>();
    (input_lines, tokens_matrix, tree_list)
}

fn handle_input(filename: &str) -> (Vec<String>, Vec<Vec<Token>>, Vec<Option<TreeNodePtr>>) {
    let (input_lines, tokens_matrix, tree_list) = read_input(filename);

    for (index, tree) in tree_list.iter().enumerate() {
        info!("[{}] input_lines : {:#?}", index, input_lines[index]);
        info!(" [{}] tree: ---------------\n{:#?}", index, tree);
    }

    (input_lines, tokens_matrix, tree_list)
}
#[warn(dead_code)]
pub fn do_day_18() {
    info!("===============================================");
    info!("--- Day 18: Snailfish, Part One ---, ");
    info!("===============================================");
    let filename = "input/day_18-sample.txt";
    // let filename = "input/day_18-input.txt";
    let (_, _, tree_list) = handle_input(filename);

    // let new_tree = TreeNode::merge(tree_list[0].as_ref().unwrap().clone(), tree_list[1].as_ref().unwrap().clone());
    // info!("new_tree: ---------------\n{:#?}", new_tree);
    // let new_tree1 = TreeNode::merge_option(tree_list[0].clone(), tree_list[1].clone());
    // info!("new_tree1: ---------------\n{:#?}", new_tree1);

    // if let Some(root) = tree_list[0].as_ref() {
    //     root.borrow().left_order_traversal(&mut |value| {
    //         info!("{}, ", value);
    //     });
    // }

    // if let Some(root) = tree_list[0].as_ref() {
    //     root.borrow().left_order_depth_first(&mut |value| {
    //         info!("{}, ", value);
    //     });
    // }

    let c = reduction(
        tree_list[0].as_ref().unwrap().clone(),
        tree_list[1].as_ref().unwrap().clone(),
    );
    info!("c: ---------------\n{:#?}", c);

    // let mut index = 0;
    // if let Some(tree) = parse_tokens(&tokens, &mut index) {
    //     println!("{:#?}", tree);
    // } else {
    //     println!("Failed to parse tokens into a tree.");
    // }
}
