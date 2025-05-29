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

    // input polymer template: a string with severaal characters
    // the first line from input file
    info!("[ ] input polymer template -------");
    let polymer_template = first_line.chars().collect::<Vec<char>>();

    // input instructions list:
    // each input line is like (CH -> B style) and starts from 3rd line in input file.
    // So, we skip the first two lines from input file.
    info!("[ ] input instructions list - (CH -> B style) list from 3rd line. so skip the first two lines -------");
    let insertion_rules = lines
        .iter()
        .skip(2)
        .map(|line| line.split("->").collect::<Vec<_>>())
        .map(|items| {
            let key = items[0].trim().to_owned();
            let value = items[1].trim().chars().next().unwrap();
            (key, value)
        })
        .collect::<HashMap<String, char>>();

    (polymer_template, insertion_rules)
}

pub fn do_day_14() {
    day_14_part_one();
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

    let step_limit = 40;

    //-- naive
    do_naive_way(&polymer_template, &insertion_rules, step_limit);

    //-- smart
    do_smart_way(&polymer_template, &insertion_rules, step_limit);

    info!("-----------------------------------------");
    info!("--- Day 14: Extended Polymerization, Part One --- ");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!("-----------------------------------------");
}

fn do_naive_way(
    polymer_template: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    step_limit: u32,
) {
    let mut key_occur_count_map: HashMap<char, u64> = HashMap::new();

    count_new_char_naive(
        &polymer_template,
        &insertion_rules,
        &mut key_occur_count_map,
        step_limit,
    );

    display_occurrence_count(&key_occur_count_map);
}

fn do_smart_way(
    polymer_template: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    step_limit: u32,
) {
    let mut key_occur_count_map: HashMap<char, u64> = HashMap::new();
    let mut global_loop_count: u64 = 0;

    count_new_char_smart(
        &polymer_template,
        &insertion_rules,
        &mut key_occur_count_map,
        step_limit,
        &mut global_loop_count,
    );

    display_occurrence_count(&key_occur_count_map);
}

fn count_init_polymer_chars(
    polymer_template: &Vec<char>,
    key_occur_count_map: &mut HashMap<char, u64>,
) {
    //-- init: count the occurrence of this input string (polymer_template itself)
    polymer_template.iter().for_each(|item| {
        info!("[] input char: {}", item);
        if key_occur_count_map.contains_key(item) {
            // info!("  -- UPDATE: {}", item);
            *key_occur_count_map.get_mut(item).unwrap() += 1;
        } else {
            // info!("  -- INSERT: {}", item);
            key_occur_count_map.insert(*item, 1);
        }
    });

    display_occurrence_count(key_occur_count_map);
}

//-- Aug 12, 2023
fn count_new_char_naive(
    polymer_template: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    key_occur_count_map: &mut HashMap<char, u64>,
    step_limit: u32,
) {
    let step_limit = step_limit;

    //-- init: count the occurrence of this input string (polymer_template itself)
    count_init_polymer_chars(&polymer_template, key_occur_count_map);

    for i in 0..polymer_template.len() - 1 {
        let first_ch = polymer_template[i];
        let second_ch = polymer_template[i + 1];
        let input_pair = polymer_template[i..=i + 1].iter().collect::<String>();
        info!("[{}] ===> input_pair: {:?}", i, input_pair);

        if let Some(element) = insertion_rules.get(&input_pair) {
            info!(" TOP      --> found: {:?} -> {:?}", input_pair, element);

            //-- count insertion rule - "key" count
            if let Some(item) = key_occur_count_map.get_mut(element) {
                *item += 1;
            } else {
                key_occur_count_map.insert(*element, 1);
            }

            let step_count = 1;
            let new_pair_one = vec![first_ch, *element];
            let new_pair_two = vec![*element, second_ch];

            count_first_part(
                new_pair_one,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
            );
            count_second_part(
                new_pair_two,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
            );
        }
    }
}

