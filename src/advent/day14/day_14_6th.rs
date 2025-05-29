// advent/day_14_4th.rs
use log::{debug, info};
#[allow(unused_imports)]
use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
// use num_format::{Locale, ToFormattedString};

// --- Day 14: Extended Polymerization ---
//
// The incredible pressures at this depth are starting to put a strain on your submarine.
// The submarine has polymerization equipment that would produce suitable materials
// to reinforce the submarine, and the nearby volcanically-active caves should even
// have the necessary input elements in sufficient quantities.
//
// The submarine manual contains instructions for finding the optimal polymer formula;
// specifically, it offers a polymer template and a list of pair insertion rules
// (your puzzle input).
// You just need to work out what polymer would result after repeating
// the pair insertion process a few times.

// --- Day 14: Extended Polymerization ---
//
// The incredible pressures at this depth are starting to put a strain on your submarine.
// The submarine has polymerization equipment that would produce suitable materials
// to reinforce the submarine, and the nearby volcanically-active caves should even
// have the necessary input elements in sufficient quantities.
//
// The submarine manual contains instructions for finding the optimal polymer formula;
// specifically, it offers a polymer template and a list of pair insertion rules
// (your puzzle input).
// You just need to work out what polymer would result after repeating
// the pair insertion process a few times.

//  [ ] input instructions list - (x or y,value) list -------
//   [] input -  polymer_template len: 4
//   [] input -  polymer_template : ['N', 'N', 'C', 'B']
//   [] input -  insertion_rules len: 16
//   -------- Pair Insertion Rules --------------------
//
//   [ 11]: "BB" -> 'N'  ==> BN, NB
//   [ 14]: "BC" -> 'B'  ==> BB, BB
//   [  9]: "BH" -> 'H'  ==> BH, HH
//   [  1]: "BN" -> 'B'  ==> BB, NN

//   [  3]: "CB" -> 'H'  ==> CH, HB
//   [ 13]: "CC" -> 'N'  ==> CN, NC
//   [ 15]: "CH" -> 'B'  ==> CB, BH
//   [  0]: "CN" -> 'C'  ==> CC, CN

//   [  8]: "HB" -> 'C'  ==> HC, CB
//   [ 12]: "HC" -> 'B'  ==> HB, BC
//   [ 10]: "HH" -> 'N'  ==> HN, NH
//   [  2]: "HN" -> 'C'  ==> HC, CN

//   [  5]: "NB" -> 'B'  ==> NB, BB
//   [  4]: "NC" -> 'B'  ==> NB, BC
//   [  7]: "NH" -> 'C'  ==> NC, CH
//   [  6]: "NN" -> 'C'  ==> NC, CN

// input: NNCB,
// (step 0) NN = 1, NC = 1, CB = 1
// (step 1) NN -> NC, CN ||=> NC = 1, CN = 1; NN = 0,
//          NC -> NB, BC ||=> NB = 1, BC = 1; NC = 0,
//          CB -> CH, HB ||=> CH = 1, HB = 1; CB = 0,
// (step 2) NC -> HB, BC ||=> HB = 2, BC = 2; NC = 0,
// ...

//-----------------------------------------------------------------
// Template:     NNCB
// After step 1: NCNBCHB
// After step 2: NBCCNBBBCBHCB
// After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
// After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
//-----------------------------------------------------------------

// input: NNCB -> NN:1, NC:1, CB:1
// (step 1)
// NN becomes NCN -> NC:1, CN:1
// NC becomes NBC -> NB:1, BC:1
// CB becomes CHB -> CH:1, HB:1
// NCNBCHB -> NC:1 CN:1 NB:1 BC:1 CH:1 HB:1
//-- count the 2nd char of each tuple(pair) (+ 1 of the first char from input: N)
// N's = 2
// C's = 2
// B's = 2
// H's = 1 (total: 7)
//--------
// step 2:
// NBCCNBBBCBHCB (total: 13)
// NB:2, BC:2 CC:1 CN:1 BB:2 CB:2 BH:1 HC:1
// N's = 2
// B's = 6
// C's = 4
// H's = 1 (total: 13)
// This was tough one and this was a fun to solve because we had to re-arrange our thinking
// to keep track of pairs instead of just generating the full string.

