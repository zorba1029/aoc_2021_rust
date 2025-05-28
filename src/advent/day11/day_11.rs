// advent/day_11.rs
use log::{debug, info};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

///////////////////////////////////////////
//---   Day 11: Dumbo Octopus          ---
///////////////////////////////////////////

pub fn do_day_11() {
    day_11_part_one();
    day_11_part_two();
}

fn day_11_part_one() {
    info!("===============================================");
    info!("--- Day 11: Dumbo Octopus, Part One ---, 1/23/2022 ==> DONE");
    info!("===============================================");
    // let filename = "input/day_11-sample-a.txt";
    // let filename = "input/day_11-sample-b.txt";
    let filename = "input/day_11-input.txt";
    let input_lines = handle_input(filename);
    info!("input_lines: {:?}", input_lines.len());

    let mut energy_level_table = make_energy_level_table(&input_lines);
    display_energy_level_table(&energy_level_table, 0);

    let mut total_flash_count = 0;

    let total_step = 100;
    for step in 1..=total_step {
        energy_level_table = update_energy_level(energy_level_table, &mut total_flash_count, step);
        display_energy_level_table(&energy_level_table, step);
    }

    info!("-----------------------------------------");
    info!("🟠 --- Day 11: Dumbo Octopus, 🟠 Part One --- ");
    info!("Input File: {}", filename);
    info!("Total Step: {}", total_step);
    info!("🟢 Total Flash Count: {}", total_flash_count);
    info!("-----------------------------------------");
}

//-------------------------------------------------------
//-- For input/day_11-sample-b.txt
// 1) step 10;
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//     INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-sample-b.txt
//     INFO  advent_code_2021::advent::day_11 > Total Step: 10
//     INFO  advent_code_2021::advent::day_11 > Total Flash Count: 204
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
// 2) step 100;
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//     INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-sample-b.txt
//     INFO  advent_code_2021::advent::day_11 > Total Step: 100
//     INFO  advent_code_2021::advent::day_11 > Total Flash Count: 1656
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//-------------------------
// INFO  advent_code_2021::advent::day_11 >    [100]-[2] energy_level_table: After STEP[100]-[2] ---------
// INFO  advent_code_2021::advent::day_11 >     <0>: 🥥 3 9 7 6 6 6 8 6 6
// INFO  advent_code_2021::advent::day_11 >     <1>: 🥥 7 4 9 7 6 6 9 1 8
// INFO  advent_code_2021::advent::day_11 >     <2>: 🥥🥥 5 3 9 7 6 9 3 3
// INFO  advent_code_2021::advent::day_11 >     <3>: 🥥🥥🥥 4 2 9 7 8 2 2
// INFO  advent_code_2021::advent::day_11 >     <4>: 🥥🥥🥥 4 2 2 9 8 9 2
// INFO  advent_code_2021::advent::day_11 >     <5>: 🥥🥥 5 3 2 2 2 8 7 7
// INFO  advent_code_2021::advent::day_11 >     <6>: 🥥 5 3 2 2 2 2 9 6 6
// INFO  advent_code_2021::advent::day_11 >     <7>:  9 3 2 2 2 2 8 9 6 6
// INFO  advent_code_2021::advent::day_11 >     <8>:  7 9 2 2 2 8 6 8 6 6
// INFO  advent_code_2021::advent::day_11 >     <9>:  6 7 8 9 9 9 8 7 6 6
// INFO  advent_code_2021::advent::day_11 > [100] 🍏🍒 DISPLAY energy_level_table: After STEP[100] 🍏🍒 ---------
// INFO  advent_code_2021::advent::day_11 >  [0]:  _ 3 9 7 6 6 6 8 6 6
// INFO  advent_code_2021::advent::day_11 >  [1]:  _ 7 4 9 7 6 6 9 1 8
// INFO  advent_code_2021::advent::day_11 >  [2]:  _ _ 5 3 9 7 6 9 3 3
// INFO  advent_code_2021::advent::day_11 >  [3]:  _ _ _ 4 2 9 7 8 2 2
// INFO  advent_code_2021::advent::day_11 >  [4]:  _ _ _ 4 2 2 9 8 9 2
// INFO  advent_code_2021::advent::day_11 >  [5]:  _ _ 5 3 2 2 2 8 7 7
// INFO  advent_code_2021::advent::day_11 >  [6]:  _ 5 3 2 2 2 2 9 6 6
// INFO  advent_code_2021::advent::day_11 >  [7]:  9 3 2 2 2 2 8 9 6 6
// INFO  advent_code_2021::advent::day_11 >  [8]:  7 9 2 2 2 8 6 8 6 6
// INFO  advent_code_2021::advent::day_11 >  [9]:  6 7 8 9 9 9 8 7 6 6
// INFO  advent_code_2021::advent::day_11 > -----------------------------------------
// INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-sample-b.txt
// INFO  advent_code_2021::advent::day_11 > Total Step: 100
// INFO  advent_code_2021::advent::day_11 > Total Flash Count: 1656
// INFO  advent_code_2021::advent::day_11 > -----------------------------------------

