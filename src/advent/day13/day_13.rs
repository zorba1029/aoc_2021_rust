// advent/day_13.rs
// use std::collections::HashMap;
// use std::collections::VecDeque;
use log::{debug, info};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn do_day_13() {
    day_13_part_one();
    day_13_part_two();
}

fn handle_input(filename: &str) -> (Vec<(u16, u16)>, u16, u16, Vec<(String, u16)>) {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    info!("[*] Input Filename: {}", filename);
    info!("[*] input lines count = {}", lines_count);

    // let first_line = &lines[0];
    // info!("[ ] First Line: len={}, {}, ", first_line.len(), first_line);

    let separator_index = lines
        .iter()
        .enumerate()
        .find(|(_i, line)| line.is_empty())
        .unwrap()
        .0;
    info!("[ ] input data separator index: {} ", separator_index);

    info!("[ ] input coordinates list - (x,y) list -------");
    let coordinates_list = lines
        .iter()
        .take(separator_index)
        .map(|line| line.split(',').collect::<Vec<_>>())
        .enumerate()
        .map(|(i, item)| {
            let x = item[0].trim().parse::<u16>().unwrap();
            let y = item[1].trim().parse::<u16>().unwrap();
            debug!("   [{:4}]: ({},{})", i, x, y);
            (x, y)
        })
        .collect::<Vec<(u16, u16)>>();
    // let max_x = coordinates_list.iter().map(|(x, _)| x).max().unwrap().clone();
    // let max_y = coordinates_list.iter().map(|(_, y)| y).max().unwrap().clone();
    let max_x = *coordinates_list.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *coordinates_list.iter().map(|(_, y)| y).max().unwrap();
    info!("[ ] (max-x, max-y) = ({},{})", max_x, max_y);

    info!("[ ] input instructions list - (x or y,value) list -------");
    let instruction_list = lines
        .iter()
        .skip(separator_index + 1)
        .enumerate()
        .map(|(i, line)| {
            let pos = line.rfind(' ').unwrap();
            // let fold_items = (&line[pos..]).split('=').collect::<Vec<_>>();
            let fold_items = line[pos..].split('=').collect::<Vec<_>>();
            let fold_key = fold_items[0].trim().to_owned();
            let fold_value = fold_items[1].trim().to_owned().parse::<u16>().unwrap();
            debug!(" [{}]- fold along: ({},{})", i, fold_key, fold_value);
            (fold_key, fold_value)
        })
        .collect::<Vec<(String, u16)>>();

    (coordinates_list, max_x, max_y, instruction_list)
}

pub fn day_13_part_one() {
    info!("===============================================");
    info!("--- Day 13: Transparent Origami,  Part One ---, 2/4/2022 (Feb, 4) ==> DONE");
    info!("===============================================");
    // let filename = "input/day_13-sample-a.txt";
    let filename = "input/day_13-input.txt";
    let (coordinates_list, max_x, max_y, instruction_list) = handle_input(filename);
    info!( "input_lines: coordinates_list len: {}", coordinates_list.len() );
    info!("input_lines: (max_x, max_y) = ({},{})", max_x, max_y);
    info!( "input_lines: instruction_list len: {}", instruction_list.len() );

    let mut xy_map = make_coordinates_matrix(&coordinates_list, &max_x, &max_y);

    // display_coord_matrix(&xy_map, xy_map.first().unwrap().len() as u16, xy_map.len() as u16);

    let loop_count = 1;
    let result_xy = fold_page(&mut xy_map, &instruction_list, loop_count);
    info!( "final fold_page: result (x,y) = ({},{})", result_xy.0, result_xy.1 );

    let visible_dots_count = count_visible_dots(&xy_map, &result_xy.0, &result_xy.1);

    info!("-----------------------------------------");
    info!("--- Day 13: Transparent Origami, Part One --- ");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!("[*] visible_dots_count: {}", visible_dots_count);
    info!("[*] loop_count: {}", loop_count);
    info!("-----------------------------------------");
}

//--------------------------------------------
// For input:  "input/day_13-sample-a.txt";
// -------- x-y coordi maps: --------------------
//        [012345678901 ---
//     0: [#.##..#..#.]
//     1: [#...#......]
//     2: [......#...#]
//     3: [#...#......]
//     4: [.#.#..#.###]
//     5: [...........]
//     6: [...........]
// -----------------------------------------
// --- Day 13: Transparent Origami, Part One ---
// [ ] Input File: input/day_13-sample-a.txt
// ------------------------------------------
// [*] visible_dots_count: 17
// [*] loop_count: 1
// ------------------------------------------

// -------- x-y coordi maps: --------------------
//         [012345678901 ---
//      0: [#####]
//      1: [#...#]
//      2: [#...#]
//      3: [#...#]
//      4: [#####]
//      5: [.....]
//      6: [.....]
//  final fold_page: result (x,y) = (5,7)
//  [ ] count_visible_dots() ---: x_len:5, y_len:7
//  -----------------------------------------
//  --- Day 13: Transparent Origami, Part One ---
//  [ ] Input File: input/day_13-sample-a.txt
//  ------------------------------------------
//  [*] visible_dots_count: 16
//  [*] loop_count: 2
//  ------------------------------------------

