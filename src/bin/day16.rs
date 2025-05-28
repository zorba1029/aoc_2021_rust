// advent of code 2021 in RUST - Day 01
use pretty_env_logger;

#[allow(unused_imports)]
use aoc_2021_rust::utils;

use aoc_2021_rust::advent::day16;

fn main() {
    pretty_env_logger::init();

    //-- Day 16: Packet Decoder -- Aug 9, 2024 ~ Aug 14, 2024
    // day16::day_16::do_day_16();
    // day16::day_16::do_day16_part_one();
    day16::day_16::do_day16_part_two();
}

// run
// RUST_LOG=info cargo run
// /Volumes/SSD_01/zorba/fun/rust-lang-study/advent_code/aoc_2021_rust 
//  time RUST_LOG=info cargo run --bin day02