// INFO  advent_code_2021::advent::day_11 >    [100]-[3] energy_level_table: After STEP[100]-[3] ---------
//  INFO  advent_code_2021::advent::day_11 >     <0>:  9 3 3 4 6 3🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <1>:  3 3 4 6 2 3 5🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <2>:  3 4 6 2 2 2 3 5🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <3>:  4 6 2 2 2 2 2 3 6🥥
//  INFO  advent_code_2021::advent::day_11 >     <4>:  5 2 3 4 4 3 2 3 6 2
//  INFO  advent_code_2021::advent::day_11 >     <5>:  6 4 6🥥🥥 5 4 5 2 1
//  INFO  advent_code_2021::advent::day_11 >     <6>:  6 4🥥🥥🥥🥥 7🥥 3 1
//  INFO  advent_code_2021::advent::day_11 >     <7>:  7🥥🥥🥥🥥🥥🥥🥥 9 2
//  INFO  advent_code_2021::advent::day_11 >     <8>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥 5
//  INFO  advent_code_2021::advent::day_11 >     <9>:  4🥥🥥🥥🥥🥥🥥🥥 6🥥
//  INFO  advent_code_2021::advent::day_11 > [100] 🍏🍒 DISPLAY energy_level_table: After STEP[100] 🍏🍒 ---------
//  INFO  advent_code_2021::advent::day_11 >  [0]:  9 3 3 4 6 3 _ _ _ _
//  INFO  advent_code_2021::advent::day_11 >  [1]:  3 3 4 6 2 3 5 _ _ _
//  INFO  advent_code_2021::advent::day_11 >  [2]:  3 4 6 2 2 2 3 5 _ _
//  INFO  advent_code_2021::advent::day_11 >  [3]:  4 6 2 2 2 2 2 3 6 _
//  INFO  advent_code_2021::advent::day_11 >  [4]:  5 2 3 4 4 3 2 3 6 2
//  INFO  advent_code_2021::advent::day_11 >  [5]:  6 4 6 _ _ 5 4 5 2 1
//  INFO  advent_code_2021::advent::day_11 >  [6]:  6 4 _ _ _ _ 7 _ 3 1
//  INFO  advent_code_2021::advent::day_11 >  [7]:  7 _ _ _ _ _ _ _ 9 2
//  INFO  advent_code_2021::advent::day_11 >  [8]:  _ _ _ _ _ _ _ _ _ 5
//  INFO  advent_code_2021::advent::day_11 >  [9]:  4 _ _ _ _ _ _ _ 6 _
//  INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//  INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-input.txt
//  INFO  advent_code_2021::advent::day_11 > Total Step: 100
//  INFO  advent_code_2021::advent::day_11 > Total Flash Count: 1785
//  INFO  advent_code_2021::advent::day_11 > -----------------------------------------

//-------------------------------------------------------
//-- For input/day_11-input.txt
// 1) step 10;
//     INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-input.txt
//     INFO  advent_code_2021::advent::day_11 > Total Step: 10
//     INFO  advent_code_2021::advent::day_11 > Total Flash Count: 209
// 2) step 100;
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//     INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-input.txt
//     INFO  advent_code_2021::advent::day_11 > Total Step: 100
//     INFO  advent_code_2021::advent::day_11 > Total Flash Count: 1785
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------

