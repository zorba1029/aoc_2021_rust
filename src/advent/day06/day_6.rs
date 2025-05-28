// advent/day_6.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::usize;

////////////////////////////////
//--- Day 6: Lanternfish ---

//-------------------------------------------------------------------------

pub fn do_day_6() {
    day_6_part_one();
    day_6_part_two();
}

fn handle_input(filename: &str) -> Vec<usize> {
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
    println!("[ ] Initial State: {}", first_line);

    // 3,4,3,1,2
    let init_fishes: Vec<usize> = first_line
        .split(',')
        // .map(|a| a.to_string().parse().unwrap())
        .map(|a| a.parse::<usize>().unwrap())
        .collect();

    init_fishes
}

// 1) Input fish list: [3,4,3,1,2]
// 2) Output: [0, 1, 1, 2, 1, 0, 0, 0, 0, 0], index: i-th day (0~9)
//   <index>   0  1  2  3  4  5  6  7  8  9, -- where [9] is NOT used
fn make_fish_table(input_list: Vec<usize>) -> Vec<u64> {
    let mut result_table = vec![0; 10];
    input_list.iter().for_each(|v| result_table[*v] += 1);
    result_table
}

fn get_fish_count_total(fish_table: &mut Vec<u64>, days: i32) -> u64 {
    (1..=days).for_each(|_d| {
        let new_fish = fish_table[0];
        (1..9).for_each(|i| fish_table[i - 1] = fish_table[i]);
        fish_table[8] = new_fish;
        fish_table[6] += new_fish;
    });
    println!("[ ] final fish_table: {:?}", fish_table);
    fish_table.iter().sum::<u64>()
}

fn day_6_part_one() {
    println!("//////////////////////////////////////////////");
    println!("--- Day 6: Lanternfish, Part One ---, 1/9/2022 ==> DONE");
    println!("-----------------------------------------------");
    let filename = "input/day_6-sample.txt";
    // let filename = "input/day_6-input.txt";
    let init_fishes: Vec<usize> = handle_input(filename);

    let fish_table = make_fish_table(init_fishes);
    println!("[ ] init fish_table: {:?}", fish_table);
    println!("-----------------------------------------------");

    let days = 18;
    let mut tmp_fish_tbl = fish_table.clone();
    let total_sum = get_fish_count_total(&mut tmp_fish_tbl, days);
    println!("[*] Sum (for {} days): {}", days, total_sum);
    println!("-----------------------------------------------");

    let days = 80;
    let mut tmp_fish_tbl = fish_table.clone();
    let total_sum = get_fish_count_total(&mut tmp_fish_tbl, days);
    println!("[*] Sum (for {} days): {}", days, total_sum);
    println!("-----------------------------------------------");

    let days = 256;
    let mut tmp_fish_tbl = fish_table.clone();
    let total_sum = get_fish_count_total(&mut tmp_fish_tbl, days);
    println!("[*] Sum (for {} days): {}", days, total_sum);
    println!("-----------------------------------------------");
}

// [*] Input Filename: input/day_6-sample.txt
// [*] input lines count = 1
// [ ] Initial State: 3,4,3,1,2
// [ ] init fish_table: [0, 1, 1, 2, 1, 0, 0, 0, 0, 0]
// [*] final fish_table: [3, 5, 3, 2, 2, 1, 5, 1, 4, 0]
// [**] Sum (for 18 days): 26
// [*] final fish_table: [424, 729, 558, 790, 739, 762, 991, 370, 571, 0]
// [**] Sum (for 80 days): 5934
// [*] final fish_table: [2376852196, 2731163883, 2897294544, 3164316379, 3541830408, 3681986557, 4275812629, 1985489551, 2329711392, 0]
// [**] Sum (for 256 days): 26984457539