//--------------------------------------------
// For input:  "input/day_13-input.txt";
// Loop Count: 1
// -----------------------------------------
// --- Day 13: Transparent Origami, Part One ---
// [ ] Input File: input/day_13-input.txt
// ------------------------------------------
// [*] visible_dots_count: 712   [**] <-- Correct Answer for Part 1
// [*] loop_count: 1
// -----------------------------------------

// -----------------------------------------
// --- Day 13: Transparent Origami, Part One ---
// [ ] Input File: input/day_13-input.txt
// ------------------------------------------
// [*] visible_dots_count: 595
// [*] loop_count: 2
// -----------------------------------------

pub fn day_13_part_two() {
    info!("===============================================");
    info!("--- Day 13: Transparent Origami,  Part Two ---, 2/4/2022 (Feb, 5) ==> DONE");
    info!("===============================================");
    // let filename = "input/day_13-sample-a.txt";
    let filename = "input/day_13-input.txt";
    let (coordinates_list, max_x, max_y, instruction_list) = handle_input(filename);
    info!( "input_lines: coordinates_list len: {}", coordinates_list.len() );
    info!("input_lines: (max_x, max_y) = ({},{})", max_x, max_y);
    info!( "input_lines: instruction_list len: {}", instruction_list.len() );

    let mut xy_map = make_coordinates_matrix(&coordinates_list, &max_x, &max_y);

    // display_coord_matrix(&xy_map, xy_map.first().unwrap().len() as u16, xy_map.len() as u16);

    let result_xy = fold_page_all(&mut xy_map, &instruction_list);
    info!( "final fold_page: result (x,y) = ({},{})", result_xy.0, result_xy.1 );
    let loop_count = result_xy.2;

    let visible_dots_count = count_visible_dots(&xy_map, &result_xy.0, &result_xy.1);

    display_coord_matrix(&xy_map, result_xy.0 as u16, result_xy.1 as u16);

    info!("-----------------------------------------");
    info!("--- Day 13: Transparent Origami, Part Two --- ");
    info!("[ ] Input File: {}", filename);
    info!("------------------------------------------");
    info!("[*] visible_dots_count: {}", visible_dots_count);
    info!("[*] loop_count: {}", loop_count);
    info!("-----------------------------------------");
}

//--------------------------------------------
// For input:  "input/day_13-input.txt";
// -----------------------------------------
//  fold_page_all[1]: (x_len, y_len) = (655,895)
//  fold_page_all[2]: (x_len, y_len) = (655,447)
//  fold_page_all[3]: (x_len, y_len) = (327,447)
//  fold_page_all[4]: (x_len, y_len) = (327,223)
//  fold_page_all[5]: (x_len, y_len) = (163,223)
//  fold_page_all[6]: (x_len, y_len) = (163,111)
//  fold_page_all[7]: (x_len, y_len) = (81,111)
//  fold_page_all[8]: (x_len, y_len) = (81,55)
//  fold_page_all[9]: (x_len, y_len) = (40,55)
//  fold_page_all[10]: (x_len, y_len) = (40,27)
//  fold_page_all[11]: (x_len, y_len) = (40,13)
//  fold_page_all[12]: (x_len, y_len) = (40,6)
// final fold_page: result (x,y) = (40,6)
// [ ] count_visible_dots() ---: x_len:40, y_len:6
// -------- x-y coordi maps: --------------------------
// [**] Correct Answer : Eight(8) Capital Letters: BLHF JPJF  <-- ########
//------------------------------------------------------
//        [012345678901 ---       [**] 8 Capital Letters: BLHF JPJF
//     0: [###..#....#..#.####...##.###....##.####.]
//     1: [#..#.#....#..#.#.......#.#..#....#.#....]
//     2: [###..#....####.###.....#.#..#....#.###..]
//     3: [#..#.#....#..#.#.......#.###.....#.#....]
//     4: [#..#.#....#..#.#....#..#.#....#..#.#....]
//     5: [###..####.#..#.#.....##..#.....##..#....]
// -----------------------------------------
// --- Day 13: Transparent Origami, Part Two ---
// [ ] Input File: input/day_13-input.txt
// ------------------------------------------
// [*] visible_dots_count: 90
// [*] loop_count: 12
// -----------------------------------------

// fn make_coordinates_matrix(coord_list: &Vec<(u16,u16)>, max_x: &u16, max_y: &u16) -> Vec<Vec<u32>> {
fn make_coordinates_matrix(coord_list: &[(u16, u16)], max_x: &u16, max_y: &u16) -> Vec<Vec<u32>> {
    let x_size: usize = (*max_x + 1) as usize;
    let y_size: usize = (*max_y + 1) as usize;
    let mut xy_map = vec![vec![0u32; x_size]; y_size];

    coord_list.iter().for_each(|(x, y)| {
        let x = *x as usize;
        let y = *y as usize;
        debug!("   []: ({},{})", x, y);
        xy_map[y][x] = 1;
    });

    xy_map
}

