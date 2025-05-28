// advent/day_2.rs
//
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn do_day_2() {
    day_2_part_one();
    day_2_part_two();
}

fn handle_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    println!("[*] Input Filename: {}", filename);
    println!("[*] input lines count = {}", lines_count);

    lines
}

//--- Day 2: Dive! ---
fn day_2_part_one() {
    println!("--- Day 2: Dive!, Part One ---");
    let filename = "input/day_2-input.txt";
    let input_lines = handle_input(filename);

    let pairs = input_lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect::<Vec<Vec<&str>>>();

    let mut horizontal = 0;
    let mut depth = 0;
    for pair in pairs.iter() {
        match pair[0] {
            "forward" => {
                horizontal += pair[1].parse::<i32>().unwrap();
                // println!("command = {}, value = {}, horizontal = {}", pair[0], pair[1], horizontal);
            }
            "up" => {
                depth -= pair[1].parse::<i32>().unwrap();
                // println!("command = {}, value = {}, depth = {}", pair[0], pair[1], depth);
            }
            "down" => {
                depth += pair[1].parse::<i32>().unwrap();
                // println!("command = {}, value = {}, depth = {}", pair[0], pair[1], depth);
            }
            _ => {}
        }
    }

    let final_value = horizontal * depth;
    println!("part 1-0: final value = {}", final_value);
    //-- part 1: final value = 1451208
}

fn day_2_part_two() {
    println!("--- Day 2: Dive!, Part Two ---");
    let filename = "input/day_2-input.txt";
    let input_lines = handle_input(filename);

    let pairs = input_lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect::<Vec<Vec<&str>>>();

    //--- part two ---
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for pair in pairs.iter() {
        match pair[0] {
            "forward" => {
                horizontal += pair[1].parse::<i32>().unwrap();
                depth += aim * pair[1].parse::<i32>().unwrap();
                // println!("command = {}, value = {}, horizontal = {}, depth = {}",
                // pair[0], pair[1], horizontal, depth);
            }
            "up" => {
                aim -= pair[1].parse::<i32>().unwrap();
                // println!("command = {}, value = {}, aim = {}", pair[0], pair[1], aim);
            }
            "down" => {
                aim += pair[1].parse::<i32>().unwrap();
                // println!("command = {}, value = {}, aim = {}", pair[0], pair[1], aim);
            }
            _ => {}
        }
    }

    let final_value = horizontal * depth;
    println!("part 2-0: final value = {}", final_value);
    // part 2: final value = 1620141160

    //=====================================================================

    //-- Part 2: another solution ---------------
    let pairs = input_lines
        .iter()
        .map(|line| {
            let tmp: Vec<&str> = line.split_whitespace().collect();
            (tmp[0], tmp[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<(&str, i32)>>();

    let mut horizontal = 0;
    let mut depth = 0;
    for (cmd, value) in pairs.iter() {
        match *cmd {
            "forward" => {
                horizontal += value;
                // println!("command = {}, value = {}, horizontal = {}", pair[0], pair[1], horizontal);
            }
            "up" => {
                depth -= value;
                // println!("command = {}, value = {}, depth = {}", pair[0], pair[1], depth);
            }
            "down" => {
                depth += value;
                // println!("command = {}, value = {}, depth = {}", pair[0], pair[1], depth);
            }
            _ => {}
        }
    }

    let final_value = horizontal * depth;
    println!("part 1-1: final value = {}", final_value);
    // input lines count = 1000
    // part 1-0: final value = 1451208
    // part 2-0: final value = 1620141160
    // part 1-1: final value = 1451208
    // part 2-1: final value = 1620141160

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (cmd, value) in pairs.iter() {
        match *cmd {
            "forward" => {
                horizontal += value;
                depth += aim * value;
                // println!("command = {}, value = {}, horizontal = {}, depth = {}",
                // pair[0], pair[1], horizontal, depth);
            }
            "up" => {
                aim -= value;
                // println!("command = {}, value = {}, aim = {}", pair[0], pair[1], aim);
            }
            "down" => {
                aim += value;
                // println!("command = {}, value = {}, aim = {}", pair[0], pair[1], aim);
            }
            _ => {}
        }
    }

    let final_value = horizontal * depth;
    println!("part 2-1: final value = {}", final_value);
    // part 2: final value = 1620141160
    // part 1-0: final value = 1451208
    // part 2-0: final value = 1620141160
    // part 1-1: final value = 1451208
    // part 2-1: final value = 1620141160
}