fn count_first_part(
    input_pair: Vec<char>,
    insertion_rules: &HashMap<String, char>,
    key_occur_count_map: &mut HashMap<char, u64>,
    step_count: u32,
    step_limit: u32,
) {
    let first_ch = input_pair[0];
    let second_ch = input_pair[1];
    let input_pair = input_pair[0..=1].iter().collect::<String>();
    debug!(
        "[{}/{}] STEP/LIMIT ===> input_pair: {:?}",
        step_count, step_limit, input_pair
    );

    if step_count >= step_limit {
        debug!(
            "    RETURN FIRST --> step_count: {:?} >= step_limit: {:?}",
            step_count, step_limit
        );
        return;
    }

    if let Some(element) = insertion_rules.get(&input_pair) {
        debug!("    FIRST   --> found: {:?} -> {:?}", input_pair, element);

        //-- count insertion rule - "key" count
        // if key_occur_count_map.contains_key(element) {
        //     *key_occur_count_map.get_mut(element).unwrap() += 1;
        // } else {
        //     key_occur_count_map.insert(*element, 1);
        // }
        if let Some(item) = key_occur_count_map.get_mut(element) {
            *item += 1;
        } else {
            key_occur_count_map.insert(*element, 1);
        }

        let step_count = step_count + 1;
        let new_pair_one = vec![first_ch, *element];
        let new_pair_two = vec![*element, second_ch];

        count_first_part(
            new_pair_one,
            insertion_rules,
            key_occur_count_map,
            step_count,
            step_limit,
        );
        count_second_part(
            new_pair_two,
            insertion_rules,
            key_occur_count_map,
            step_count,
            step_limit,
        );
    }
}

fn count_second_part(
    input_pair: Vec<char>,
    insertion_rules: &HashMap<String, char>,
    key_occur_count_map: &mut HashMap<char, u64>,
    step_count: u32,
    step_limit: u32,
) {
    let first_ch = input_pair[0];
    let second_ch = input_pair[1];
    let input_pair = input_pair[0..=1].iter().collect::<String>();
    debug!(
        "[{}/{}] STEP/LIMIT ===> input_pair: {:?}",
        step_count, step_limit, input_pair
    );

    if step_count >= step_limit {
        debug!(
            "    RETURN SECOND --> step_count: {:?} >= step_limit: {:?}",
            step_count, step_limit
        );
        return;
    }

    if let Some(element) = insertion_rules.get(&input_pair) {
        debug!("    SECOND    --> found: {:?} -> {:?}", input_pair, element);

        //-- count insertion rule - "key" count
        // if key_occur_count_map.contains_key(element) {
        //     *key_occur_count_map.get_mut(element).unwrap() += 1;
        // } else {
        //     key_occur_count_map.insert(*element, 1);
        // }
        if let hash_map::Entry::Vacant(new_entry) = key_occur_count_map.entry(*element) {
            new_entry.insert(1);
        } else {
            *key_occur_count_map.get_mut(element).unwrap() += 1;
        }

        let step_count = step_count + 1;
        let new_pair_one = vec![first_ch, *element];
        let new_pair_two = vec![*element, second_ch];

        count_first_part(
            new_pair_one,
            insertion_rules,
            key_occur_count_map,
            step_count,
            step_limit,
        );
        count_second_part(
            new_pair_two,
            insertion_rules,
            key_occur_count_map,
            step_count,
            step_limit,
        );
    }
}

//--------------------------------------------------------------------

fn count_smart_init_polymer_chars(
    polymer_template: &Vec<char>,
    key_occur_count_map: &mut HashMap<char, u64>,
) {
    //-- init: count the occurrence of this input string (polymer_template itself)
    polymer_template.iter().for_each(|item| {
        info!("[] input char: {}", item);
        if let hash_map::Entry::Vacant(new_entry) = key_occur_count_map.entry(*item) {
            new_entry.insert(1);
            // info!("  -- INSERT: {}", item);
        } else {
            *key_occur_count_map.get_mut(item).unwrap() += 1;
            // info!("  -- UPDATE: {}", item);
        }
    });

    display_occurrence_count(key_occur_count_map);
}

fn count_new_char_smart(
    polymer_template: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    key_occur_count_map: &mut HashMap<char, u64>,
    step_limit: u32,
    global_loop_count: &mut u64,
) {
    let step_limit = step_limit;

    //-- init: count the occurrence of this input string (polymer_template itself)
    count_smart_init_polymer_chars(&polymer_template, key_occur_count_map);

    //-- start counting
    for i in 0..polymer_template.len() - 1 {
        let first_ch = polymer_template[i];
        let second_ch = polymer_template[i + 1];
        let input_pair = polymer_template[i..=i + 1].iter().collect::<String>();
        info!("[{}] ===> input_pair: {:?}", i, input_pair);

        if let Some(element) = insertion_rules.get(&input_pair) {
            info!(" TOP      --> found: {:?} -> {:?}", input_pair, element);

            //-- count insertion rule - "key" count
            if let hash_map::Entry::Vacant(new_entry) = key_occur_count_map.entry(*element) {
                new_entry.insert(1);
            } else {
                *key_occur_count_map.get_mut(element).unwrap() += 1;
            }

            let step_count = 1;
            *global_loop_count += 1;

            let new_pair_one = vec![first_ch, *element];
            let new_pair_two = vec![*element, second_ch];

            count_first_part_smart(
                new_pair_one,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
                global_loop_count,
            );
            count_second_part_smart(
                new_pair_two,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
                global_loop_count,
            );
        }
    }
}

