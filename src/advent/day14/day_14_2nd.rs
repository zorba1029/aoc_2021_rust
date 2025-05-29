// advent/day_14.rs
use log::info;
use std::collections::HashMap;
// use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use num_format::Locale::cu;

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

fn handle_input(
    filename: &str,
) -> (
    Vec<char>,
    HashMap<String, char>,
    HashMap<String, Vec<String>>,
) {
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
            // let value = items[1].trim().chars().collect::<Vec<_>>()[0];
            (key, value)
        })
        .into_iter()
        .collect::<HashMap<String, char>>();

    let expand_rules = lines
        .iter()
        .skip(2)
        .map(|line| line.split("->").collect::<Vec<_>>())
        .map(|items| {
            let key = items[0].trim().to_owned();
            let value = items[1].trim().chars().next().unwrap();
            // let value = items[1].trim().chars().collect::<Vec<_>>()[0];
            let value_str = format!("{}", value);
            let next_1 = format!("{}{}", &key[0..1], value);
            let next_2 = format!("{}{}", value, &key[1..]);
            (key, vec![value_str, next_1, next_2])
        })
        .into_iter()
        .collect::<HashMap<String, Vec<String>>>();

    (polymer_template, insertion_rules, expand_rules)
}

pub fn do_day_14() {
    day_14_part_two();
}

fn day_14_part_two() {
    info!("===============================================");
    info!("--- Day 14: Extended Polymerization,  Part Two ---, 2/x/2022 (Feb, x) ==> DONE");
    info!("===============================================");
    let filename = "input/day_14-sample-a.txt";
    // let filename = "input/day_14-input.txt";
    let (polymer_template, insertion_rules, expand_rules) = handle_input(filename);
    info!(
        "[] input -  polymer_template len: {}",
        polymer_template.len()
    );
    info!("[] input -  polymer_template : {:?}", polymer_template);
    info!("[] input -  insertion_rules len: {}", insertion_rules.len());

    display_pair_insertion_rules(&insertion_rules, &expand_rules);

    let mut counter_map: HashMap<char, u64> = HashMap::new();
    let total_step_count = 10;
    apply_insertion_rules_all(
        &polymer_template,
        &mut counter_map,
        &insertion_rules,
        &expand_rules,
        total_step_count,
    );
    //-- get max, min counts
    let (max_item, min_item, max_min_diff) = get_max_min_difference(&counter_map);

    info!("-----------------------------------------");
    info!("--- Day 14: Extended Polymerization, Part Two --- ");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!("[*] max_item: {:?}", max_item);
    info!("[*] min_item: {:?}", min_item);
    info!(
        "[*] difference: {} (=| {} - {})",
        max_min_diff, max_item.1, min_item.1
    );
    info!("[*] total_step_count: {}", total_step_count);
    info!("-----------------------------------------");
    info!("-----------------------------------------");
}

// ===============================================
// --- Day 14: Extended Polymerization,  Part Two ---, 2/x/2022 (Feb, x) ==> DONE
// ===============================================
// [*] Input Filename: input/day_14-sample-a.txt
// [*] input lines count = 18
// [ ] First Line: len=4, NNCB,
// [ ] input polymer template -------
// [ ] input instructions list - (x or y,value) list -------
// [] input -  polymer_template len: 4
// [] input -  polymer_template : ['N', 'N', 'C', 'B']
// [] input -  insertion_rules len: 16
// -------- Expand Rules --------------------
// [  0]: "CB" -> H || ["CH", "HB"]
// [  1]: "CC" -> N || ["CN", "NC"]
// [  2]: "NN" -> C || ["NC", "CN"]
// [  3]: "HB" -> C || ["HC", "CB"]
// [  4]: "HN" -> C || ["HC", "CN"]
// [  5]: "NC" -> B || ["NB", "BC"]
// [  6]: "BN" -> B || ["BB", "BN"]
// [  7]: "BC" -> B || ["BB", "BC"]
// [  8]: "CN" -> C || ["CC", "CN"]
// [  9]: "NH" -> C || ["NC", "CH"]
// [ 10]: "NB" -> B || ["NB", "BB"]
// [ 11]: "BB" -> N || ["BN", "NB"]
// [ 12]: "CH" -> B || ["CB", "BH"]
// [ 13]: "HH" -> N || ["HN", "NH"]
// [ 14]: "HC" -> B || ["HB", "BC"]
// [ 15]: "BH" -> H || ["BH", "HH"]
//
// [*] - polymer_template: "NNCB"
// ----------------------------------------------------------
// [**] counter_map: |- {'B': 1749, 'N': 865, 'C': 298, 'H': 161}
// ----------------------------------------------------------
// [ ] Input File: input/day_14-sample-a.txt
// ------------------------------------------
// [*] max_item: ('B', 1749)
// [*] min_item: ('H', 161)
// [*] difference: 1588 (=| 1749 - 161)
// [*] total_step_count: 10
// -----------------------------------------
// -----------------------------------------