type PolymerCounterMap = HashMap<(char, char), usize>;
type InsertionRuleMap = HashMap<(char, char), char>;

fn handle_input(filename: &str) -> (PolymerCounterMap, InsertionRuleMap, char, char) {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let input_template = lines[0].chars().collect::<Vec<char>>();
    let first_char = input_template.first().unwrap();
    let last_char = input_template.last().unwrap();
    let lines_count = lines.len();
    info!("[*] Input Filename: {}", filename);
    info!("[*] input lines count = {}", lines_count);

    info!("[ ] First Line: len={}, {}, ", lines[0].len(), lines[0]);

    //------------------------
    // input polymer template: a string with several characters
    // the first line from input file
    info!("[ ] input polymer template -------");
    let mut polymer_counter: PolymerCounterMap = HashMap::new();
    input_template.windows(2).for_each(|items| {
        *polymer_counter.entry((items[0], items[1])).or_default() += 1;
        info!(
            "  [] {}{} => {}",
            items[0],
            items[1],
            polymer_counter.get(&(items[0], items[1])).unwrap()
        );
    });

    //------------------------
    // input instructions list:
    // each input line is like (CH -> B style) and starts from 3rd line in input file.
    // So, we skip the first two lines from input file.
    info!("[ ] input instructions list - \n\t(CH -> B style) list from 3rd line. so skip the first two lines -------");
    let mut insertion_rules: InsertionRuleMap = HashMap::new();
    lines.iter().skip(2).enumerate().for_each(|(_index, line)| {
        if let Some((left, right)) = line.split_once("->") {
            let mut left_chars = left.trim().chars();
            let right_char = right.trim().chars().next().unwrap();
            let left0 = left_chars.next().unwrap();
            let left1 = left_chars.next().unwrap();
            // info!("  [{:3}] {}{} -> {}", index, left0, left1, right);
            insertion_rules.insert((left0, left1), right_char);
        }
    });

    (polymer_counter, insertion_rules, *first_char, *last_char)
}

pub fn do_day_14() {
    day_14_part_one();
}

fn day_14_part_one() {
    let now = Instant::now();
    info!("===============================================");
    info!("--- Day 14: Extended Polymerization,  Part TWO ---, START: 2/5/2022 (Feb,5) ");
    info!("--- DONE: Aug/15/2023 (Aug 15) ----");
    info!("===============================================");
    // let filename = "input/day_14-sample-a.txt";
    let filename = "input/day_14-input.txt";
    let (polymer_counter, insertion_rules, first_char, last_char) = handle_input(filename);
    info!(
        "[] input -  polymer_template len: {}",
        polymer_counter.len()
    );
    info!("[] input -  polymer_template : {:?}", polymer_counter);
    info!("[] input -  insertion_rules len: {}", insertion_rules.len());
    let polymer_counter_1 = polymer_counter.clone();

    display_insertion_rules(&insertion_rules);
    // insertion rules that matched with initial input string (NNCB)
    // ex) for NNCB,
    //    [0] (N,N) -> C, 1 (count)
    //    [1] (C,B) -> H, 1 (count)
    //    [2] (N,C) -> B, 1 (count)
    display_polymer_count_status(&polymer_counter, &insertion_rules, first_char, last_char);

    let step_limit = 10;
    let polymer_counter = run_task(
        &polymer_counter,
        &insertion_rules,
        step_limit,
        first_char,
        last_char,
    );
    let counts_with_first_char = get_counts_with_first_char(&polymer_counter, first_char);
    let counts_with_last_chart = get_counts_with_last_char(&polymer_counter, last_char);
    display_polymer_counter_map(&polymer_counter, step_limit);

    info!("-----------------------------------------");
    info!("Day 14: Extended Polymerization, Part One: DONE (SUCCESS)");
    info!("---- Aug/13/2023 (Aug 13) ----");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!(
        "[Day 14] Part 1 (step={}): {} (={})",
        step_limit, counts_with_first_char, counts_with_last_chart
    );
    info!("-----------------------------------------");

    let step_limit = step_limit + 30;
    let polymer_counter_1 = run_task(
        &polymer_counter_1,
        &insertion_rules,
        step_limit,
        first_char,
        last_char,
    );
    let counts_with_first_char = get_counts_with_first_char(&polymer_counter_1, first_char);
    let counts_with_last_chart = get_counts_with_last_char(&polymer_counter_1, last_char);
    display_polymer_counter_map(&polymer_counter_1, step_limit);

    info!("------------------------------------------");
    info!(
        "[Day 14] Part 1 (step={}): {} (={})",
        step_limit, counts_with_first_char, counts_with_last_chart
    );
    info!("-----------------------------------------");
    let elapsed = now.elapsed();
    info!(
        "[**] Complete time: {:6.2}ms",
        elapsed.as_micros() as f64 / 1000.
    );
    info!("-----------------------------------------");
}