fn count_first_part_smart(
    input_pair: Vec<char>,
    insertion_rules: &HashMap<String, char>,
    key_occur_count_map: &mut HashMap<char, u64>,
    step_count: u32,
    step_limit: u32,
    global_loop_count: &mut u64,
) {
    let first_ch = input_pair[0];
    let second_ch = input_pair[1];
    let input_pair = format!("{first_ch}{second_ch}");
    debug!(
        "[{}/{}] STEP/LIMIT ===> input_pair: {:?}",
        step_count, step_limit, input_pair
    );

    if step_count >= step_limit {
        debug!(
            "    RETURN FIRST --> step_count: {:?} >= step_limit: {:?}",
            step_count, step_limit
        );
        if *global_loop_count % 10000000 == 0 {
            info!(
                "    RETURN FIRST --> *global_loop_count: {} mil, {}",
                *global_loop_count / 1000000,
                *global_loop_count
            );
            if *global_loop_count % 10000000 == 0 {
                display_occurrence_count(key_occur_count_map);
            }
        }
        return;
    }

    if let Some(element) = insertion_rules.get(&input_pair) {
        debug!("    FIRST   --> found: {:?} -> {:?}", input_pair, element);

        //-- count insertion rule - "key" count
        if let hash_map::Entry::Vacant(new_entry) = key_occur_count_map.entry(*element) {
            new_entry.insert(1);
        } else {
            *key_occur_count_map.get_mut(element).unwrap() += 1;
        }

        let step_count = step_count + 1;
        *global_loop_count += 1;

        if first_ch == *element || second_ch == *element {
            // left-recursion
            debug!(
                "     [*****] SECOND ===> left : {:?} -> {} ",
                input_pair, *element
            );
            let tmp_step_count = step_limit as u64 - step_count as u64;
            *key_occur_count_map.get_mut(element).unwrap() += tmp_step_count;

            let mut tmp_polymer_template: Vec<char> = vec![];
            for _ in 0..=tmp_step_count {
                tmp_polymer_template.push(*element);
            }

            for i in 0..tmp_polymer_template.len() - 1 {
                let first_ch = tmp_polymer_template[i];
                let second_ch = tmp_polymer_template[i + 1];
                let new_pair_one = vec![first_ch, second_ch];
                let step_count = step_count + i as u32;
                *global_loop_count += 1;

                count_first_part_smart(
                    new_pair_one,
                    insertion_rules,
                    key_occur_count_map,
                    step_count,
                    step_limit,
                    global_loop_count,
                );
            }
        } else {
            let new_pair_one = vec![first_ch, *element];
            let new_pair_two = vec![*element, second_ch];
            debug!(
                "         FIRST ELSE --> found: pair_one: {:?}, pair_two: {:?}",
                new_pair_one, new_pair_two
            );

            count_first_part_smart(
                new_pair_one,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
                global_loop_count,
            );
            count_second_part_smart(
                new_pair_two,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
                global_loop_count,
            );
        }
    }
}

