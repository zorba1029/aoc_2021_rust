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
                .map(|c| c.to_digit(10).unwrap() as u32)
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

// INFO  aoc_2021_rust::advent::day11::day_11 > ===============================================
// INFO  aoc_2021_rust::advent::day11::day_11 > --- Day 11: Dumbo Octopus, Part One ---, 1/23/2022 ==> DONE
// INFO  aoc_2021_rust::advent::day11::day_11 > ===============================================
// INFO  aoc_2021_rust::advent::day11::day_11 > [*] Input Filename: input/day_11-input.txt
// INFO  aoc_2021_rust::advent::day11::day_11 > [*] input lines count = 10
// INFO  aoc_2021_rust::advent::day11::day_11 > [ ] First Line: len=10, 5723573158,
// INFO  aoc_2021_rust::advent::day11::day_11 > [5, 7, 2, 3, 5, 7, 3, 1, 5, 8]
// INFO  aoc_2021_rust::advent::day11::day_11 > [3, 1, 5, 4, 7, 4, 8, 5, 6, 3]
// INFO  aoc_2021_rust::advent::day11::day_11 > [4, 7, 8, 3, 5, 1, 4, 8, 7, 8]
// INFO  aoc_2021_rust::advent::day11::day_11 > [3, 8, 4, 8, 1, 4, 2, 3, 7, 5]
// INFO  aoc_2021_rust::advent::day11::day_11 > [3, 6, 3, 7, 7, 2, 4, 1, 5, 1]
// INFO  aoc_2021_rust::advent::day11::day_11 > [8, 5, 8, 3, 1, 7, 2, 4, 8, 4]
// INFO  aoc_2021_rust::advent::day11::day_11 > [7, 7, 4, 7, 4, 4, 4, 1, 8, 4]
// INFO  aoc_2021_rust::advent::day11::day_11 > [1, 6, 1, 3, 3, 6, 7, 8, 8, 2]
// INFO  aoc_2021_rust::advent::day11::day_11 > [6, 2, 2, 8, 6, 1, 4, 2, 2, 7]
// INFO  aoc_2021_rust::advent::day11::day_11 > [4, 7, 3, 2, 2, 2, 5, 3, 3, 4]
// INFO  aoc_2021_rust::advent::day11::day_11 > input_lines: 10
// INFO  aoc_2021_rust::advent::day11::day_11 > *[0] 🍏🍒 DISPLAY energy_level_table: After STEP[0] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  5 7 2 3 5 7 3 1 5 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  3 1 5 4 7 4 8 5 6 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  4 7 8 3 5 1 4 8 7 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  3 8 4 8 1 4 2 3 7 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  3 6 3 7 7 2 4 1 5 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  8 5 8 3 1 7 2 4 8 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  7 7 4 7 4 4 4 1 8 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  1 6 1 3 3 6 7 8 8 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  6 2 2 8 6 1 4 2 2 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  4 7 3 2 2 2 5 3 3 4
// INFO  aoc_2021_rust::advent::day11::day_11 > [1]: ----->> ┣┓ UPDATE energy_level_table: step[1] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[1] 🍏🍒 DISPLAY energy_level_table: After STEP[1] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  6 8 3 4 6 8 4 2 6 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  4 2 6 5 8 5 9 6 7 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  5 8 9 4 6 2 5 9 8 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  4 9 5 9 2 5 3 4 8 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  4 7 4 8 8 3 5 2 6 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  9 6 9 4 2 8 3 5 9 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  8 8 5 8 5 5 5 2 9 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  2 7 2 4 4 7 8 9 9 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  7 3 3 9 7 2 5 3 3 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  5 8 4 3 3 3 6 4 4 5
// INFO  aoc_2021_rust::advent::day11::day_11 > [2]: ----->> ┣┓ UPDATE energy_level_table: step[2] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[2] 🍏🍒 DISPLAY energy_level_table: After STEP[2] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  7 9 4 6 9 _ 8 7 _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  6 5 9 8 _ 9 _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  8 _ _ 9 9 5 9 _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  8 _ _ _ 6 7 5 9 _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  9 _ _ _ _ 6 7 6 _ 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  _ _ _ _ 8 _ 5 9 _ 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  _ _ _ _ 9 8 9 8 _ 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  6 _ 8 8 7 9 _ _ _ 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  9 5 6 _ 9 4 8 7 7 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  6 9 6 5 5 4 7 5 6 7
// INFO  aoc_2021_rust::advent::day11::day_11 > [3]: ----->> ┣┓ UPDATE energy_level_table: step[3] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[3] 🍏🍒 DISPLAY energy_level_table: After STEP[3] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  9 _ 9 _ _ 4 _ 9 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  9 9 _ _ 8 _ 5 3 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  _ 4 4 _ _ _ _ 4 2 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  _ 4 2 4 _ _ _ _ 3 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  _ 3 1 3 5 _ _ _ 5 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  2 2 1 3 _ 8 _ _ 6 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  1 2 3 6 _ _ _ _ 5 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  9 5 _ _ _ _ 6 4 3 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  _ _ _ 7 _ 9 _ 9 8 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  _ _ _ 9 7 7 9 7 7 8
// INFO  aoc_2021_rust::advent::day11::day_11 > [4]: ----->> ┣┓ UPDATE energy_level_table: step[4] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[4] 🍏🍒 DISPLAY energy_level_table: After STEP[4] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  _ 5 _ 2 1 5 2 _ 3 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  _ _ 3 2 9 1 7 5 3 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  3 7 6 1 1 1 1 5 3 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  1 5 3 5 1 1 1 1 5 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  1 4 2 4 6 1 1 1 7 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  3 3 2 4 1 9 1 1 8 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  3 4 4 7 1 1 1 1 7 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  _ 7 2 2 3 2 9 7 7 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  2 2 3 _ 6 _ 6 _ _ 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  1 1 3 _ _ _ _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 > [5]: ----->> ┣┓ UPDATE energy_level_table: step[5] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[5] 🍏🍒 DISPLAY energy_level_table: After STEP[5] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  1 6 1 4 3 7 3 1 4 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  1 1 4 4 _ 3 8 6 4 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  4 8 7 3 3 3 2 6 4 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  2 6 4 6 2 2 2 2 6 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  2 5 3 5 8 3 3 2 8 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  4 4 3 5 3 _ 3 2 9 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  4 5 5 8 3 4 4 3 8 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  1 8 3 3 4 4 _ 9 8 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  3 3 4 1 7 2 8 2 1 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  2 2 4 1 1 1 1 1 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 > [6]: ----->> ┣┓ UPDATE energy_level_table: step[6] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[6] 🍏🍒 DISPLAY energy_level_table: After STEP[6] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  2 7 2 5 4 8 4 2 5 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  2 2 5 5 1 4 9 7 5 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  5 9 8 4 4 4 3 7 5 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  3 7 5 7 3 3 3 4 8 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  3 6 4 6 9 4 4 5 _ 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  5 5 4 6 4 1 4 6 _ 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  5 6 6 9 4 5 6 8 _ 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  2 9 4 4 5 6 3 _ _ 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  4 4 5 2 8 4 _ 6 4 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  3 3 5 2 2 3 3 3 2 2
// INFO  aoc_2021_rust::advent::day11::day_11 > [7]: ----->> ┣┓ UPDATE energy_level_table: step[7] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[7] 🍏🍒 DISPLAY energy_level_table: After STEP[7] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  3 8 3 6 6 _ 7 4 6 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  4 5 8 7 3 7 _ 9 6 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  8 _ _ 8 6 6 5 9 6 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  7 _ _ _ 7 5 4 5 9 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  7 _ _ _ _ 6 5 6 1 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  9 _ _ _ 9 3 5 7 1 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  9 _ _ _ 7 6 7 9 1 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  5 _ 9 7 7 7 4 1 2 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  6 6 7 3 9 5 1 7 6 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  4 4 6 3 3 4 4 4 4 4
// INFO  aoc_2021_rust::advent::day11::day_11 > [8]: ----->> ┣┓ UPDATE energy_level_table: step[8] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[8] 🍏🍒 DISPLAY energy_level_table: After STEP[8] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  5 _ 7 9 9 3 _ 8 9 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  6 8 _ _ 9 _ 7 _ _ 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  9 2 4 _ _ _ _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  8 1 2 4 _ _ _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  9 2 1 3 6 _ _ _ 7 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  _ 3 1 3 _ _ _ _ 5 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  _ 4 3 6 _ _ _ _ 4 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  7 4 _ _ _ _ 9 4 4 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  7 9 _ 9 _ 9 3 8 7 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  5 6 8 6 5 6 5 5 5 5
// INFO  aoc_2021_rust::advent::day11::day_11 > [9]: ----->> ┣┓ UPDATE energy_level_table: step[9] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[9] 🍏🍒 DISPLAY energy_level_table: After STEP[9] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  7 3 _ _ _ 6 2 _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  9 _ 4 5 _ 3 9 3 5 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  _ 6 6 2 2 2 1 1 2 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  _ 5 3 5 1 1 1 2 3 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  _ 5 2 4 7 1 1 3 _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  2 5 2 4 1 1 1 3 _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  1 5 4 7 1 2 2 3 8 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  9 6 3 2 3 3 _ 7 7 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  9 _ 4 _ 3 _ 7 _ 9 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  7 9 _ 9 8 8 8 7 7 6
// INFO  aoc_2021_rust::advent::day11::day_11 > [10]: ----->> ┣┓ UPDATE energy_level_table: step[10] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[10] 🍏🍒 DISPLAY energy_level_table: After STEP[10] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  9 5 1 1 1 8 4 2 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  _ 2 5 6 1 5 _ 5 6 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  2 8 7 3 3 4 3 3 3 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  1 6 4 6 2 2 2 3 4 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  1 6 3 5 8 2 2 4 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  3 6 3 5 2 2 2 5 2 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  3 7 5 8 2 3 4 7 _ 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  _ 9 4 3 4 5 3 _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  _ 5 7 3 7 5 _ 8 _ 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  _ _ 3 _ _ _ _ _ _ 9
// INFO  aoc_2021_rust::advent::day11::day_11 > [11]: ----->> ┣┓ UPDATE energy_level_table: step[11] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [12]: ----->> ┣┓ UPDATE energy_level_table: step[12] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [13]: ----->> ┣┓ UPDATE energy_level_table: step[13] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [14]: ----->> ┣┓ UPDATE energy_level_table: step[14] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [15]: ----->> ┣┓ UPDATE energy_level_table: step[15] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [16]: ----->> ┣┓ UPDATE energy_level_table: step[16] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [17]: ----->> ┣┓ UPDATE energy_level_table: step[17] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [18]: ----->> ┣┓ UPDATE energy_level_table: step[18] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [19]: ----->> ┣┓ UPDATE energy_level_table: step[19] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [20]: ----->> ┣┓ UPDATE energy_level_table: step[20] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[20] 🍏🍒 DISPLAY energy_level_table: After STEP[20] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  4 2 9 8 9 7 _ _ 7 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  9 4 4 5 4 _ _ _ _ 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  8 3 3 6 _ _ _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  8 3 3 5 _ _ _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  9 4 3 5 8 _ _ _ _ 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  1 5 3 5 3 4 5 6 5 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  2 7 5 7 2 2 3 5 _ 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  _ 2 6 2 2 2 3 _ _ 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  6 8 9 3 2 2 3 9 _ 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  4 5 1 7 7 7 6 6 7 3
// INFO  aoc_2021_rust::advent::day11::day_11 > [21]: ----->> ┣┓ UPDATE energy_level_table: step[21] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [22]: ----->> ┣┓ UPDATE energy_level_table: step[22] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [23]: ----->> ┣┓ UPDATE energy_level_table: step[23] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [24]: ----->> ┣┓ UPDATE energy_level_table: step[24] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [25]: ----->> ┣┓ UPDATE energy_level_table: step[25] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [26]: ----->> ┣┓ UPDATE energy_level_table: step[26] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [27]: ----->> ┣┓ UPDATE energy_level_table: step[27] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [28]: ----->> ┣┓ UPDATE energy_level_table: step[28] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [29]: ----->> ┣┓ UPDATE energy_level_table: step[29] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [30]: ----->> ┣┓ UPDATE energy_level_table: step[30] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[30] 🍏🍒 DISPLAY energy_level_table: After STEP[30] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  9 _ 4 3 3 3 5 5 _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  4 4 4 3 3 7 5 5 7 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  3 3 3 3 8 5 4 4 5 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  3 3 3 3 8 5 4 4 5 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  3 3 3 4 5 8 7 7 8 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  9 3 3 5 2 2 2 2 6 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  8 3 4 7 2 2 3 4 4 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  9 6 7 3 2 2 4 1 2 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  1 1 1 4 2 2 4 1 2 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  1 1 1 4 2 2 3 4 5 9
// INFO  aoc_2021_rust::advent::day11::day_11 > [31]: ----->> ┣┓ UPDATE energy_level_table: step[31] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [32]: ----->> ┣┓ UPDATE energy_level_table: step[32] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [33]: ----->> ┣┓ UPDATE energy_level_table: step[33] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [34]: ----->> ┣┓ UPDATE energy_level_table: step[34] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [35]: ----->> ┣┓ UPDATE energy_level_table: step[35] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [36]: ----->> ┣┓ UPDATE energy_level_table: step[36] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [37]: ----->> ┣┓ UPDATE energy_level_table: step[37] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [38]: ----->> ┣┓ UPDATE energy_level_table: step[38] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [39]: ----->> ┣┓ UPDATE energy_level_table: step[39] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [40]: ----->> ┣┓ UPDATE energy_level_table: step[40] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[40] 🍏🍒 DISPLAY energy_level_table: After STEP[40] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  2 6 7 7 9 2 2 2 6 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  8 8 8 9 2 2 2 2 2 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  6 7 9 2 2 2 2 2 2 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  7 9 2 3 3 3 2 2 3 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  9 2 2 3 _ 3 2 3 5 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  5 2 2 9 5 5 3 3 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  4 5 6 8 _ _ 6 6 8 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  4 5 7 _ _ _ _ _ 5 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  7 2 5 _ _ _ _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  5 7 _ _ _ _ _ _ _ 5
// INFO  aoc_2021_rust::advent::day11::day_11 > [41]: ----->> ┣┓ UPDATE energy_level_table: step[41] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [42]: ----->> ┣┓ UPDATE energy_level_table: step[42] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [43]: ----->> ┣┓ UPDATE energy_level_table: step[43] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [44]: ----->> ┣┓ UPDATE energy_level_table: step[44] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [45]: ----->> ┣┓ UPDATE energy_level_table: step[45] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [46]: ----->> ┣┓ UPDATE energy_level_table: step[46] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [47]: ----->> ┣┓ UPDATE energy_level_table: step[47] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [48]: ----->> ┣┓ UPDATE energy_level_table: step[48] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [49]: ----->> ┣┓ UPDATE energy_level_table: step[49] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [50]: ----->> ┣┓ UPDATE energy_level_table: step[50] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[50] 🍏🍒 DISPLAY energy_level_table: After STEP[50] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  8 2 2 3 5 _ 9 7 6 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  2 2 2 4 _ _ _ 9 7 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  2 2 2 5 _ _ _ _ 9 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  2 2 2 5 _ _ _ _ _ 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  3 3 3 6 7 _ _ _ _ 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  1 6 6 _ _ _ _ _ 5 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  1 3 _ _ _ _ 9 9 _ 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  2 4 _ _ _ 7 6 3 6 9
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  _ 5 8 _ 7 5 5 5 5 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  _ _ 8 6 5 4 4 4 4 8
// INFO  aoc_2021_rust::advent::day11::day_11 > [51]: ----->> ┣┓ UPDATE energy_level_table: step[51] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [52]: ----->> ┣┓ UPDATE energy_level_table: step[52] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [53]: ----->> ┣┓ UPDATE energy_level_table: step[53] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [54]: ----->> ┣┓ UPDATE energy_level_table: step[54] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [55]: ----->> ┣┓ UPDATE energy_level_table: step[55] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [56]: ----->> ┣┓ UPDATE energy_level_table: step[56] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [57]: ----->> ┣┓ UPDATE energy_level_table: step[57] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [58]: ----->> ┣┓ UPDATE energy_level_table: step[58] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [59]: ----->> ┣┓ UPDATE energy_level_table: step[59] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [60]: ----->> ┣┓ UPDATE energy_level_table: step[60] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[60] 🍏🍒 DISPLAY energy_level_table: After STEP[60] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  1 6 5 6 8 5 2 2 2 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  6 6 6 8 5 5 7 2 2 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  5 6 8 3 6 4 5 7 2 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  6 8 3 3 8 5 4 5 8 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  7 3 4 5 6 7 5 4 7 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  8 4 6 _ _ _ 6 5 8 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  8 5 _ _ _ _ 9 8 4 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  9 5 _ _ _ 5 3 8 1 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  3 4 7 _ 5 3 2 4 6 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  3 3 5 3 3 2 2 3 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 > [61]: ----->> ┣┓ UPDATE energy_level_table: step[61] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [62]: ----->> ┣┓ UPDATE energy_level_table: step[62] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [63]: ----->> ┣┓ UPDATE energy_level_table: step[63] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [64]: ----->> ┣┓ UPDATE energy_level_table: step[64] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [65]: ----->> ┣┓ UPDATE energy_level_table: step[65] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [66]: ----->> ┣┓ UPDATE energy_level_table: step[66] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [67]: ----->> ┣┓ UPDATE energy_level_table: step[67] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [68]: ----->> ┣┓ UPDATE energy_level_table: step[68] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [69]: ----->> ┣┓ UPDATE energy_level_table: step[69] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [70]: ----->> ┣┓ UPDATE energy_level_table: step[70] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[70] 🍏🍒 DISPLAY energy_level_table: After STEP[70] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  7 1 1 1 1 3 8 6 5 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  1 1 1 1 5 3 2 8 6 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  1 1 1 5 3 2 2 2 8 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  1 1 5 3 2 2 2 2 3 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  1 5 4 4 5 4 3 3 5 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  6 4 5 _ _ _ 6 6 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  4 7 _ _ _ _ 5 2 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  4 7 _ _ _ _ _ 2 8 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  _ 8 _ _ _ _ 7 8 6 7
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  9 _ 6 _ _ _ _ _ 7 5
// INFO  aoc_2021_rust::advent::day11::day_11 > [71]: ----->> ┣┓ UPDATE energy_level_table: step[71] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [72]: ----->> ┣┓ UPDATE energy_level_table: step[72] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [73]: ----->> ┣┓ UPDATE energy_level_table: step[73] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [74]: ----->> ┣┓ UPDATE energy_level_table: step[74] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [75]: ----->> ┣┓ UPDATE energy_level_table: step[75] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [76]: ----->> ┣┓ UPDATE energy_level_table: step[76] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [77]: ----->> ┣┓ UPDATE energy_level_table: step[77] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [78]: ----->> ┣┓ UPDATE energy_level_table: step[78] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [79]: ----->> ┣┓ UPDATE energy_level_table: step[79] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [80]: ----->> ┣┓ UPDATE energy_level_table: step[80] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[80] 🍏🍒 DISPLAY energy_level_table: After STEP[80] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  _ 5 4 5 7 _ 4 2 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  5 5 5 7 _ _ _ 4 2 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  4 5 7 _ _ _ _ _ 4 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  5 7 4 _ _ _ _ _ _ 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  7 3 3 5 _ _ _ _ _ 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  1 4 2 8 8 _ _ _ 9 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  1 9 6 5 5 8 4 3 9 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  1 6 4 4 4 6 1 5 6 8
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  6 7 5 5 4 5 7 6 7 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  2 6 2 5 4 4 5 7 1 1
// INFO  aoc_2021_rust::advent::day11::day_11 > [81]: ----->> ┣┓ UPDATE energy_level_table: step[81] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [82]: ----->> ┣┓ UPDATE energy_level_table: step[82] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [83]: ----->> ┣┓ UPDATE energy_level_table: step[83] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [84]: ----->> ┣┓ UPDATE energy_level_table: step[84] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [85]: ----->> ┣┓ UPDATE energy_level_table: step[85] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [86]: ----->> ┣┓ UPDATE energy_level_table: step[86] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [87]: ----->> ┣┓ UPDATE energy_level_table: step[87] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [88]: ----->> ┣┓ UPDATE energy_level_table: step[88] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [89]: ----->> ┣┓ UPDATE energy_level_table: step[89] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [90]: ----->> ┣┓ UPDATE energy_level_table: step[90] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[90] 🍏🍒 DISPLAY energy_level_table: After STEP[90] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  6 _ _ _ _ 5 4 4 4 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  _ _ _ _ 7 5 4 4 4 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  _ _ _ 8 5 4 4 4 4 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  _ _ 4 8 5 4 4 4 5 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  _ 4 5 3 7 5 4 5 7 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  9 5 4 2 2 8 7 8 3 3
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  8 _ 3 2 2 2 2 6 4 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  9 3 3 2 2 2 2 7 _ 4
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  2 2 2 2 2 2 2 8 9 6
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  8 2 2 2 2 2 2 2 6 4
// INFO  aoc_2021_rust::advent::day11::day_11 > [91]: ----->> ┣┓ UPDATE energy_level_table: step[91] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [92]: ----->> ┣┓ UPDATE energy_level_table: step[92] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [93]: ----->> ┣┓ UPDATE energy_level_table: step[93] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [94]: ----->> ┣┓ UPDATE energy_level_table: step[94] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [95]: ----->> ┣┓ UPDATE energy_level_table: step[95] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [96]: ----->> ┣┓ UPDATE energy_level_table: step[96] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [97]: ----->> ┣┓ UPDATE energy_level_table: step[97] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [98]: ----->> ┣┓ UPDATE energy_level_table: step[98] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [99]: ----->> ┣┓ UPDATE energy_level_table: step[99] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > [100]: ----->> ┣┓ UPDATE energy_level_table: step[100] ┏┥ <<-----
// INFO  aoc_2021_rust::advent::day11::day_11 > *[100] 🍏🍒 DISPLAY energy_level_table: After STEP[100] 🍏🍒
// INFO  aoc_2021_rust::advent::day11::day_11 >      [0]:  9 3 3 4 6 3 _ _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [1]:  3 3 4 6 2 3 5 _ _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [2]:  3 4 6 2 2 2 3 5 _ _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [3]:  4 6 2 2 2 2 2 3 6 _
// INFO  aoc_2021_rust::advent::day11::day_11 >      [4]:  5 2 3 4 4 3 2 3 6 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [5]:  6 4 6 _ _ 5 4 5 2 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [6]:  6 4 _ _ _ _ 7 _ 3 1
// INFO  aoc_2021_rust::advent::day11::day_11 >      [7]:  7 _ _ _ _ _ _ _ 9 2
// INFO  aoc_2021_rust::advent::day11::day_11 >      [8]:  _ _ _ _ _ _ _ _ _ 5
// INFO  aoc_2021_rust::advent::day11::day_11 >      [9]:  4 _ _ _ _ _ _ _ 6 _
// INFO  aoc_2021_rust::advent::day11::day_11 > -----------------------------------------
// INFO  aoc_2021_rust::advent::day11::day_11 > 🟠 --- Day 11: Dumbo Octopus, 🟠 Part One ---
// INFO  aoc_2021_rust::advent::day11::day_11 > Input File: input/day_11-input.txt
// INFO  aoc_2021_rust::advent::day11::day_11 > Total Step: 100
// INFO  aoc_2021_rust::advent::day11::day_11 > 🟢 Total Flash Count: 1785
// INFO  aoc_2021_rust::advent::day11::day_11 > -----------------------------------------
// RUST_LOG=info cargo run --bin day11  0.06s user 0.03s system 19% cpu 0.444 total