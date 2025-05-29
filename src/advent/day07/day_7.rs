// advent/day_6.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

///////////////////////////////////////////
//--- Day 7: The Treachery of Whales ---
///////////////////////////////////////////
//-------------------------------------------------------------------------

pub fn do_day_7() {
    day_7_part_one();
    day_7_part_two();
}

// input: a list of the horizontal position of each crab (your puzzle input)
// ex: horizontal positions:
//     16,1,2,0,4,2,7,1,2,14
fn handle_input(filename: &str) -> Vec<u32> {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    println!("[*] Input Filename: {}", filename);
    println!("[*] input lines count = {}", lines_count);

    let first_line = &lines[0];
    // println!("[ ] Initial State: {}", first_line);

    // 16,1,2,0,4,2,7,1,2,14
    let horizontal_positions: Vec<u32> = first_line
        .split(',')
        // .map(|a| a.to_string().parse().unwrap())
        .map(|a| a.parse::<u32>().unwrap())
        .collect();

    horizontal_positions
}

#[allow(dead_code)]
fn get_distance_list_1(horizontal_positions: &Vec<u32>) -> Vec<i32> {
    let mut distance_list = Vec::new();
    for i in 0..horizontal_positions.len() {
        let mut dist = 0;
        horizontal_positions.iter().for_each(|val| {
            dist += i32::abs(i as i32 - *val as i32);
        });
        distance_list.push(dist);
    }
    distance_list
}

#[allow(dead_code)]
fn get_distance_list_2(horizontal_positions: &Vec<u32>) -> Vec<i32> {
    let mut distance_list = Vec::new();
    for i in 0..horizontal_positions.len() {
        let dist = horizontal_positions
            .iter()
            .fold(0, |acc, val| {
                let dist = acc + i32::abs(i as i32 - *val as i32);
                dist
            });
        distance_list.push(dist);
    }
    distance_list
}

#[allow(dead_code)]
fn get_distance_list_2_b(horizontal_positions: &Vec<u32>) -> Vec<i32> {
    let mut distance_list = Vec::new();
    for i in 0..horizontal_positions.len() {
        let dist = horizontal_positions
            .iter()
            .fold(0, |acc, val| acc + i32::abs(i as i32 - *val as i32));
        distance_list.push(dist);
    }
    distance_list
}

// fn get_distance_list_3(horizontal_positions: &Vec<u32>) -> Vec<i32> {
//     let distance_list = (0..horizontal_positions.len())
//         .map(|i| {
//             let dist = horizontal_positions
//                 .iter()
//                 .fold(0, |acc, val| acc + i32::abs(i as i32 - *val as i32));
//             dist
//         })
//         .collect::<Vec<_>>();
//     distance_list
// }
fn get_distance_list_3(horizontal_positions: &Vec<u32>) -> Vec<i32> {
    let distance_list = (0..horizontal_positions.len())
        .map(|i| {
            horizontal_positions
                .iter()
                .map(|&val| i32::abs(i as i32 - val as i32))
                .sum()
        })
        .collect::<Vec<_>>();
    distance_list
}

fn day_7_part_one() {
    println!("===============================================");
    println!("--- Day 7: The Treachery of Whales, Part One ---, 1/9/2022 ==> DONE");
    println!("===============================================");
    // let filename = "input/day_7-sample.txt";
    let filename = "input/day_7-input.txt";
    let horizontal_positions = handle_input(filename);
    // println!("horizontal positions: {:?}", horizontal_positions);

    //-- distance_list
    // distance_list[i] = sum of (all distances between each postion and i)
    //-- method-1
    // let distance_list = get_distance_list_1(&horizontal_positions);
    //-- method-2
    // let distance_list = get_distance_list_2(&horizontal_positions);
    //-- method-3
    // let distance_list = get_distance_list_2_b(&horizontal_positions);
    //-- method-4
    let distance_list = get_distance_list_3(&horizontal_positions);

    println!("distance_list: {:?}", distance_list);

    // let mut min_distance = distance_list[0];
    // let mut min_position = 0;
    // for (i, v) in distance_list.iter().enumerate() {
    //     if *v < min_distance {
    //         min_distance = *v;
    //         min_position = i;
    //     }
    // }
    let (min_position, min_distance) = distance_list.iter()
        .enumerate()
        .min_by_key(|(_i, &v)| v)
        .map(|(i, &v)| (i, v))
        .unwrap();
   
    println!( "total feul (min_distance): {}, min_position: {}", min_distance, min_position );

    // let min_fuel = horizontal_positions.iter()
    //     .map(|v| i32::abs(min_position as i32 - *v as i32))
    //     .collect::<Vec<i32>>();
    // println!("min_fuel: {:?}, min_position: {}", min_fuel.iter().sum::<i32>(), min_position);

    println!("-----------------------------------------------");
}

