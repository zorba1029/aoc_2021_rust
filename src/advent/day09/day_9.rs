// advent/day_9.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

///////////////////////////////////////////
//--- Day 9: Smoke Basin ---
///////////////////////////////////////////

pub fn do_day_9() {
    day_9_part_one();
    day_9_part_two();
}

fn day_9_part_one() {
    println!("===============================================");
    println!("--- Day 9: Smoke Basin, Part One ---, 1/16/2022 ==> START & DONE");
    println!("===============================================");
    // let filename = "input/day_9-sample.txt";
    let filename = "input/day_9-input.txt";
    let input_lines = handle_input(filename);
    println!("input_lines: {:?}", input_lines.len());

    let heightmap = make_heightmap(&input_lines);
    let low_points_map = find_lowpoints(&heightmap);
    let sum_risk_level = sum_of_risk_level(&heightmap, &low_points_map);

    println!("-----------------------------------------");
    println!("Sum Of Risk Level: {}", sum_risk_level);
    println!("-----------------------------------------");
}

// ===============================================
// --- Day 9: Smoke Basin, Part One ---, 1/16/2022 ==> START & DONE
// ===============================================
// [*] Input Filename: input/day_9-sample.txt
// [*] input lines count = 5
// [ ] First Line: 2199943210, len=10
//  [2, 1, 9, 9, 9, 4, 3, 2, 1, 0]
//  [3, 9, 8, 7, 8, 9, 4, 9, 2, 1]
//  [9, 8, 5, 6, 7, 8, 9, 8, 9, 2]
//  [8, 7, 6, 7, 8, 9, 6, 7, 8, 9]
//  [9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
// input_lines: 5
//  test: heightmap[i][j]  ---------
//  2 1 9 9 9 4 3 2 1 0
//  3 9 8 7 8 9 4 9 2 1
//  9 8 5 6 7 8 9 8 9 2
//  8 7 6 7 8 9 6 7 8 9
//  9 8 9 9 9 6 5 6 7 8
//  test: low_points_map[i][j]  ---------
//  . T . . . . . . . T
//  . . . . . . . . . .
//  . . T . . . . . . .
//  . . . . . . . . . .
//  . . . . . . T . . .
//   sum_of_risk_level: 15
//----------------------------------------
// Sum Of Risk Level: 15
//----------------------------------------

// ===============================================
// --- Day 9: Smoke Basic, Part One ---, 1/16/2022 ==> START & DONE
// ===============================================
// [*] Input Filename: input/day_9-input.txt
// [*] input lines count = 100
// [ ] First Line: 3566789567953212679875689976651013679329876404568999884568910249798689921989789990134578923557899767, len=100
// ....
// ...
//  sum_of_risk_level: 566
//-----------------------------------------
// Sum Of Risk Level: 566
//-----------------------------------------

//-- need to find the largest basins so you know what areas are most important to avoid
// Basin is all locations that eventually flow downward to a single low point.
// Locations of height 9 do not count as being in any basin, and
// all other locations will always be part of exactly one basin.
//   Size of a basin is the number of locations within the basin, including
// the low point.

fn day_9_part_two() {
    println!("===============================================");
    println!("--- Day 9: Smoke Basic, Part Two ---, 1/20/2022 ==> DONE!!");
    println!("===============================================");
    // let filename = "input/day_9-sample.txt";
    let filename = "input/day_9-input.txt";
    let input_lines = handle_input(filename);
    println!("input_lines len: {:?}", input_lines.len());

    let mut heightmap = make_heightmap(&input_lines);
    let low_points_map = find_lowpoints(&heightmap);

    let basin_size_list = find_all_basins(&mut heightmap, &low_points_map);
    println!(
        "basin_size_list(len={}): {:?}",
        basin_size_list.len(),
        basin_size_list
    );

    let mut sorted_list_desc = basin_size_list.clone();
    sorted_list_desc.sort_by(|a, b| b.cmp(a));
    println!(
        "sorted_list_desc(len={}), first 3 items: {:?}",
        sorted_list_desc.len(),
        &sorted_list_desc[0..3]
    );

    let three_sum = sorted_list_desc
        .iter()
        .take(3)
        .enumerate()
        .fold(1, |acc, (i, v)| {
            println!("  [{}]: {}", i, v);
            acc * v
        });

    println!("-----------------------------------------");
    println!("Three Largest Basins Multiply: {:?}", three_sum);
    println!("-----------------------------------------");
}

