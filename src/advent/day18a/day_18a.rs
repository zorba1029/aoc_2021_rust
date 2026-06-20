// advent/day_18a.rs - part 1
#[allow(unused_imports)]
use log::{debug, info, trace};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// use crate::advent::day18a::tokenizer::{tokenize, Token};
use super::tokenizer::{tokenize, Token};
use super::tree_handler::{parse_tokens, TreeNode, TreeNodePtr};

fn handle_input(filename: &str) -> Result<(Vec<String>, Vec<Vec<Token>>, Vec<Option<TreeNodePtr>>)> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let input_lines: Result<Vec<String>> = buf.lines().collect();
    let input_lines = input_lines?;
    let input_lines = input_lines
        .into_iter()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<String>>();
    trace!("input_lines = {:#?}", input_lines);

    let tokens_matrix: Vec<Vec<Token>> = input_lines.iter().map(|line| tokenize(line)).collect();

    trace!("tokens_matrix = {:#?}", tokens_matrix);

    let tree_list = tokens_matrix
        .iter()
        .map(|tokens| parse_tokens(tokens, &mut 0))
        .collect::<Vec<Option<TreeNodePtr>>>();

    trace!("tree_list = {:#?}", tree_list);
    Ok((input_lines, tokens_matrix, tree_list))
}

#[warn(dead_code)]
pub fn do_day_18a() {
    do_day_18a_part1();
    // do_day_18a_part2();
}

pub fn do_day_18a_part1() {
    info!("==================================================");
    info!("--- Day 18: Snailfish, Part One ---, Nov 30, 2025 ");
    info!("==================================================");
    let filename = "input/day_18-sample-1.txt";
    // let filename = "input/day_18-sample-2.txt";
    // let filename = "input/day_18-input.txt";
    let (_input_lines, _tokens_matrix, tree_list) = handle_input(filename).expect("Failed to handle input");

    let mut result = tree_list[0].as_ref().unwrap().clone();
    debug!("tree[0] : {}", result.borrow().tree_to_list());

    for i in 1..tree_list.len() {
        // debug!("Adding tree[{}]...", i);
        debug!(
            "tree[{}] : {}",
            i,
            tree_list[i].as_ref().unwrap().borrow().tree_to_list()
        );
        debug!("    Left : {}", result.borrow().tree_to_list());
        debug!("    Right: {}", tree_list[i].as_ref().unwrap().borrow().tree_to_list());
        result = TreeNode::add(result, tree_list[i].as_ref().unwrap().clone());
        // debug!("  --> result : {:#?}", result);
    }
    info!("Final result: {}", result.borrow().tree_to_list());

    let magnitude = result.borrow().magnitude();
    info!("Magnitude of final result: {}", magnitude);
}

// pub fn do_day_18a_part2() {
//     info!("===============================================");
//     info!("--- Day 18: Snailfish, Part Two ---, Dec 01, 2025");
//     info!("===============================================");
//     // let filename = "input/day_18-sample-2.txt";
//     let filename = "input/day_18-input.txt";
//     let (_input_lines, _tokens_matrix, tree_list) = handle_input(filename);

//     // Add all numbers sequentially: first + second, then + third, etc.
//     if tree_list.is_empty() {
//         info!("No trees to process");
//         return;
//     }

//     let mut max_value = 0;
//     let mut max_index = (0, 0);
//     for i in 0..tree_list.len() {
//         for j in 0..tree_list.len() {
//             if i == j {
//                 continue;
//             }
//             // Deep copy 필수! add()가 원본 트리를 수정하므로
//             let sum = TreeNode::add(
//                 clone_tree(tree_list[i].as_ref().unwrap()),
//                 clone_tree(tree_list[j].as_ref().unwrap()),
//             );
//             let magnitude = sum.borrow().magnitude();
//             debug!("Magnitude of sum[{},{}]: {}", i, j, magnitude);
//             max_value = max_value.max(magnitude);
//             if magnitude == max_value {
//                 max_index = (i, j);
//             }
//         }
//     }
//     // let max_magnitude = *mag_list.iter().max().unwrap();
//     info!(
//         "The largest magnitude: {} (max_pair index=[{},{}])",
//         max_value, max_index.0, max_index.1
//     );
//     info!(
//         "The 1st of the max pair[{}] = {}",
//         max_index.0,
//         tree_to_list(tree_list[max_index.0].as_ref().unwrap())
//     );
//     info!(
//         "The 2nd of the max pair[{}] = {}",
//         max_index.1,
//         tree_to_list(tree_list[max_index.1].as_ref().unwrap())
//     );
//     // Magnitude of sum[67,92]: 4763
// }
