// advent/day_1.rs
//
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn do_day_1() {
    day_1_part_one();
    day_1_part_two();
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

//--- Day 1: Sonar Sweep ---
fn day_1_part_one() {
    println!("--- Day 1: Sonar Sweep, Part One ---");
    let filename = "input/day_1-input.txt";
    let input_lines = handle_input(filename);
    println!("input_lines: {:?}", input_lines.len());

    let lines_count = input_lines.len();
    println!("input lines count = {}", lines_count);

    //-- using for loop
    {
        let nums = input_lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut count = 0;
        for i in nums.windows(2) {
            if i[0] < i[1] {
                count += 1;
            }
        }

        println!("part 1-A: total count = {}", count);
        // Right Answer for part 1: 1713
    }

    //-- use functional style: fold(), filter()
    {
        let nums = input_lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let count1 = nums
            .windows(2)
            .fold(0, |acc, item| if item[0] < item[1] { acc + 1 } else { acc });
        println!("  part 1-B: total count = {}", count1);

        let count2 = nums.windows(2).filter(|item| item[0] < item[1]).count();

        println!("  part 1-C: total count = {}", count2);
    }
}

fn day_1_part_two() {
    println!("--- Day 1: Sonar Sweep : Part Two ---");
    let filename = "input/day_1-input.txt";
    let input_lines = handle_input(filename);
    println!("input_lines: {:?}", input_lines.len());

    let lines_count = input_lines.len();
    println!("input lines count = {}", lines_count);

    //-- part 2-A:
    {
        let nums = input_lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let count = nums
            .windows(4)
            // .filter(|item| {
            //     let part1 = item[0] + item[1] + item[2];
            //     let part2 = item[1] + item[2] + item[3];
            //     part1 < part2
            // })
            //--> same as the above filter: item[1] + item[2] are common in the two statements
            .filter(|item| item[0] < item[3])
            .count();

        println!("part 2-A: total count = {}", count);
        // right answer for part 2: 1734
    }

    //-- part 2-B:
    {
        let count = input_lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .windows(4)
            .filter(|item| item[0] < item[3])
            .count();

        println!("part 2-B: total count = {}", count);
        // right answer for part 2: 1734
    }
}