fn run_task(
    polymer_counter: &PolymerCounterMap,
    insertion_rules: &InsertionRuleMap,
    step_limit: u32,
    first_char: char,
    _last_char: char,
) -> PolymerCounterMap {
    let mut polymer_counter = polymer_counter.clone();

    for i in 0..step_limit {
        debug!("[STEP: {}] <<--------------------------", i);
        let mut new_polymer_counter = polymer_counter.clone();

        debug!(" [BEFORE]");
        tmp_counts_with_first_char(&new_polymer_counter, first_char);

        for (j, (tuple, tuple_count)) in polymer_counter.iter().enumerate() {
            let rhs = insertion_rules.get(tuple).unwrap();
            debug!(
                "  input [{}][{}] {:?} --> {}, {} [PREV]",
                i, j, tuple, rhs, tuple_count
            );

            let new_tuple0 = (tuple.0, *rhs);
            let new_tuple1 = (*rhs, tuple.1);
            *new_polymer_counter.get_mut(tuple).unwrap() -= tuple_count;
            *new_polymer_counter.entry(new_tuple0).or_default() += tuple_count;
            *new_polymer_counter.entry(new_tuple1).or_default() += tuple_count;

            //-- debug print
            let new_count = new_polymer_counter.get(&tuple).unwrap();
            let c0 = new_polymer_counter.get(&new_tuple0).unwrap();
            let c1 = new_polymer_counter.get(&new_tuple1).unwrap();
            debug!(
                "        [{},{}] *({}, {}) -> {} [New]",
                i, j, tuple.0, tuple.1, new_count
            );
            debug!(
                "        [{},{}]  ({}, {}) -> {} [New] [{}] ",
                i,
                j,
                tuple.0,
                rhs,
                c0,
                if *tuple == new_tuple0 { '*' } else { ' ' }
            );
            debug!(
                "        [{},{}]  ({}, {}) -> {} [New] [{}] ",
                i,
                j,
                rhs,
                tuple.1,
                c1,
                if *tuple == new_tuple1 { '*' } else { ' ' }
            );
        }

        debug!(" [AFTER]");
        tmp_counts_with_first_char(&new_polymer_counter, first_char);
        display_polymer_counter_map(&new_polymer_counter, i);

        polymer_counter = new_polymer_counter;
    }

    polymer_counter
}