// ===============================================
// --- Day 9: Smoke Basic, Part Two ---, 1/20/2022 ==> DONE!!
// ===============================================
// [*] Input Filename: input/day_9-sample.txt
// [*] input lines count = 5
// [ ] First Line: 2199943210, len=10
// input_lines len: 5
// DebugPrint: heightmap[i][j]  ---------
// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678
// -- find_basin[0][1]: ---->> basin_size: 3
// -- find_basin[0][9]: ---->> basin_size: 9
// -- find_basin[2][2]: ---->> basin_size: 14
// -- find_basin[4][6]: ---->> basin_size: 9
// basin_size_list(len=4): [3, 9, 14, 9]
// sorted_list_desc(len=4), first 3 items: [14, 9, 9]
// [0]: 14
// [1]: 9
// [2]: 9
// -----------------------------------------
// Three Largest Basins Multiply: 1134
// -----------------------------------------

//===============================================
// --- Day 9: Smoke Basic, Part Two ---, 1/20/2022 ==> DONE!!
// ===============================================
// [*] Input Filename: input/day_9-input.txt
// [*] input lines count = 100
// [ ] First Line: 359013.....23557899767, len=100
// input_lines len: 100
// -- find_basin[0][14]: ---->> basin_size: 27
// -- find_basin[0][31]: ---->> basin_size: 53
// -- find_basin[0][44]: ---->> basin_size: 27
// -- find_basin[0][54]: ---->> basin_size: 19
// ....
// -- find_basin[98][81]: ---->> basin_size: 26
// -- find_basin[98][87]: ---->> basin_size: 14
// -- find_basin[99][18]: ---->> basin_size: 13
// -- find_basin[99][59]: ---->> basin_size: 2
// basin_size_list(len=242): [27, 53, 27, 19, ...., 6, 26, 14, 13, 2]
// sorted_list_desc(len=242), first 3 items: [102, 94, 93]
//   [0]: 102
//   [1]: 94
//   [2]: 93
// -----------------------------------------
// Three Largest Basins Multiply: 891684
// -----------------------------------------

fn handle_input(filename: &str) -> Vec<Vec<u8>> {
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
    println!("[ ] First Line: {}, len={}", first_line, first_line.len());

    let input_lines = lines
        .iter()
        .map(|line| {
            let points = line
                .chars()
                .into_iter()
                // .map(|c| c.to_string().parse::<u8>().unwrap())
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>();
            // println!("{:?}", points);
            points
        })
        .collect::<Vec<Vec<_>>>();

    input_lines
}

fn make_heightmap(input_lines: &Vec<Vec<u8>>) -> Vec<Vec<i8>> {
    let lines_count = input_lines.len();
    let line_len = input_lines[0].len();
    let mut heightmap: Vec<Vec<i8>> = Vec::with_capacity(lines_count);

    //-- heightmap vector of vector
    input_lines.iter().enumerate().for_each(|(i, line)| {
        let v: Vec<i8> = Vec::with_capacity(line_len);
        heightmap.push(v);
        line.iter().enumerate().for_each(|(_j, item)| {
            heightmap[i].push(*item as i8);
        });
        // println!("heightmap[{}] = {:?}", i, heightmap[i]);
    });

    println!("DebugPrint: heightmap[i][j]  ---------");
    // heightmap.iter().enumerate().for_each(|(i, line)| {
    //     line.iter().enumerate().for_each(|(j, _item)| {
    //         print!(" {}", heightmap[i][j]);
    //     });
    //     println!();
    // });
    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            // print!(" {}", heightmap[i][j]);
            print!("{}", heightmap[i][j]);
        }
        println!();
    }

    heightmap
}