fn day_11_part_two() {
    info!("===============================================");
    info!("--- Day 11: Dumbo Octopus, Part Two ---, 1/23/2022 ==> DONE");
    info!("===============================================");
    // let filename = "input/day_11-sample-a.txt";
    // let filename = "input/day_11-sample-b.txt";
    let filename = "input/day_11-input.txt";
    let input_lines = handle_input(filename);
    info!("input_lines: {:?}", input_lines.len());

    let energy_level_table = make_energy_level_table(&input_lines);
    display_energy_level_table(&energy_level_table, 0);

    let mut total_flash_count = 0;

    let total_step = 500;
    let mut used_step = 0;
    let mut level_table = energy_level_table;
    for step in 1..=total_step {
        used_step = step;
        let (level_table_result, all_syncd) =
            update_energy_level_until_all(level_table, &mut total_flash_count, step);
        display_energy_level_table(&level_table_result, step);
        level_table = level_table_result;
        if all_syncd {
            break;
        }
    }

    info!("-----------------------------------------");
    info!("🍏 --- Day 11: Dumbo Octopus, 🍏 Part Two ---  ");
    info!("Input File: {}", filename);
    info!("Total Step: {} ", total_step);
    info!("Total Flash Count: {}", total_flash_count);
    info!("🟢 [*] First Step All Octopuses Flashed: {}", used_step);
    info!("-----------------------------------------");
}

//-------------------------------------------------------
//-- For input/day_11-sample-a.txt
// INFO  advent_code_2021::advent::day_11 > [6]: --->> UPDATE energy_level_table: step[6] ---------
//  INFO  advent_code_2021::advent::day_11 >    [6]-[0] energy_level_table: After STEP[6]-[0] ---------
//  INFO  advent_code_2021::advent::day_11 >     <0>:  810🥥🥥10
//  INFO  advent_code_2021::advent::day_11 >     <1>: 10 7 7 9🥥
//  INFO  advent_code_2021::advent::day_11 >     <2>: 🥥 7 5 8🥥
//  INFO  advent_code_2021::advent::day_11 >     <3>: 🥥 9 810🥥
//  INFO  advent_code_2021::advent::day_11 >     <4>: 10🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >    [6]-[1] energy_level_table: After STEP[6]-[1] ---------
//  INFO  advent_code_2021::advent::day_11 >     <0>: 10🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <1>: 🥥 9 9🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <2>: 🥥 8 710🥥
//  INFO  advent_code_2021::advent::day_11 >     <3>: 🥥10 9🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <4>: 🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >    [6]-[2] energy_level_table: After STEP[6]-[2] ---------
//  INFO  advent_code_2021::advent::day_11 >     <0>: 🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <1>: 🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <2>: 🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <3>: 🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <4>: 🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 > [6]🍏🍒 DISPLAY energy_level_table: After STEP[6] 🍏🍒 ---------
//  INFO  advent_code_2021::advent::day_11 >  [0]:  _ _ _ _ _
//  INFO  advent_code_2021::advent::day_11 >  [1]:  _ _ _ _ _
//  INFO  advent_code_2021::advent::day_11 >  [2]:  _ _ _ _ _
//  INFO  advent_code_2021::advent::day_11 >  [3]:  _ _ _ _ _
//  INFO  advent_code_2021::advent::day_11 >  [4]:  _ _ _ _ _
//  INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//  INFO  advent_code_2021::advent::day_11 > --- Day 11: Dumbo Octopus, Part Two ---
//  INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-sample-a.txt
//  INFO  advent_code_2021::advent::day_11 > Total Step: 500, used_step: 6
//  INFO  advent_code_2021::advent::day_11 > Total Flash Count: 34
//  INFO  advent_code_2021::advent::day_11 > -----------------------------------------

