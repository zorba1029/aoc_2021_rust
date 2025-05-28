// advent/day_4.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

////////////////////////////////
//--- Day 4: Giant Squid ---
//

use crate::advent::day04::board::Board;

//-------------------------------------------------------------------------

pub fn do_day_4() {
    day_4_part_one();
    day_4_part_two();
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

fn day_4_part_one() {
    println!("//////////////////////////////////////////////");
    println!("--- Day 4: Giant Squid, part one ---");
    println!("-----------------------------------------------");
    // let filename = "input/day_4-sample.txt";
    let filename = "input/day_4-input.txt";
    let input_lines = handle_input(filename);

    let draw_numbers = input_lines
        .first()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|item| item.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("[*] draw_numbers: {:?}", draw_numbers);

    let mut board_list: Vec<Board> = Vec::new();
    let mut row_index = 0;
    let mut new_board = None;
    let row_count = 5;
    for (_i, line) in input_lines.iter().skip(1).enumerate() {
        if line.len() < 2 {
            // println!(" input board line count {} : ---------------------", i);
            new_board = Some(Board::new(5, 5));
            row_index = 0;

            continue;
        }
        // println!("line: {:?}", line);

        match new_board {
            Some(ref mut board) => {
                for (column, value) in line.split_whitespace().enumerate() {
                    // println!("column: {}, value: {}", column, value);
                    let num = value.parse::<i32>().unwrap();
                    board.set_value_xy(row_index, column, num);
                }
                row_index += 1;
                // println!("board: {:?}", board.values);
            }
            _ => {
                println!(" board:-- Nothing Matched");
            }
        }

        let t_board = new_board.clone();
        if row_index >= row_count {
            board_list.push(t_board.unwrap());
        }
    }

    //-- debug print
    // for (i, b) in board_list.iter().enumerate() {
    //     println!("board {}: {:?}", i, b.values);
    //     println!("board {}: {:?}", i, b.marks);
    // }

    //-----------------------------------------
    println!("-----------------------------------------");

    let mut find_bingo = false;
    for draw_num in draw_numbers.iter() {
        for (board_index, board) in board_list.iter_mut().enumerate() {
            match board.find_value(*draw_num) {
                (true, i) => {
                    board.set_mark(i, *draw_num);
                    match board.is_bingo_row_col() {
                        (true, row_index) => {
                            println!("=========================================");
                            println!("[*] board {}: {:?}", board_index, board.values);
                            println!("[*] board {}: {:?}", board_index, board.marks);
                            println!("----------------------");
                            //--print_board_values(&board);
                            board.print_board_values();
                            println!("----------------------");
                            //-- print_board_marks(&board);
                            board.print_board_marks();
                            println!("----------------------");

                            let sum_unmarked_values = board.get_sum_unmarked_values();
                            let final_score = sum_unmarked_values * (*draw_num);
                            find_bingo = true;

                            println!(
                                "[**] FIND BINGO: board {}, row: {}, draw_num: {}, BINGO_SUM: {} ",
                                board_index, row_index, *draw_num, sum_unmarked_values
                            );
                            println!(
                                "[**] FINAL SCORE: {} (= {} x {}) ",
                                final_score, sum_unmarked_values, *draw_num
                            );
                            println!("-------------------------------------\n");
                            break;
                        }
                        (false, _) => {}
                    }
                }
                (false, _) => {}
            }
        }

        if find_bingo {
            break;
        }
    }
}

//-- Part 1:
//-----------------------------------
// sample-input:
// BINGO_SUM: 188
// [**] FIND BINGO: board 2, row: 0, draw_num: 24, BINGO_SUM: 188
// [**] FINAL SCORE: 4512

//-----------------------------------
//-- input: input/day_4-input.txt
// BINGO_SUM: 658
// [**] FIND BINGO: board 37, row: 2, draw_num: 76, BINGO_SUM: 658
// [**] FINAL SCORE: 50008

fn day_4_part_two() {
    println!("//////////////////////////////////////////////");
    println!("--- Day 4: Giant Squid, part two ---");
    println!("-----------------------------------------------");
    // let filename = "input/day_4-sample.txt";
    let filename = "input/day_4-input.txt";
    let input_lines = handle_input(filename);

    let draw_numbers = input_lines
        .first()
        .iter()
        .flat_map(|line| line.split(','))
        .map(|item| item.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("[*] draw_numbers: {:?}", draw_numbers);

    let mut board_list: Vec<Board> = Vec::new();
    let mut row_index = 0;
    let mut new_board = None;
    let row_count = 5;
    for (_i, line) in input_lines.iter().skip(1).enumerate() {
        if line.len() < 2 {
            // println!(" input board line count {} : ---------------------", i);
            new_board = Some(Board::new(5, 5));
            row_index = 0;

            continue;
        }
        // println!("line: {:?}", line);

        match new_board {
            Some(ref mut board) => {
                for (column, value) in line.split_whitespace().enumerate() {
                    // println!("column: {}, value: {}", column, value);
                    let num = value.parse::<i32>().unwrap();
                    board.set_value_xy(row_index, column, num);
                }
                row_index += 1;
                // println!("board: {:?}", board.values);
            }
            _ => {
                println!("  board:-- Nothing Matched");
            }
        }

        let t_board = new_board.clone();
        if row_index >= row_count {
            board_list.push(t_board.unwrap());
        }
    }

    //-----------------------------------------
    println!("-----------------------------------------");
    let mut bingo_board_log: Vec<usize> = Vec::new();
    let mut bingo_board_log_tuple: Vec<(usize, i32)> = Vec::new();
    let mut find_bingo = false;
    for draw_num in draw_numbers.iter() {
        for (board_index, board) in board_list.iter_mut().enumerate() {
            if bingo_board_log.contains(&board_index) {
                continue;
            }
            match board.find_value(*draw_num) {
                (true, i) => {
                    board.set_mark(i, *draw_num);
                    match board.is_bingo_row_col() {
                        (true, _row_index) => {
                            find_bingo = true;
                            bingo_board_log.push(board_index);
                            bingo_board_log_tuple.push((board_index, *draw_num));
                        }
                        (false, _) => {}
                    }
                }
                (false, _) => {}
            }
        }
    }

    println!("=========================================");
    let (last_win_board_index, draw_num) = bingo_board_log_tuple.last().unwrap();
    println!(
        "[*] Last Win Board: {}, draw_num: {}",
        last_win_board_index, draw_num
    );
    let last_win_board = &board_list[*last_win_board_index];
    println!("----------------------");
    //-- print_board_values(last_win_board);
    last_win_board.print_board_values();
    println!("----------------------");
    //-- print_board_marks(last_win_board);
    last_win_board.print_board_marks();
    println!("----------------------");
    let sum_unmarked_values = last_win_board.get_sum_unmarked_values();
    let final_score = sum_unmarked_values * (*draw_num);
    println!(
        "[**] FINAL SCORE: {} (= {} x {}) ",
        final_score, sum_unmarked_values, *draw_num
    );
    println!("-------------------------------------\n");
}

//-- Part 2:
//-----------------------------------
//-- input: input/day_4-input.txt
// =========================================
// Last Win Board: 28, draw_num: 68
// ----------------------
//  13  31  91   1   .
//  96  35  20   .  40
//   .  27  78   9  22
//  11   .   .  46  51
//  72  68  23  25  85
// ----------------------
//   T   T   T   T   .
//   T   T   T   .   T
//   .   T   T   T   T
//   T   .   .   T   T
//   T   T   T   T   T
// ----------------------
// BINGO_SUM: 256
// [**] FINAL SCORE: 17408 (= 256 x 68)

//
//-- Your puzzle answer was 17408
//

//-----------------------------------
// sample-input:
// =========================================
// [*] Last Win Board: 1, draw_num: 13
// ----------------------
//   .   .   0   2   .
//   9   .  13  17   5
//   .   .   7   .  23
//   .  11  10  24   4
//  14  21  16   .   .
// ----------------------
//   .   .   T   T   .
//   T   .   T   T   T
//   .   .   T   .   T
//   .   T   T   T   T
//   T   T   T   .   .
// ----------------------
// BINGO_SUM: 148
// [**] FINAL SCORE: 1924 (= 148 x 13)
// -------------------------------------

// //////////////////////////////////////////////
// --- Day 4: Giant Squid, part one ---
// -----------------------------------------------
// [*] Input Filename: input/day_4-input.txt
// [*] input lines count = 601
// [*] draw_numbers: [25, 8, 32, 53, 22, 94, 55, 80, 33, 4, 63, 14, 60, 95, 31, 89, 30, 5, 47, 66, 84, 70, 17, 74, 99, 82, 21, 35, 64, 2, 76, 9, 90, 56, 78, 28, 51, 86, 49, 98, 29, 96, 23, 58, 52, 75, 41, 50, 13, 72, 92, 83, 62, 37, 18, 11, 34, 71, 91, 85, 27, 12, 24, 73, 7, 77, 10, 93, 15, 61, 3, 46, 16, 97, 1, 57, 65, 40, 0, 48, 69, 6, 20, 68, 19, 45, 42, 79, 88, 44, 26, 38, 36, 54, 81, 59, 43, 87, 39, 67]
// -----------------------------------------
// =========================================
// [*] board 37: [38, 27, 60, 37, 44, 98, 9, 13, 45, 57, 4, 76, 33, 8, 21, 19, 7, 77, 50, 22, 71, 35, 80, 46, 20]
// [*] board 37: [false, false, true, false, false, false, false, false, false, false, true, true, true, true, true, false, false, false, false, true, false, true, true, false, false]
// ----------------------
//   .   .  60   .   .
//   .   .   .   .   .
//   4  76  33   8  21
//   .   .   .   .  22
//   .  35  80   .   .
// ----------------------
//   .   .   T   .   .
//   .   .   .   .   .
//   T   T   T   T   T
//   .   .   .   .   T
//   .   T   T   .   .
// ----------------------
// BINGO_SUM: 658
// [**] FIND BINGO: board 37, row: 2, draw_num: 76, BINGO_SUM: 658
// [**] FINAL SCORE: 50008 (= 658 x 76)
// -------------------------------------
//
// //////////////////////////////////////////////
// --- Day 4: Giant Squid, part two ---
// -----------------------------------------------
// [*] Input Filename: input/day_4-input.txt
// [*] input lines count = 601
// [*] draw_numbers: [25, 8, 32, 53, 22, 94, 55, 80, 33, 4, 63, 14, 60, 95, 31, 89, 30, 5, 47, 66, 84, 70, 17, 74, 99, 82, 21, 35, 64, 2, 76, 9, 90, 56, 78, 28, 51, 86, 49, 98, 29, 96, 23, 58, 52, 75, 41, 50, 13, 72, 92, 83, 62, 37, 18, 11, 34, 71, 91, 85, 27, 12, 24, 73, 7, 77, 10, 93, 15, 61, 3, 46, 16, 97, 1, 57, 65, 40, 0, 48, 69, 6, 20, 68, 19, 45, 42, 79, 88, 44, 26, 38, 36, 54, 81, 59, 43, 87, 39, 67]
// -----------------------------------------
// =========================================
// [*] Last Win Board: 28, draw_num: 68
// ----------------------
//  13  31  91   1   .
//  96  35  20   .  40
//   .  27  78   9  22
//  11   .   .  46  51
//  72  68  23  25  85
// ----------------------
//   T   T   T   T   .
//   T   T   T   .   T
//   .   T   T   T   T
//   T   .   .   T   T
//   T   T   T   T   T
// ----------------------
// BINGO_SUM: 256
// [**] FINAL SCORE: 17408 (= 256 x 68)
// -------------------------------------