fn apply_insertion_rules_all(
    polymer_template: &Vec<char>,
    counter_map: &mut HashMap<char, u64>,
    insertion_rules: &HashMap<String, char>,
    expand_rules: &HashMap<String, Vec<String>>,
    total_step_count: i32,
) {
    info!(
        "[*] - polymer_template: {:?}",
        polymer_template.iter().collect::<String>()
    );

    //-- count initial input chars
    for i in 0..polymer_template.len() {
        let ch = polymer_template[i].clone();
        // info!("    [] {:?} |- {}", pair, ch);
        if counter_map.contains_key(&ch) {
            *counter_map.get_mut(&ch).unwrap() += 1;
        } else {
            counter_map.insert(*&ch, 1);
        }
    }

    let mut loop_count: u64 = 0;
    for i in 0..polymer_template.len() - 1 {
        // let first_ch = polymer_template[i].clone();
        // let second_ch = polymer_template[i+1].clone();
        // let pair = format!("{first_ch}{second_ch}");
        let pair = polymer_template[i..=i + 1].iter().collect::<String>();

        check_occurrence_recursive(
            &pair,
            counter_map,
            insertion_rules,
            expand_rules,
            1,
            total_step_count,
            &mut loop_count,
        );
    }

    info!("----------------------------------------------------------");
    info!("[**] counter_map AFTER all count: |- {:?}", counter_map);
    info!("----------------------------------------------------------");
}

fn check_occurrence_recursive(
    pair: &String,
    counter_map: &mut HashMap<char, u64>,
    insertion_rules: &HashMap<String, char>,
    expand_rules: &HashMap<String, Vec<String>>,
    current_step: i32,
    total_step_count: i32,
    loop_count: &mut u64,
) {
    *loop_count += 1;
    if current_step > total_step_count {
        return;
    } else {
        // info!("[{}/{}] - check_occurrence() - pair: {:?}", current_step, total_step_count, pair);
    }

    //-- expand rule sample item
    //-- ex) "NC" -> ["B", "NB", "BC"]
    if let Some(element) = expand_rules.get(pair) {
        let ch = &element[0].chars().nth(0).unwrap();
        // info!("    [*]       pair: {:?} |- {}", pair, element);
        if counter_map.contains_key(ch) {
            *counter_map.get_mut(ch).unwrap() += 1;
        } else {
            counter_map.insert(*ch, 1);
        }

        //-- DEBUG
        if current_step % 10 == 0 && current_step > 20 && *loop_count % 1_000_000 == 0 {
            info!(
                "    [{}]: {} -> {}, {:?}, || {:?}",
                current_step,
                pair,
                &element[0],
                &element[1..],
                counter_map
            );
        }

        if current_step < total_step_count {
            for item in element.iter().skip(1) {
                check_occurrence_recursive(
                    &item.clone(),
                    counter_map,
                    insertion_rules,
                    expand_rules,
                    current_step + 1,
                    total_step_count,
                    loop_count,
                );
            }
        }
    }
}

//-- use VecDeque but it takes too many items push-backed
// -- FAILED
// fn check_occurrence_iterative(pair: &String,
//                     counter_map: &mut HashMap<char, u64>,
//                     insertion_rules: &HashMap<String,char>,
//                     expand_rules: &HashMap<String, Vec<String>>,
//                     current_step: i32,
//                     total_step_count: i32) {
//     let mut current_step = current_step;
//     // let pair = format!("{first_ch}{second_ch}");
//     let mut input_list: VecDeque<(String, i32)> = VecDeque::new();
//     input_list.push_back((pair.clone(), current_step));
//
//     if current_step > total_step_count {
//         return;
//     } else {
//         // info!("[{}/{}] - check_occurrence() - pair: {:?}", current_step, total_step_count, pair);
//     }
//
//     let mut level_count: u32 = 0;
//     while input_list.len() > 0 {
//         let (current_input, current_step) = input_list.pop_front().unwrap();
//         if current_step > total_step_count {
//             break;
//         }
//
//         if let Some(element) = expand_rules.get(&current_input) {
//             let ch = &element[0].chars().nth(0).unwrap();
//             if current_step % 10 == 0 {
//                 level_count += 1;
//                 if level_count % 100_000 == 0 {
//                     info!("    [{}] {:?} |- {}, {:?}", current_step, current_input, ch, element);
//                     info!("    [{}] input_list.len() |- {}", current_step, input_list.len());
//                 }
//             }
//
//             if counter_map.contains_key(ch) {
//                 *counter_map.get_mut(ch).unwrap() += 1;
//             } else {
//                 counter_map.insert(*ch, 1);
//             }
//
//             for item in element.iter().skip(1) {
//                 input_list.push_back((item.clone(), current_step+1));
//             }
//         }
//     }
// }

fn display_pair_insertion_rules(
    _insertion_rules: &HashMap<String, char>,
    expand_rules: &HashMap<String, Vec<String>>,
) {
    // info!("-------- Pair Insertion Rules --------------------");
    // let mut format_str = format!("\n");
    // for (i, line) in insertion_rules.iter().enumerate() {
    //     format_str += &*format!("  [{:3}]: {:?} -> {:?}\n", i, line.0, line.1);
    // }
    // info!(" {} ", format_str);

    info!("-------- Expand Rules --------------------");
    let mut format_str = format!("\n");
    for (i, line) in expand_rules.iter().enumerate() {
        format_str += &*format!(
            "  [{:3}]: {:?} -> {} || [{:?}, {:?}]\n",
            i, line.0, line.1[0], line.1[1], line.1[2]
        );
    }
    info!(" {} ", format_str);
}

fn get_max_min_difference(counter_map: &HashMap<char, u64>) -> ((char, u64), (char, u64), u64) {
    //-- get max, min counts
    let max_item = counter_map.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    let min_item = counter_map.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap();
    let max_min_diff: u64 = max_item.1 - min_item.1;

    (
        (*max_item.0, *max_item.1),
        (*min_item.0, *min_item.1),
        max_min_diff,
    )
}
