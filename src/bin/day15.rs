// advent of code 2021 in RUST - Day 01
use pretty_env_logger;

#[allow(unused_imports)]
use aoc_2021_rust::utils;

use aoc_2021_rust::advent::day15;

fn main() {
    pretty_env_logger::init();

    //-- Day 15: Chiton --
    // day15::day_15::do_day_15();
    // day15::day_15::do_day15_part_one();
    // day15::day_15::do_day15_part_one_faster();
    // day15::day_15::do_day15_part_two();
    // day15::day_15::do_day15_part_two_faster();
    day15::day_15::do_day15_part_two_faster_async();
    // day15::day_15_uncle::do_day_15();
}

// run
// RUST_LOG=info cargo run
// /Volumes/SSD_01/zorba/fun/rust-lang-study/advent_code/aoc_2021_rust 
//  time RUST_LOG=info cargo run --bin day02