// fn display_coord_matrix(xy_map: &Vec<Vec<u32>>, x_len: u16, y_len: u16) {
fn display_coord_matrix(xy_map: &[Vec<u32>], x_len: u16, y_len: u16) {
    info!("-------- x-y coordi maps: --------------------");
    info!("       [          1         2         3         4]");
    info!("       [01234567890123456789012345678901234567890]");
    for (j, line) in xy_map.iter().enumerate() {
        let mut format_str = format!("{:4}: [", j);
        for (i, v) in line.iter().enumerate() {
            if *v == 1 {
                format_str += &*format!("{:1}", '#');
            } else {
                format_str += &*format!("{:1}", '.');
            }
            if i >= x_len as usize - 1 {
                break;
            }
        }
        format_str += &*format!("]");
        debug!(" {} ", format_str);
        if j >= y_len as usize - 1 {
            break;
        }
    }
    //----------------------
    //-->>  BLHF JPJF
    //----------------------
}

fn fold_page(
    xy_map: &mut Vec<Vec<u32>>,
    fold_instructions: &Vec<(String, u16)>,
    loop_count: u16,
) -> (u16, u16) {
    let mut y_len: u16 = xy_map.len() as u16;
    let mut x_len: u16 = xy_map.first().unwrap().len() as u16;
    let mut count: u16 = 0;

    for (inst, v) in fold_instructions {
        count += 1;
        match (&inst[..], v) {
            ("x", value) => {
                let result = fold_left(xy_map, value, x_len, y_len);
                x_len = result.0;
                y_len = result.1;
            }
            ("y", value) => {
                let result = fold_up(xy_map, value, x_len, y_len);
                x_len = result.0;
                y_len = result.1;
            }
            (_, _) => {}
        }

        if count >= loop_count {
            break;
        }
    }

    display_coord_matrix(xy_map.as_ref(), x_len, y_len);
    (x_len, y_len)
}

fn fold_page_all(
    xy_map: &mut Vec<Vec<u32>>,
    fold_instructions: &Vec<(String, u16)>,
) -> (u16, u16, u16) {
    let mut y_len: u16 = xy_map.len() as u16;
    let mut x_len: u16 = xy_map.first().unwrap().len() as u16;
    let mut count: u16 = 0;

    for (inst, v) in fold_instructions {
        count += 1;
        match (&inst[..], v) {
            ("x", value) => {
                let result = fold_left(xy_map, value, x_len, y_len);
                x_len = result.0;
                y_len = result.1;
            }
            ("y", value) => {
                let result = fold_up(xy_map, value, x_len, y_len);
                x_len = result.0;
                y_len = result.1;
            }
            (_, _) => {}
        }
        info!(
            " fold_page_all[{}]: (x_len, y_len) = ({},{})",
            count, x_len, y_len
        );
    }

    // display_coord_matrix(xy_map.as_ref(), x_len, y_len);
    (x_len, y_len, count)
}

// fold line: y, horizontal y=... lines
// fn fold_up(xy_map: &mut Vec<Vec<u32>>, y: &u16, x_len: u16, y_len: u16) -> (u16, u16) {
fn fold_up(xy_map: &mut [Vec<u32>], y: &u16, x_len: u16, _y_len: u16) -> (u16, u16) {
    for row_line in 0..(*y as usize) {
        for column in 0..(x_len as usize) {
            if xy_map[row_line + *y as usize + 1][column] == 1 {
                xy_map[*y as usize - row_line - 1][column] = 1;
            }
        }
    }

    (x_len, *y)
}

// fold line: x, vertical x=... lines
// fn fold_left(xy_map: &mut Vec<Vec<u32>>, x: &u16, x_len: u16, y_len: u16) -> (u16, u16)  {
fn fold_left(xy_map: &mut [Vec<u32>], x: &u16, _x_len: u16, y_len: u16) -> (u16, u16) {
    for row_line in 0..(y_len as usize) {
        for column in 0..(*x as usize) {
            if xy_map[row_line][column + *x as usize + 1] == 1 {
                xy_map[row_line][*x as usize - column - 1] = 1;
            }
        }
    }

    (*x, y_len)
}

// fn count_visible_dots(xy_map: &Vec<Vec<u32>>, x_len: &u16, y_len: &u16) -> u32 {
fn count_visible_dots(xy_map: &[Vec<u32>], x_len: &u16, y_len: &u16) -> u32 {
    info!( "[ ] count_visible_dots() ---: x_len:{}, y_len:{}", x_len, y_len );
    let mut total_sum: u32 = 0;
    for i in 0..(*y_len as usize) {
        let line = xy_map.get(i).unwrap();
        let sum = line[0..(*x_len as usize)].iter().sum::<u32>() as u32;
        // info!("[ ] count_visible_dots({}): line sum: {}  ---:", i, sum);
        total_sum += sum;
    }

    total_sum
}
