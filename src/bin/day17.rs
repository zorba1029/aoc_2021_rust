// advent of code 2021 in RUST - Day 01
use pretty_env_logger;

#[allow(unused_imports)]
use aoc_2021_rust::utils;

use aoc_2021_rust::advent::day17;

fn main() {
    pretty_env_logger::init();

    //-- Day 17: Trick Shot
    //-- "Advent of Code”에서 “sleigh keys”는 문제를 해결하기 위한 중요한 단서를 나타내는 경우가 많죠.
    day17::day_17::do_day_17();
}

// run
// RUST_LOG=info cargo run
// /Volumes/SSD_01/zorba/fun/rust-lang-study/advent_code/aoc_2021_rust 
//  time RUST_LOG=info cargo run --bin day02