//-------------------------------------------------------
//-- For input/day_11-sample-b.txt
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//     INFO  advent_code_2021::advent::day_11 > --- Day 11: Dumbo Octopus, Part Two ---
//     INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-sample-b.txt
//     INFO  advent_code_2021::advent::day_11 > Total Step: 200, used_step: 195
//     INFO  advent_code_2021::advent::day_11 > Total Flash Count: 3125
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//---------------
// INFO  advent_code_2021::advent::day_11 > [195]: --->> UPDATE energy_level_table: step[195] ---------
//  INFO  advent_code_2021::advent::day_11 >    [195]-[0] energy_level_table: After STEP[195]-[0] ---------
//  INFO  advent_code_2021::advent::day_11 >     <0>: 10🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <1>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <2>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <3>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <4>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <5>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <6>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <7>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <8>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <9>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >    [195]-[1] energy_level_table: After STEP[195]-[1] ---------
//  INFO  advent_code_2021::advent::day_11 >     <0>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <1>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <2>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <3>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <4>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <5>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <6>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <7>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <8>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 >     <9>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
//  INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//  INFO  advent_code_2021::advent::day_11 > --- Day 11: Dumbo Octopus, Part Two ---
//  INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-sample-b.txt
//  INFO  advent_code_2021::advent::day_11 > Total Step: 500, used_step: 195
//  INFO  advent_code_2021::advent::day_11 > Total Flash Count: 3125
//  INFO  advent_code_2021::advent::day_11 > -----------------------------------------

//-------------------------------------------------------
//-- For input/day_11-input.txt
//     INFO  advent_code_2021::advent::day_11 > -----------------------------------------
//     INFO  advent_code_2021::advent::day_11 > --- Day 11: Dumbo Octopus, Part Two ---
//     INFO  advent_code_2021::advent::day_11 > Input File: input/day_11-input.txt
//     INFO  advent_code_2021::advent::day_11 > Total Step: 500, used_step: 354
//     INFO  advent_code_2021::advent::day_11 > Total Flash Count: 5607
//     INFO  advent_code_2021::advent::day_11 > ----------------------------------------
//---------------
// INFO  advent_code_2021::advent::day_11 > [354]: --->> UPDATE energy_level_table: step[354] ---------
// INFO  advent_code_2021::advent::day_11 >    [354]-[0] energy_level_table: After STEP[354]-[0] ---------
// INFO  advent_code_2021::advent::day_11 >     <0>: 11🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <1>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <2>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <3>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <4>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <5>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <6>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <7>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <8>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <9>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >    [354]-[1] energy_level_table: After STEP[354]-[1] ---------
// INFO  advent_code_2021::advent::day_11 >     <0>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <1>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <2>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <3>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <4>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <5>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <6>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <7>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <8>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 >     <9>: 🥥🥥🥥🥥🥥🥥🥥🥥🥥🥥
// INFO  advent_code_2021::advent::day_11 > -----------------------------------------

//-- energy level of each octopus
// The energy level of each octopus is a value betweeon 0 and 9.
// You can model the energy levels and flashes of light in steps.
// During a single step following occurs:

fn handle_input(filename: &str) -> Vec<Vec<u32>> {
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

    let input_lines = lines
        .iter()
        .map(|line| {
            let energy_levels = line
                .chars()
                .into_iter()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            info!("{:?}", energy_levels);
            energy_levels
        })
        .collect::<Vec<Vec<_>>>();

    input_lines
}

fn make_energy_level_table(input_lines: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let lines_count = input_lines.len();
    let line_len = input_lines[0].len();
    let mut energy_level_table: Vec<Vec<u32>> = Vec::with_capacity(lines_count);

    //-- chunks_table vector of vector
    input_lines.iter().enumerate().for_each(|(i, line)| {
        let v: Vec<u32> = Vec::with_capacity(line_len);
        energy_level_table.push(v);
        line.iter().enumerate().for_each(|(_j, item)| {
            energy_level_table[i].push(*item);
        });
        // info!("energy_level_table[{}](len:{}) = {:?}", i, energy_level_table[i].len(), energy_level_table[i]);
    });

    energy_level_table
}

fn display_energy_level_table(energy_level_table: &Vec<Vec<u32>>, step: u16) {
    if step > 10 {
        if step % 10 == 0 {
            info!("*[{step}] 🍏🍒 DISPLAY energy_level_table: After STEP[{step}] 🍏🍒 ");
            energy_level_table.iter().enumerate().for_each(|(i, line)| {
                let mut format_str = format!("   [{i}]: ");
                line.iter().enumerate().for_each(|(j, item)| {
                    if *item == 0 {
                        format_str += &*format!(" _");
                    } else {
                        format_str += &*format!("{:2}", energy_level_table[i][j]);
                    }
                });
                info!("  {}", format_str);
            });
        }
    } else {
        info!("*[{step}] 🍏🍒 DISPLAY energy_level_table: After STEP[{step}] 🍏🍒 ");
        energy_level_table.iter().enumerate().for_each(|(i, line)| {
            let mut format_str = format!("   [{i}]: ");
            line.iter().enumerate().for_each(|(j, item)| {
                if *item == 0 {
                    format_str += &*format!(" _");
                } else {
                    format_str += &*format!("{:2}", energy_level_table[i][j]);
                }
            });
            info!("  {}", format_str);
        });
    }
}