fn find_lowpoints(heightmap: &Vec<Vec<i8>>) -> Vec<Vec<bool>> {
    let lines_count = heightmap.len();
    let line_len = heightmap[0].len();
    let mut low_points_map: Vec<Vec<bool>> = Vec::with_capacity(lines_count);

    for i in 0..heightmap.len() {
        let v: Vec<bool> = Vec::with_capacity(line_len);
        low_points_map.push(v.clone());

        for j in 0..line_len {
            // let up = heightmap[i-1][j];
            // let left = heightmap[i][j-1];
            // let right = heightmap[i][j+1];
            // let down = heightmap[i+1][j];
            low_points_map[i].push(false);

            if i == 0 {
                // 1-st raw
                if j == 0 {
                    if heightmap[i][j] < heightmap[i][j+1] &&   // Right
                        heightmap[i][j] < heightmap[i+1][j]
                    {
                        // Down
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                } else if j == line_len - 1 {
                    if heightmap[i][j] < heightmap[i][j-1] &&   // Left
                        heightmap[i][j] < heightmap[i+1][j]
                    {
                        // Down
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                } else {
                    if heightmap[i][j] < heightmap[i][j+1] &&   // Right
                        heightmap[i][j] < heightmap[i+1][j] &&  // Down
                        heightmap[i][j] < heightmap[i][j-1]
                    {
                        // Left
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                }
            } else if i == lines_count - 1 {
                if j == 0 {
                    if heightmap[i][j] < heightmap[i][j+1] &&   // Right
                        heightmap[i][j] < heightmap[i-1][j]
                    {
                        // Up
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                } else if j == line_len - 1 {
                    if heightmap[i][j] < heightmap[i][j-1] &&   // Left
                        heightmap[i][j] < heightmap[i-1][j]
                    {
                        // Up
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                } else {
                    if heightmap[i][j] < heightmap[i][j+1] &&   // Right
                        heightmap[i][j] < heightmap[i-1][j] &&  // Up
                        heightmap[i][j] < heightmap[i][j-1]
                    {
                        // Left
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                }
            } else {
                if j == 0 {
                    if heightmap[i][j] < heightmap[i][j+1] &&   // Right
                        heightmap[i][j] < heightmap[i-1][j] &&  // Up
                        heightmap[i][j] < heightmap[i+1][j]
                    {
                        // Down
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                } else if j == line_len - 1 {
                    if heightmap[i][j] < heightmap[i][j-1] &&   // Left
                        heightmap[i][j] < heightmap[i-1][j] &&  // Up
                        heightmap[i][j] < heightmap[i+1][j]
                    {
                        // Down
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                } else {
                    if heightmap[i][j] < heightmap[i][j+1] &&   // Right
                        heightmap[i][j] < heightmap[i-1][j] &&  // Up
                        heightmap[i][j] < heightmap[i+1][j] &&  // Down
                        heightmap[i][j] < heightmap[i][j-1]
                    {
                        // Left
                        low_points_map[i][j] = true;
                    } else {
                        low_points_map[i][j] = false;
                    }
                }
            }
        }
    }

    low_points_map
}

fn sum_of_risk_level(heightmap: &Vec<Vec<i8>>, low_points_map: &Vec<Vec<bool>>) -> u32 {
    // let low_points_map =  make_lowpoints(heightmap);
    println!("[] test: low_points_map[i][j]  ---------");

    let mut sum_of_risk_level = 0u32;
    for i in 0..low_points_map.len() {
        for j in 0..low_points_map[0].len() {
            if low_points_map[i][j] == true {
                sum_of_risk_level += heightmap[i][j] as u32 + 1;
                // print!(" T");
                print!("T");
            } else {
                // print!(" .");
                print!(".");
            }
        }
        println!();
    }
    println!("[] sum_of_risk_level: {}", sum_of_risk_level);

    sum_of_risk_level
}

// A basin is all locations that eventually flow downward to a single low point.
// Therefore, every low point has a basin, although some basins are very small.
// The size of a basic is the number of locations within the basin, including
// the low point.
fn find_all_basins(heightmap: &mut Vec<Vec<i8>>, low_points_map: &Vec<Vec<bool>>) -> Vec<i32> {
    let mut basins_vec: Vec<i32> = Vec::new();

    for i in 0..low_points_map.len() {
        for j in 0..low_points_map[0].len() {
            if low_points_map[i][j] == true {
                print!("-- find_basin[{}][{}]: ----", i, j);
                let basin_size = find_basin_size(heightmap, i, j, 0);
                println!(">> basin_size: {} ", basin_size);
                basins_vec.push(basin_size);
            }
        }
    }
    // println!("-- [**] basin-count list = {:?} ", basins_vec);

    basins_vec
}

fn find_basin_size(heightmap: &mut Vec<Vec<i8>>, i: usize, j: usize, basin_size: i32) -> i32 {
    let mut basin_size = basin_size;
    // current
    if 0 <= heightmap[i][j] && heightmap[i][j] <= 8 {
        basin_size += 1;
        // println!("[*] [{}][{}]: {}, basin_size={}", i, j, heightmap[i][j], basin_size);
        heightmap[i][j] = -1;
    } else {
        return basin_size;
    }

    // UP
    if i >= 1 && 0 <= heightmap[i - 1][j] && heightmap[i - 1][j] <= 8 {
        // println!(" -- UP:  [{}][{}], basin_size: {}", i-1, j, basin_size);
        basin_size = find_basin_size(heightmap, i - 1, j, basin_size);
    }

    // RIGHT
    if j <= heightmap[0].len() - 2 && 0 <= heightmap[i][j + 1] && heightmap[i][j + 1] <= 8 {
        // println!(" -- RIGHT:  [{}][{}], basin_size: {}", i, j+1, basin_size);
        basin_size = find_basin_size(heightmap, i, j + 1, basin_size);
    }

    // DOWN
    if i <= heightmap.len() - 2 && 0 <= heightmap[i + 1][j] && heightmap[i + 1][j] <= 8 {
        // println!(" -- DOWN:  [{}][{}], basin_size: {}", i+1, j, basin_size);
        basin_size = find_basin_size(heightmap, i + 1, j, basin_size);
    }

    // LEFT
    if j >= 1 && 0 <= heightmap[i][j - 1] && heightmap[i][j - 1] <= 8 {
        // println!(" -- LEFT:  [{}][{}], basin_size: {}", i, j-1, count);
        basin_size = find_basin_size(heightmap, i, j - 1, basin_size);
    }

    basin_size
}