fn day_6_part_two() {
    println!("//////////////////////////////////////////////");
    println!("--- Day 6: Lanternfish, Part Two ---, 1/9/2022 ==> DONE");
    println!("-----------------------------------------------");
    // let filename = "input/day_6-sample.txt";
    let filename = "input/day_6-input.txt";
    let init_fishes: Vec<usize> = handle_input(filename);

    let fish_table = make_fish_table(init_fishes);
    println!("[ ] init fish_table: {:?}", fish_table);
    println!("-----------------------------------------------");

    let days = 80;
    let mut tmp_fish_tbl = fish_table.clone();
    let total_sum = get_fish_count_total(&mut tmp_fish_tbl, days);
    println!("[*] Sum (for {} days): {}", days, total_sum);
    println!("-----------------------------------------------");

    let days = 256;
    let mut tmp_fish_tbl = fish_table.clone();
    let total_sum = get_fish_count_total(&mut tmp_fish_tbl, days);
    println!("[*] Sum (for {} days): {}", days, total_sum);
    println!("-----------------------------------------------");
}

// [*] Input Filename: input/day_6-input.txt
// [*] input lines count = 1
// [ ] Initial State: 1,1,1,1,1,1,1,4,1,2,1,1,4,1,1,1,5,1,1,1,1,1,1,1,1,1,1,1,1,5,1,1,1,1,3,1,1,2,1,2,1,3,3,4,1,4,1,1,3,1,1,5,1,1,1,1,4,1,1,5,1,1,1,4,1,5,1,1,1,3,1,1,5,3,1,1,1,1,1,4,1,1,1,1,1,2,4,1,1,1,1,4,1,2,2,1,1,1,3,1,2,5,1,4,1,1,1,3,1,1,4,1,1,1,1,1,1,1,4,1,1,4,1,1,1,1,1,1,1,2,1,1,5,1,1,1,4,1,1,5,1,1,5,3,3,5,3,1,1,1,4,1,1,1,1,1,1,5,3,1,2,1,1,1,4,1,3,1,5,1,1,2,1,1,1,1,1,5,1,1,1,1,1,2,1,1,1,1,4,3,2,1,2,4,1,3,1,5,1,2,1,4,1,1,1,1,1,3,1,4,1,1,1,1,3,1,3,3,1,4,3,4,1,1,1,1,5,1,3,3,2,5,3,1,1,3,1,3,1,1,1,1,4,1,1,1,1,3,1,5,1,1,1,4,4,1,1,5,5,2,4,5,1,1,1,1,5,1,1,2,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1,1,5,1,1,1,1,1,1,3,1,1,2,1,1
// [ ] init fish_table: [0, 205, 19, 27, 26, 23, 0, 0, 0, 0]
// [*] final fish_table: [14141, 61432, 23119, 56268, 46339, 40862, 81152, 16201, 50212, 0]
// [**] Sum (for 80 days): 389726
// [*] final fish_table: [144736007595, 181435600984, 185574699634, 199674488787, 237842569956, 222918744459, 292760381752, 118624554229, 159768944646, 0]
// [**] Sum (for 256 days): 1743335992042

fn day_6_part_two_old() {
    println!("//////////////////////////////////////////////");
    println!("--- Day 6: Lanternfish, Part Two ---, 1/9/2022 ==> DONE");
    println!("-----------------------------------------------");
    // let filename = "input/day_6-sample.txt";
    let filename = "input/day_6-input.txt";
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
    println!("Initial State: {}", first_line);
    let init_fishes = first_line
        .split(',')
        .map(|a| a.to_string().parse::<i8>().unwrap())
        .collect::<Vec<_>>();

    // let days = 80;
    let days = 256;
    let mut init_list: Vec<i8> = init_fishes.clone();

    for day in 1..=days {
        let mut reset_count = 0;
        let mut new_list = init_list.clone();
        for (i, fish) in init_list.iter().enumerate() {
            let mut reset = false;
            if *fish == 0 {
                reset = true;
                new_list[i] = 6;
            } else {
                new_list[i] = *fish - 1;
            };

            if reset {
                reset_count += 1;
            }
        }

        (0..reset_count).for_each(|_| new_list.push(8));
        print!("After day[{:2}]: ", day);
        // for v in new_list.iter() {
        //     print!("{},", v);
        // }
        println!(" -- Total Fishes: {}", new_list.len());
        init_list = new_list;
    }
}
