// advent of code 2021 in RUST - Day 01
use pretty_env_logger;

#[allow(unused_imports)]
use aoc_2021_rust::utils;

use aoc_2021_rust::advent::day05;

fn main() {
    pretty_env_logger::init();

    day05::day_5::do_day_5();
}

// run
// RUST_LOG=info cargo run
// /Volumes/SSD_01/zorba/fun/rust-lang-study/advent_code/aoc_2021_rust 
//  time RUST_LOG=info cargo run --bin day02