fn display_internal_energy_level_table(
    energy_level_table: &Vec<Vec<u32>>,
    step: u16,
    loop_count: &u16,
) {
    debug!("        ({loop_count} /{step}) 🥝 internal state: After STEP[{step}]-[{loop_count}] ");
    energy_level_table.iter().enumerate().for_each(|(i, line)| {
        let mut format_str = format!("          <{i}>: ");
        line.iter().enumerate().for_each(|(j, item)| {
            if *item == 0 {
                // format_str += &*format!("🥥");
                format_str += &*format!(" -");
            } else {
                format_str += &*format!("{:2}", energy_level_table[i][j]);
            }
        });
        debug!("    {}", format_str);
    });
}

fn update_energy_level(
    mut energy_level_table: Vec<Vec<u32>>,
    total_flash_count: &mut i32,
    step: u16,
) -> Vec<Vec<u32>> {
    let lines_count = energy_level_table.len();
    let line_len = energy_level_table[0].len();

    info!(
        "[{}]: ----->> ┣┓ UPDATE energy_level_table: step[{}] ┏┥ <<-----",
        step, step
    );
    // info!("[{}]: ----->> UPDATE energy_level_table: step[{}] ┣┓너┏♨❤♨┑나┏┥ <<-----", step, step);
    //-------------------------------------------------------------------
    //-- First: increase by 1 for each energy level
    let mut target_level_table = Vec::with_capacity(lines_count);
    let mut flash_status_table = Vec::with_capacity(lines_count);
    energy_level_table
        .iter_mut()
        .enumerate()
        .for_each(|(i, line)| {
            let values: Vec<u32> = Vec::with_capacity(line_len);
            let status: Vec<bool> = Vec::with_capacity(line_len);
            target_level_table.push(values);
            flash_status_table.push(status);
            line.iter_mut().enumerate().for_each(|(_j, item)| {
                *item += 1;
                target_level_table[i].push(*item);
                flash_status_table[i].push(false);
            });
        });

    //-------------------------------------------------------------------
    //-- Second:
    // check if energy level > 9, then set to true its status
    let mut again = true;
    let mut loop_count = 0;
    while again {
        for i in 0..lines_count {
            for j in 0..line_len {
                if target_level_table[i][j] > 9 && flash_status_table[i][j] != true {
                    // if target_level[i][j] > 9, just set to true its status value.
                    // do not update its energy level value yet because its energy level value will be
                    // updated after this "for i loop" exits
                    flash_status_table[i][j] = true;
                    *total_flash_count += 1;
                    // top-line
                    if i == 0 {
                        // left column (0, 0)
                        if j == 0 {
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                        }
                        // middle columns (0, 1..n-2)
                        else if 1 <= j && j <= line_len - 2 {
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                        }
                        // right column (0, n-1)
                        else {
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                        }
                    } else if 1 <= i && i <= lines_count - 2 {
                        // left column (1..n-2, 0)
                        if j == 0 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                        }
                        // middle columns (1..n-2, 1..n-2)
                        else if 1 <= j && j <= line_len - 2 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                        // right column (1..n-2, n-1)
                        else {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                    } else {
                        // left column (n-1, 0)
                        if j == 0 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                        }
                        // middle columns (n-1, 1..n-2)
                        else if 1 <= j && j <= line_len - 2 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                        // right column (n-1, n-1)
                        else {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                    }
                }
            }
        }

        let mut one_more = false;
        for i in 0..lines_count {
            for j in 0..line_len {
                //-- set to zero its value once a flash has happend
                if flash_status_table[i][j] == true {
                    target_level_table[i][j] = 0;
                }
                //-- check if new flashes happened so that continue this while-loop once again
                if target_level_table[i][j] > 9 {
                    one_more = true;
                }
            }
        }

        display_internal_energy_level_table(&target_level_table, step, &loop_count);
        again = one_more;
        loop_count += 1;
    }

    target_level_table
}

