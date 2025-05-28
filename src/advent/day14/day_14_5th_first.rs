// advent/day_14_4th.rs
use log::{debug, info};
use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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
//   [ 11]: "BB" -> 'N'  ==> BN B
//   [ 14]: "BC" -> 'B'  ==> BB C
//   [  9]: "BH" -> 'H'  ==> BH H
//   [  1]: "BN" -> 'B'  ==> BB N

//   [  3]: "CB" -> 'H'  ==> CH B
//   [ 13]: "CC" -> 'N'  ==> CN C
//   [ 15]: "CH" -> 'B'  ==> CB H
//   [  0]: "CN" -> 'C'  ==> CC N

//   [  8]: "HB" -> 'C'  ==> HC B
//   [ 12]: "HC" -> 'B'  ==> HB C
//   [ 10]: "HH" -> 'N'  ==> HN H
//   [  2]: "HN" -> 'C'  ==> HC N

//   [  5]: "NB" -> 'B'  ==> NB B
//   [  4]: "NC" -> 'B'  ==> NB C
//   [  7]: "NH" -> 'C'  ==> NC H
//   [  6]: "NN" -> 'C'  ==> NC N

//  * Step[]: (7), "NC NB CH B"
//   |1|-> counter_map list --, polymer_template |len|-> 7
//         [B]:2,     [C]:2,     [H]:1,     [N]:2,
//       [] - 1 max_count: ('B', 2)
//       [] - 1 min__count: ('H', 1)
//       [] - 1 diff (max-min) = 1
//   * Step[]: (13), "NB CC NB BB CB HC B"
//   |2|-> counter_map list --, polymer_template |len|-> 13
//         [B]:6,     [C]:4,     [H]:1,     [N]:2,
//       [] - 2 max_count: ('B', 6)
//       [] - 2 min__count: ('H', 1)
//       [] - 2 diff (max-min) = 5
//   * Step[]: (25), "NB BB CN CC NB BN BN BB CH BH HB CH B"
//   |3|-> counter_map list --, polymer_template |len|-> 25
//         [B]:11,     [C]:5,     [H]:4,     [N]:5,
//       [] - 3 max_count: ('B', 11)
//       [] - 3 min__count: ('H', 4)
//       [] - 3 diff (max-min) = 7
//   * Step[]: (49), "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
//   |4|-> counter_map list --, polymer_template |len|-> 49
//         [B]:23,     [C]:10,     [H]:5,     [N]:11,
//       [] - 4 max_count: ('B', 23)
//       [] - 4 min__count: ('H', 5)
//       [] - 4 diff (max-min) = 18
//   * Step[]: (97), "NBBNBBNBBBNBBNBBCNCCNBBBCCNBCNCCNBBNBBNBBNBBNBBNBNBBNBBNBBNBBNBBCHBHHBCHBHHNHCNCHBCHBNBBCHBHHBCHB"
//   |5|-> counter_map list --, polymer_template |len|-> 97
//         [B]:46,     [C]:15,     [H]:13,     [N]:23,
//       [] - 5 max_count: ('B', 46)
//       [] - 5 min__count: ('H', 13)
//       [] - 5 diff (max-min) = 33
//   |5|-> counter_map list --, polymer_template |len|-> 97
//         [B]:46,     [C]:15,     [H]:13,     [N]:23,
//       [] - 5 max_count: ('B', 46)
//       [] - 5 min__count: ('H', 13)
//       [] - 5 diff (max-min) = 33
//   -----------------------------------------
//   --- Day 14: Extended Polymerization, Part Two ---
//   [ ] Input File: input/day_14-sample-a.txt
//   ------------------------------------------
//   [*] difference: 33
//   [*] total_step_count: 5
//   -----------------------------------------

type PolymerTemplateMap = HashMap<(char, char), u64>;
type InsertionRuleMap = HashMap<(char, char), char>;

fn handle_input(filename: &str) -> (PolymerTemplateMap, InsertionRuleMap, char, char) {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    info!("[*] Input Filename: {}", filename);
    info!("[*] input lines count = {}", lines_count);

    let first_line = &lines[0];
    let input_template = lines[0].chars().collect::<Vec<char>>();
    let first_char = input_template.first().unwrap();
    let last_char = input_template.last().unwrap();
    info!("[ ] First Line: len={}, {}, ", first_line.len(), first_line);

    // input polymer template: a string with severaal characters
    // the first line from input file
    info!("[ ] input polymer template -------");
    let polymer_input = first_line.chars().collect::<Vec<char>>();
    let mut polymer_template: PolymerTemplateMap = HashMap::new();
    polymer_input.windows(2).for_each(|items| {
        *polymer_template.entry((items[0], items[1])).or_default() += 1;
        info!(
            "  [] {}{} => {}",
            items[0],
            items[1],
            polymer_template.get(&(items[0], items[1])).unwrap()
        );
    });

    // input instructions list:
    // each input line is like (CH -> B style) and starts from 3rd line in input file.
    // So, we skip the first two lines from input file.
    info!("[ ] input instructions list - (CH -> B style) list from 3rd line. so skip the first two lines -------");
    let insertion_rules = lines
        .iter()
        .skip(2)
        .map(|line| line.split_once("->"))
        .map(|item| {
            let (left, right) = item.unwrap();
            let mut left_chars = left.trim().chars();
            let right_char = right.trim().chars().next().unwrap();
            let left0 = left_chars.next().unwrap();
            let left1 = left_chars.next().unwrap();
            ((left0, left1), right_char)
        })
        .collect::<HashMap<(char, char), char>>();

    (polymer_template, insertion_rules, *first_char, *last_char)
}

