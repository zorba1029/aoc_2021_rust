// advent/day_14.rs
use log::{info, warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
fn handle_input(filename: &str) -> (Vec<char>, HashMap<String, char>) {
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
    info!("[ ] First Line: len={}, {}, ", first_line.len(), first_line);

    info!("[ ] input polymer template -------");
    let polymer_template = first_line.chars().into_iter().collect::<Vec<char>>();

    info!("[ ] input instructions list - (x or y,value) list -------");
    let insertion_rules = lines
        .iter()
        .skip(2)
        .map(|line| line.split("->").collect::<Vec<_>>())
        .map(|items| {
            let key = items[0].trim().to_owned();
            let value = items[1].trim().chars().next().unwrap();
            (key, value)
        })
        .into_iter()
        .collect::<HashMap<String, char>>();

    (polymer_template, insertion_rules)
}

pub fn do_day_14() {
    day_14_part_one();
    day_14_part_two();
}

fn day_14_part_one() {
    info!("===============================================");
    info!("--- Day 14: Extended Polymerization,  Part One ---, 2/5/2022 (Feb,5) ==> DONE");
    info!("===============================================");
    // let filename = "input/day_14-sample-a.txt";
    let filename = "input/day_14-input.txt";
    let (polymer_template, insertion_rules) = handle_input(filename);
    info!(
        "[] input -  polymer_template len: {}",
        polymer_template.len()
    );
    info!("[] input -  polymer_template : {:?}", polymer_template);
    info!("[] input -  insertion_rules len: {}", insertion_rules.len());

    display_pair_insertion_rules(&insertion_rules);

    let total_step_count = 10;
    let new_template =
        apply_insertion_rules_all(&polymer_template, &insertion_rules, total_step_count);
    let difference = get_insertion_value_max_min_difference(&new_template, total_step_count);

    info!("-----------------------------------------");
    info!("--- Day 14: Extended Polymerization, Part One --- ");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!("[*] difference: {}", difference);
    info!("[*] total_step_count: {}", total_step_count);
    info!("-----------------------------------------");
}

fn day_14_part_two() {
    info!("===============================================");
    info!("--- Day 14: Extended Polymerization,  Part Two ---, 2/4/2022 (Feb, 5) ==> DONE");
    info!("===============================================");
    // let filename = "input/day_14-sample-a.txt";
    let filename = "input/day_14-input.txt";
    let (polymer_template, insertion_rules) = handle_input(filename);
    info!(
        "[] input -  polymer_template len: {}",
        polymer_template.len()
    );
    info!("[] input -  polymer_template : {:?}", polymer_template);
    info!("[] input -  insertion_rules len: {}", insertion_rules.len());

    display_pair_insertion_rules(&insertion_rules);

    let total_step_count = 10;
    let new_template =
        apply_insertion_rules_all(&polymer_template, &insertion_rules, total_step_count);
    let difference = get_insertion_value_max_min_difference(&new_template, total_step_count);

    info!("-----------------------------------------");
    info!("--- Day 14: Extended Polymerization, Part Two --- ");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!("[*] difference: {}", difference);
    info!("[*] total_step_count: {}", total_step_count);
    info!("-----------------------------------------");
}

fn apply_insertion_rules_all(
    polymer_template: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    total_step_count: i32,
) -> Vec<char> {
    let mut new_template = polymer_template.clone();
    let mut key_occur_count_map: HashMap<String, u32> = HashMap::new();
    let total_step = total_step_count;

    for step_count in 1..=total_step {
        new_template = apply_insertion_rules_once( &mut new_template, insertion_rules, &mut key_occur_count_map);
        get_insertion_value_max_min_difference(&new_template, step_count);
        // info!("Step[{}]: len=({}) {:?}", step_count, new_template.len(), new_template.iter().collect::<String>());
    }

    // info!("* Final Step[{}]: len=({}) {:?}", total_step, new_template.len(), new_template.iter().collect::<String>());
    new_template
}

fn apply_insertion_rules_once(
    polymer_template: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    key_occur_count_map: &mut HashMap<String, u32>,
) -> Vec<char> {
    let current_template = polymer_template.clone();
    let mut new_template = Vec::new();
    // let mut key_occur_count_map: HashMap<String, u32> = HashMap::new();

    for i in 0..current_template.len() - 1 {
        let first_ch = current_template[i].clone();
        // let second_ch = current_template[i+1].clone();
        // let pair = format!("{first_ch}{second_ch}");
        let pair = current_template[i..=i + 1].iter().collect::<String>();
        // info!("    <{}> - pair: {:?}", i, pair);
        if let Some(element) = insertion_rules.get(&pair) {
            // info!("       --> found: {:?} -> {:?}", pair, element);
            new_template.push(first_ch);
            new_template.push(*element);
        }

        //-- count insertion rule - "key" count
        if key_occur_count_map.contains_key(&pair) {
            *key_occur_count_map.get_mut(&pair).unwrap() += 1;
        } else {
            key_occur_count_map.insert(pair, 0);
        }
        // info!("    <{}> - current status: {:?}", i, new_template.iter().collect::<String>());
    }
    //-- push the last char from the original(from) to the current(to) template
    new_template.push(*current_template.last().unwrap());

    info!(
        "* Step[]: ({}), {:?}",
        new_template.len(),
        new_template.iter().collect::<String>()
    );

    new_template
}

fn display_pair_insertion_rules(insertion_rules: &HashMap<String, char>) {
    info!("-------- Pair Insertion Rules --------------------");
    let mut format_str = format!("\n");
    for (i, line) in insertion_rules.iter().enumerate() {
        format_str += &*format!("  [{:3}]: {:?} -> {:?}\n", i, line.0, line.1);
    }
    info!(" {} ", format_str);
}

fn get_insertion_value_max_min_difference(polymer_template: &Vec<char>, step_count: i32) -> u32 {
    let mut counter_map: HashMap<char, u32> = HashMap::new();

    polymer_template.iter().for_each(|ch| {
        if counter_map.contains_key(ch) {
            *counter_map.get_mut(ch).unwrap() += 1;
        } else {
            counter_map.insert(*ch, 1);
        }
    });

    info!( "|{}|-> counter_map list --, polymer_template |len|-> {}", step_count, polymer_template.len());
    let mut format_str = format!("");
    let mut max_item = (' ', u32::MIN);
    let mut min_item = (' ', u32::MAX);
    let mut sorted_map: Vec<_> = counter_map.into_iter().collect();
    sorted_map.sort_by(|x, y| x.0.cmp(&y.0));

    sorted_map.iter().for_each(|(key, value)| {
        format_str += &*format!("    [{}]:{}, ", key, value);
        if *value > max_item.1 {
            max_item.0 = *key;
            max_item.1 = *value;
        }
        if *value < min_item.1 {
            min_item.0 = *key;
            min_item.1 = *value;
        }
    });
    info!("  {} ", format_str);

    let diff = max_item.1 - min_item.1;

    info!("    [] - {} max_count: {:?}", step_count, max_item);
    info!("    [] - {} min__count: {:?}", step_count, min_item);
    info!("    [] - {} diff (max-min) = {}", step_count, diff);

    diff
}

fn count_insertion_key_occurrences(key_occur_count_map: &HashMap<String, u32>, step_count: i32) {
    //-- key occurrence count
    info!(
        "|{}|-> key occurrence count, len={})",
        step_count,
        key_occur_count_map.len()
    );
    let mut format_str = format!(" ");
    let mut sorted_map: Vec<_> = key_occur_count_map.into_iter().collect();
    sorted_map.sort_by(|x, y| x.0.cmp(&y.0));

    sorted_map.iter().for_each(|(key, value)| {
        // info!("    item[{}]: {}", key, value);
        format_str += &*format!("[{}]:{}, ", key, value);
    });
    info!("  {} ", format_str);
}