fn update_energy_level_until_all(
    mut energy_level_table: Vec<Vec<u32>>,
    total_flash_count: &mut i32,
    step: u16,
) -> (Vec<Vec<u32>>, bool) {
    let lines_count = energy_level_table.len();
    let line_len = energy_level_table[0].len();

    info!(
        "[{}]: ----->> ┣┓ UPDATE energy_level_table: step[{}] ┏┥ <<-----",
        step, step
    );
    // info!("[{}]: ----->> UPDATE energy_level_table: step[{}] ┣┓웃┏♨❤♨┑유┏┥ <<-----", step, step);
    //-------------------------------------------------------------------
    //-- First: increase by 1 for each energy level
    let mut target_level_table = Vec::with_capacity(lines_count);
    let mut flash_status_table = Vec::with_capacity(lines_count);
    energy_level_table
        .iter_mut()
        .enumerate()
        .for_each(|(i, line)| {
            let values: Vec<u32> = Vec::with_capacity(line_len);
            let status: Vec<bool> = Vec::with_capacity(line_len);
            target_level_table.push(values.clone());
            flash_status_table.push(status.clone());
            line.iter_mut().enumerate().for_each(|(_j, item)| {
                *item += 1;
                target_level_table[i].push(*item);
                flash_status_table[i].push(false);
            });
        });

    //-------------------------------------------------------------------
    //-- Second:
    // check if energy level > 9, then set to true its status
    let mut again = true;
    let mut loop_count = 0;
    while again {
        for i in 0..lines_count {
            for j in 0..line_len {
                if target_level_table[i][j] > 9 && flash_status_table[i][j] != true {
                    // if target_level[i][j] > 9, just set to true its status value.
                    // do not update its energy level value yet because its energy level value will be
                    // updated after this "for i loop" exits
                    flash_status_table[i][j] = true;
                    *total_flash_count += 1;

                    // top-line (row)
                    if i == 0 {
                        // left-most column (0, 0) (left-top corner)
                        if j == 0 {
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                        }
                        // middle columns (0, 1..n-2)
                        else if 1 <= j && j <= line_len - 2 {
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                        }
                        // right-most column (0, n-1) (right-top corner)
                        else {
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                        }
                    }
                    // middle rows
                    else if 1 <= i && i <= lines_count - 2 {
                        // left-most column (1..n-2, 0)
                        if j == 0 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                        }
                        // middle columns (1..n-2, 1..n-2)
                        else if 1 <= j && j <= line_len - 2 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i + 1][j + 1] += 1;
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                        // right-most column (1..n-2, n-1)
                        else {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i + 1][j] += 1;
                            target_level_table[i + 1][j - 1] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                    }
                    // bottom row
                    else {
                        // left-most column (n-1, 0) (left-bottom corner)
                        if j == 0 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                        }
                        // middle columns (n-1, 1..n-2)
                        else if 1 <= j && j <= line_len - 2 {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i - 1][j + 1] += 1;
                            target_level_table[i][j + 1] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                        // right-most column (n-1, n-1) (right-bottom corner)
                        else {
                            target_level_table[i - 1][j] += 1;
                            target_level_table[i][j - 1] += 1;
                            target_level_table[i - 1][j - 1] += 1;
                        }
                    }
                }
            }
        }

        let mut one_more = false;
        for i in 0..lines_count {
            for j in 0..line_len {
                //-- set to zero its value once a flash has happend
                if flash_status_table[i][j] == true {
                    target_level_table[i][j] = 0;
                }
                //-- set to true if new flashes happened so that continue this while-loop once again
                if target_level_table[i][j] > 9 {
                    one_more = true;
                }
            }
        }

        display_internal_energy_level_table(&target_level_table, step, &loop_count);

        // condition for loop exit.
        let mut all_asyncd = true;
        for i in 0..lines_count {
            for j in 0..line_len {
                if flash_status_table[i][j] == false {
                    all_asyncd = false;
                }
            }
        }

        if all_asyncd == true {
            display_internal_energy_level_table(&target_level_table, step, &loop_count);
            return (target_level_table, true);
        }

        again = one_more;
        loop_count += 1;
    }

    (target_level_table, false)
}