//===============================================
// --- Day 7: Lanternfish, Part One ---, 1/9/2022 ==> DONE
// ===============================================
// [*] Input Filename: input/day_7-input.txt
// [*] input lines count = 1
// total feul (min_distance): 335271, min_position: 313
// -----------------------------------------------

fn acc_distance_list(horizontal_positions: &Vec<u32>) -> Vec<i32> {
    // let step_cost = |distance: i32| -> i32 {
    //     let cost = (1..=distance).fold(0, |acc, v| acc + v);
    //     cost
    // };
    macro_rules! step_cost {
        ($distance:expr) => {{
            let cost = (1..=$distance).fold(0, |acc, v| acc + v);
            cost
        }};
    }

    let distance_list = (0..horizontal_positions.len())
        .map(|i| {
            let dist = horizontal_positions.iter().fold(0, |acc, val| {
                acc + step_cost!(i32::abs(i as i32 - *val as i32))
            });
            dist
        })
        .collect::<Vec<_>>();
    distance_list
}

fn day_7_part_two() {
    println!("===============================================");
    println!("--- Day 7: The Treachery of Whales, Part Two ---, 1/9/2022 ==> DONE");
    println!("===============================================");
    // let filename = "input/day_7-sample.txt";
    let filename = "input/day_7-input.txt";
    let horizontal_positions = handle_input(filename);

    //-- distance_list
    // (ex) distance_list[i] ::= sum of (all distacnes between each postion and i)

    // let step_cost = |distance: i32| -> i32 {
    //     let cost = (1..=distance).fold(0, |acc,v| acc + v);
    //     cost
    // };
    // let distance_list = (0..horizontal_positions.len())
    //     .map(|i| {
    //         let dist = horizontal_positions.iter()
    //             .fold(0, |acc, val| acc + step_cost(i32::abs(i as i32 - *val as i32)));
    //         dist
    //     })
    //     .collect::<Vec<_>>();

    let distance_list = acc_distance_list(&horizontal_positions);
    // println!("distance_list: {:?}", distance_list);

    //-- find the minimum distance and its position index
    // let mut min_distance = distance_list[0];
    // let mut min_position = 0;
    // for (i, v) in distance_list.iter().enumerate() {
    //     if *v < min_distance {
    //         min_distance = *v;
    //         min_position = i;
    //     }
    // }
    let (min_position, min_distance) = distance_list.iter()
        .enumerate()
        .min_by_key(|(_i, &v)| v)
        .map(|(i, &v)| (i, v))
        .unwrap();

    println!( "total feul (min_distance): {}, min_position: {:?}", min_distance, min_position );

    // let min_fuel = horizontal_positions.iter()
    //     .map(|v| step_cost(i32::abs(min_position as i32 - *v as i32)))
    //     .collect::<Vec<i32>>();
    // println!("min_fuel: {}, min_position: {:?}", min_fuel.iter().sum::<i32>(), min_position);

    println!("-----------------------------------------------");
}

// ===============================================
// --- Day 7: Lanternfish, Part Two ---, 1/9/2022 ==> DONE
// ===============================================
// [*] Input Filename: input/day_7-input.txt
// [*] input lines count = 1
// total feul (min_distance): 95851339, min_position: 461
// -----------------------------------------------