pub fn do_day_14() {
    day_14_part_one();
}

// type PolymerTemplateMap = HashMap<(char,char),u64>;
// type InsertionRuleMap = HashMap<(char,char),char>;

fn day_14_part_one() {
    info!("===============================================");
    info!("--- Day 14: Extended Polymerization,  Part One ---, 2/5/2022 (Feb,5) ==> DONE");
    info!("===============================================");
    let filename = "input/day_14-sample-a.txt";
    // let filename = "input/day_14-input.txt";
    let (polymer_template, insertion_rules, first_char, last_char) = handle_input(filename);
    info!(
        "[] input -  polymer_template len: {}",
        polymer_template.len()
    );
    info!("[] input -  polymer_template : {:?}", polymer_template);
    info!("[] input -  insertion_rules len: {}", insertion_rules.len());

    display_pair_insertion_rules(&insertion_rules);

    let step_limit = 10;

    do_naive_way(
        &polymer_template,
        &insertion_rules,
        step_limit,
        first_char,
        last_char,
    );

    info!("-----------------------------------------");
    info!("--- Day 14: Extended Polymerization, Part One --- ");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!("-----------------------------------------");
}

fn count_init_polymer_chars(
    polymer_template: &PolymerTemplateMap,
    key_occur_count_map: &mut HashMap<char, u64>,
    first_char: char,
    last_char: char,
) {
    //-- init: count the occurrence of this input string (polymer_template itself)
    for ((tuple), right) in polymer_template.iter() {
        info!("[] input char: {:?}", tuple);
        *key_occur_count_map.entry(tuple.0).or_default() += 1;
    }
    *key_occur_count_map.entry(last_char).or_default() += 1;

    display_occurrence_count(key_occur_count_map);
}

fn do_naive_way(
    polymer_template: &PolymerTemplateMap,
    insertion_rules: &InsertionRuleMap,
    step_limit: u32,
    first_char: char,
    last_char: char,
) {
    let mut key_occur_count_map: HashMap<char, u64> = HashMap::new();

    count_init_polymer_chars(
        &polymer_template,
        &mut key_occur_count_map,
        first_char,
        last_char,
    );

    count_new_char_naive(
        &polymer_template,
        &insertion_rules,
        &mut key_occur_count_map,
        step_limit,
    );

    display_occurrence_count(&key_occur_count_map);
}

fn count_new_char_naive(
    polymer_template: &PolymerTemplateMap,
    insertion_rules: &InsertionRuleMap,
    key_occur_count_map: &mut HashMap<char, u64>,
    step_limit: u32,
) -> PolymerTemplateMap {
    let step_limit = step_limit;
    let mut polymer_template = polymer_template.clone();

    for i in 0..step_limit {
        info!("[STEP: {}] ========================", i);
        let mut new_polymer_template = polymer_template.clone();

        for (j, (tuple, tuple_count)) in polymer_template.iter().enumerate() {
            info!(
                "[{}] ===> input_pair: {:?}{:?} = {}",
                i, tuple.0, tuple.1, tuple_count
            );

            let element = insertion_rules.get(tuple).unwrap();
            *key_occur_count_map.entry(*element).or_default() += 1;

            *new_polymer_template.get_mut(tuple).unwrap() -= tuple_count;
            *new_polymer_template.entry((tuple.0, *element)).or_default() += tuple_count;
            *new_polymer_template.entry((*element, tuple.1)).or_default() += tuple_count;
        }

        display_occurrence_count(key_occur_count_map);

        polymer_template = new_polymer_template;
    }

    polymer_template
}

fn display_occurrence_count(key_occur_count_map: &HashMap<char, u64>) {
    info!("[ ] display_occurrence_count -------");
    key_occur_count_map
        .iter()
        .for_each(|(key, value)| info!("     {} ({})", key, value));

    let (key_max, value_max) = key_occur_count_map
        .iter()
        .max_by_key(|entry| entry.1)
        .unwrap();
    let (key_min, value_min) = key_occur_count_map
        .iter()
        .min_by_key(|entry| entry.1)
        .unwrap();
    info!("[ ] [MAX value] : {} ({}) ", key_max, value_max);
    info!("[ ] [min value] : {} ({}) ", key_min, value_min);
    info!("[ ] [MAX_value - min_value] : {} ", value_max - value_min);
}

fn display_pair_insertion_rules(insertion_rules: &InsertionRuleMap) {
    info!("-------- Pair Insertion Rules --------------------");
    let mut format_str = format!("\n");
    for (i, ((left0, left1), right)) in insertion_rules.iter().enumerate() {
        format_str += &*format!("  [{:3}]: {:?}{:?} -> {:?}\n", i, left0, left1, right);
    }
    info!(" {} ", format_str);
}

//--------------------------------------------------------------------