fn count_second_part_smart(
    input_pair: Vec<char>,
    insertion_rules: &HashMap<String, char>,
    key_occur_count_map: &mut HashMap<char, u64>,
    step_count: u32,
    step_limit: u32,
    global_loop_count: &mut u64,
) {
    let first_ch = input_pair[0];
    let second_ch = input_pair[1];
    let input_pair = input_pair[0..=1].iter().collect::<String>();
    debug!(
        "[{}/{}] STEP/LIMIT ===> input_pair: {:?}",
        step_count, step_limit, input_pair
    );

    if step_count >= step_limit {
        debug!(
            "    RETURN SECOND --> step_count: {:?} >= step_limit: {:?}",
            step_count, step_limit
        );
        if *global_loop_count % 10000000 == 0 {
            info!(
                "    RETURN SECOND --> *global_loop_count: {} mil, {}",
                *global_loop_count / 1000000,
                *global_loop_count
            );
            if *global_loop_count % 10000000 == 0 {
                display_occurrence_count(key_occur_count_map);
            }
        }
        return;
    }

    if let Some(element) = insertion_rules.get(&input_pair) {
        debug!("    SECOND    --> found: {:?} -> {:?}", input_pair, element);

        //-- count insertion rule - "key" count
        if let hash_map::Entry::Vacant(new_entry) = key_occur_count_map.entry(*element) {
            new_entry.insert(1);
        } else {
            *key_occur_count_map.get_mut(element).unwrap() += 1;
        }

        let step_count = step_count + 1;
        *global_loop_count += 1;

        if first_ch == *element || second_ch == *element {
            // left-recursion
            debug!(
                "     [*****] SECOND ===> left : {:?} -> {} ",
                input_pair, *element
            );
            let tmp_step_count = step_limit as u64 - step_count as u64;
            *key_occur_count_map.get_mut(element).unwrap() += tmp_step_count;
            // let occur_count = key_occur_count_map.get(element).unwrap();

            let mut tmp_polymer_template: Vec<char> = vec![];
            for _ in 0..=tmp_step_count {
                tmp_polymer_template.push(*element);
            }

            for i in 0..tmp_polymer_template.len() - 1 {
                let first_ch = tmp_polymer_template[i];
                let second_ch = tmp_polymer_template[i + 1];
                let new_pair_one = vec![first_ch, second_ch];
                let step_count = step_count + i as u32;
                *global_loop_count += 1;

                count_first_part_smart(
                    new_pair_one,
                    insertion_rules,
                    key_occur_count_map,
                    step_count,
                    step_limit,
                    global_loop_count,
                );
            }
        } else {
            let new_pair_one = vec![first_ch, *element];
            let new_pair_two = vec![*element, second_ch];
            debug!(
                "         SECOND ELSE --> found: pair_one: {:?}, pair_two: {:?}",
                new_pair_one, new_pair_two
            );

            count_first_part_smart(
                new_pair_one,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
                global_loop_count,
            );
            count_second_part_smart(
                new_pair_two,
                insertion_rules,
                key_occur_count_map,
                step_count,
                step_limit,
                global_loop_count,
            );
        }
    }
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

// input: "input/day_14-sample-a.txt"
// step_limit = 5;
//  [ ] display_occurrence_count -------
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > H = 13
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > N = 23
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > C = 15
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > B = 46

// input: "input/day_14-sample-a.txt"
// step_limit = 10;
// > [ ] display_occurrence_count -------
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > N = 865
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > B = 1749
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > H = 161
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > C = 298

//-- input: "input/day_14-input.txt"
// step_limit = 10;
//  [ ] display_occurrence_count -------
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > C = 1385
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > P = 2147
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > B = 3585
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > K = 1096
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > O = 1917
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > N = 597
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > H = 2526
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > S = 3051
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > V = 1850
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > F = 1303
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > [ ] max : V = 3585
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > [ ] min : B = 597
//  INFO  advent_code_2021::advent::day14::day_14_5th_first > [ ] [max_value - min_value] : 2988

//-- input: "input/day_14-input.txt"
// step_limit = 40;

//==============================================================================
#[allow(dead_code)]
fn apply_insertion_rules_all(
    polymer_template: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    total_step_count: i32,
) -> Vec<char> {
    let mut new_template = polymer_template.clone();
    let mut key_occur_count_map: HashMap<String, u32> = HashMap::new();
    let total_step = total_step_count;

    for step_count in 1..=total_step {
        new_template = apply_insertion_rules_once(
            &mut new_template,
            insertion_rules,
            &mut key_occur_count_map,
        );
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
        let first_ch = current_template[i];
        // let second_ch = current_template[i+1].clone();
        // let pair = format!("{first_ch}{second_ch}");
        let input_pair = current_template[i..=i + 1].iter().collect::<String>();
        info!("    <{}> input_pair: {:?}", i, input_pair);

        if let Some(element) = insertion_rules.get(&input_pair) {
            info!("       --> found: {:?} -> {:?}", input_pair, element);
            new_template.push(first_ch);
            new_template.push(*element);
        }

        //-- count insertion rule - "key" count
        if key_occur_count_map.contains_key(&input_pair) {
            *key_occur_count_map.get_mut(&input_pair).unwrap() += 1;
        } else {
            key_occur_count_map.insert(input_pair, 1);
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

    info!(
        "|{}|-> counter_map list --, polymer_template |len|-> {}",
        step_count,
        polymer_template.len()
    );
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

#[allow(dead_code)]
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
