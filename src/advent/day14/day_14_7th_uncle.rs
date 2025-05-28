use std::collections::HashMap;
use std::time::Instant;

use crate::utils::read_file;

// ex: NNCB
type PairList = HashMap<(char, char), usize>;
// ex: CH -> B
type InsertionRules = HashMap<(char, char), char>;

pub fn day14() {
    let now = Instant::now();
    // let lines = read_file("input/day_14-sample-a.txt");
    let lines = read_file("input/day_14-input.txt");
    let template: Vec<char> = lines[0].chars().collect();
    let first_char = template.first().unwrap();
    let last_char = template.last().unwrap();
    let mut pair_list: PairList = HashMap::new();

    //-- init: polymer template
    // count the # of chars from inupt string
    template.windows(2).for_each(|items| {
        *pair_list.entry((items[0], items[1])).or_default() += 1;
    });

    //-- init: insertion rules (XY -> Z)
    let mut insertion_rules: InsertionRules = HashMap::new();
    lines.iter().skip(2).enumerate().for_each(|(_index, line)| {
        // println!("  {}: {}", index, line);
        if let Some((left, right)) = line.split_once("->") {
            let mut left_chars = left.trim().chars();
            let right_char = right.trim().chars().next().unwrap();
            let l1 = left_chars.next().unwrap();
            let l2 = left_chars.next().unwrap();
            println!("      {}{} -> {}", l1, l2, right_char);
            insertion_rules.insert((l1, l2), right_char);
        }
    });
    // println!("insertion rules: {:?}", insertion_rules);
    // insertion rules that matched with initial input string (NNCB)
    // ex) for NNCB,
    //   [0] ('N', 'N') -> C, count = 1
    //   [1] ('N', 'C') -> B, count = 1
    //   [2] ('C', 'B') -> H, count = 1
    // for (j, (tuple, count)) in pair_list.iter().enumerate() {
    //     let rhs = insertion_rules.get(&tuple).unwrap();
    //     println!("  [{}] {:?} -> {}, count = {}", j, tuple, rhs, count);
    // }

    pair_list = run_loop(&pair_list, &insertion_rules, 10);
    let counts_first = get_counts_first_chars(&pair_list, *first_char, *last_char);
    let counts_last = get_counts_last_chars(&pair_list, *first_char, *last_char);
    println!(
        "[Day 14] Part 1 (step=10): {} (={})",
        counts_first, counts_last
    );

    pair_list = run_loop(&pair_list, &insertion_rules, 30);
    let counts_first = get_counts_first_chars(&pair_list, *first_char, *last_char);
    let counts_last = get_counts_last_chars(&pair_list, *first_char, *last_char);
    println!(
        "[Day 14] Part 2 (step=40): {} (={})",
        counts_first, counts_last
    );

    let elapsed = now.elapsed();
    println!(
        "-- Complete time: {:6.2}ms\n",
        elapsed.as_micros() as f64 / 1000.
    );
}

fn run_loop(pair_list: &PairList, insertion_rules: &InsertionRules, loops: usize) -> PairList {
    let mut pair_list = pair_list.clone();

    for i in 0..loops {
        let mut new_pair_list = pair_list.clone();
        for (j, (tuple, count)) in pair_list.iter().enumerate() {
            let rhs = insertion_rules.get(tuple).unwrap();
            println!("  [{},{}] {:?} -> {}, count = {}", i, j, tuple, rhs, count);
            *new_pair_list.get_mut(tuple).unwrap() -= count;
            *new_pair_list.entry((tuple.0, *rhs)).or_default() += count;
            *new_pair_list.entry((*rhs, tuple.1)).or_default() += count;
            //-- debug print
            let n_count = new_pair_list.get(&tuple).unwrap();
            println!(
                "      [{},{}] *({}, {}) -> {}, n_count = {}",
                i, j, tuple.0, tuple.1, n_count, n_count
            );
            println!(
                "      [{},{}]  ({}, {}) -> {}, count = {}",
                i, j, tuple.0, rhs, count, count
            );
            println!(
                "      [{},{}]  ({}, {}) -> {}, count = {}",
                i, j, rhs, tuple.1, count, count
            );
        }

        for (j, (tuple, count)) in new_pair_list.iter().enumerate() {
            // let rhs = insertion_rules.get(&tuple).unwrap();
            println!("    NEW [{}] {:?} ->, count = {}", j, tuple, count);
        }
        pair_list = new_pair_list;
    }

    pair_list
}

fn get_counts_last_chars(pair_list: &PairList, first_char: char, last_char: char) -> usize {
    println!("--- Count (LAST chars) ---");
    let mut count: HashMap<char, usize> = HashMap::new();
    *count.entry(first_char).or_default() += 1;
    pair_list.iter().for_each(|(key, val)| {
        *count.entry(key.1).or_default() += val;
    });

    let min = count.values().min().unwrap();
    let max = count.values().max().unwrap();
    println!("      max: ({})", max);
    println!("      min: ({})", min);
    println!("  [max - min]: ({})", max - min);
    // max - min

    let (max_key, max_value) = count.iter().max_by_key(|entry| entry.1).unwrap();
    let (min_key, min_value) = count.iter().min_by_key(|entry| entry.1).unwrap();

    println!("      max: {} ({})", max_key, max_value);
    println!("      min: {} ({})", min_key, min_value);
    println!("  [max_value - min_value]: ({})", max_value - min_value);

    max_value - min_value
}

fn get_counts_first_chars(pair_list: &PairList, _first_char: char, last_char: char) -> usize {
    println!("--- Count (FIRST chars) ---");
    let mut count: HashMap<char, usize> = HashMap::new();
    *count.entry(last_char).or_default() += 1;
    pair_list.iter().for_each(|(key, val)| {
        *count.entry(key.0).or_default() += val;
    });

    let min = count.values().min().unwrap();
    let max = count.values().max().unwrap();
    println!("  max: ({})", max);
    println!("  min: ({})", min);
    println!("  [max - min]: ({})", max - min);
    // max - min

    let (max_key, max_value) = count.iter().max_by_key(|entry| entry.1).unwrap();
    let (min_key, min_value) = count.iter().min_by_key(|entry| entry.1).unwrap();

    println!("  max: {} ({})", max_key, max_value);
    println!("  min: {} ({})", min_key, min_value);
    println!("  [max_value - min_value]: ({})", max_value - min_value);

    max_value - min_value
}