fn get_counts_with_first_char(polymer_counter: &PolymerCounterMap, first_char: char) -> usize {
    info!("[ ] ========= Count chars (+ FIRST char) ======");
    let mut count: HashMap<char, usize> = HashMap::new();

    // count the first input char: +1, i.e: NNCB ==> count(N)+1
    *count.entry(first_char).or_default() += 1;
    polymer_counter.iter().for_each(|(key, val)| {
        *count.entry(key.1).or_default() += val;
    });

    info!("  [1] -- Count for Each Char -------");
    count.iter().enumerate().for_each(|(index, (key, count))| {
        info!("        [{:2}] {} => {}", index, key, count);
    });

    let (max_key, max_value) = count.iter().max_by_key(|entry| entry.1).unwrap();
    let (min_key, min_value) = count.iter().min_by_key(|entry| entry.1).unwrap();

    info!("  [2] -- Max and Min -------");
    info!("        MAX: {} ({})", max_key, max_value);
    info!("        min: {} ({})", min_key, min_value);
    info!("       [max - min]: ({})", max_value - min_value);

    max_value - min_value
}

fn get_counts_with_last_char(polymer_counter: &PolymerCounterMap, last_char: char) -> usize {
    info!("[ ] ========= Count chars (+ LAST char) ======");
    let mut count: HashMap<char, usize> = HashMap::new();

    // count the last input char: +1, i.e: NNCB ==> count(B)+1
    *count.entry(last_char).or_default() += 1;
    polymer_counter.iter().for_each(|(key, val)| {
        *count.entry(key.0).or_default() += val;
    });

    info!("  [1] -- Count for Each Char -------");
    count.iter().enumerate().for_each(|(index, (key, count))| {
        info!("        [{:2}] {} => {}", index, key, count);
    });

    let (max_key, max_value) = count.iter().max_by_key(|entry| entry.1).unwrap();
    let (min_key, min_value) = count.iter().min_by_key(|entry| entry.1).unwrap();

    info!("  [2] -- Max and Min -------");
    info!("        MAX: {} ({})", max_key, max_value);
    info!("        min: {} ({})", min_key, min_value);
    info!("       [max - min]: ({})", max_value - min_value);

    max_value - min_value
}

fn tmp_counts_with_first_char(polymer_counter: &PolymerCounterMap, first_char: char) {
    debug!(
        "  [TMP] ----- Count chars (first char: '{}') -----",
        first_char
    );
    let mut count: HashMap<char, usize> = HashMap::new();

    // count the first input char: +1, i.e: NNCB ==> count(N)+1
    *count.entry(first_char).or_default() += 1;
    polymer_counter.iter().for_each(|(key, val)| {
        *count.entry(key.1).or_default() += val;
    });

    count.iter().enumerate().for_each(|(index, (key, count))| {
        debug!("        [{:2}] {} => {}", index, key, count);
    });
}

// -- type InsertionRuleMap = HashMap<(char,char),char>;
fn display_insertion_rules(insertion_rules: &InsertionRuleMap) {
    info!("[ ] ========= Pair Insertion Rules =========");
    for (i, (left, right)) in insertion_rules.iter().enumerate() {
        info!("  [{:2}]: {}{} -> {}", i, left.0, left.1, right);
    }
}

// -- type PolymerCounterMap = HashMap<(char,char),usize>;
fn display_polymer_counter_map(polymer_counter: &PolymerCounterMap, step_count: u32) {
    debug!(
        "[STEP: {}] ========= Polymer Counter Map =========",
        step_count
    );
    polymer_counter
        .iter()
        .enumerate()
        .for_each(|(index, (tuple, count))| {
            debug!("        [{:2}] {:?} => {}", index, tuple, count);
        });
}

fn display_polymer_count_status(
    polymer_counter: &PolymerCounterMap,
    insertion_rules: &InsertionRuleMap,
    first_char: char,
    last_char: char,
) {
    info!(
        "[ ] ===== Polymer Counter Map Status (first: {}, last: {}) =====",
        first_char, last_char
    );
    for (j, (tuple, count)) in polymer_counter.iter().enumerate() {
        let rhs = insertion_rules.get(tuple).unwrap();
        info!(
            "  [{}] ({},{}) -> {}, {} (count)",
            j, tuple.0, tuple.1, rhs, count
        );
    }
}
