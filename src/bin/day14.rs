// advent of code 2021 in RUST - Day 01
use pretty_env_logger;

#[allow(unused_imports)]
use aoc_2021_rust::utils;

use aoc_2021_rust::advent::day14;

fn main() {
    pretty_env_logger::init();

    //-- Day 14: Extended Polymerization ---
    //---## Most Hard ##---
    // day14::day_14::do_day_14();
    //-- day14::day_14_2nd::do_day_14();
    //-- day14::day_14_3rd::do_day_14();
    // -- day14::day_14_4th::do_day_14();
    // day14::day_14_5th_first::do_day_14();
    day14::day_14_6th::do_day_14();
    // day14::day_14_7th_uncle::day14();
}

// run
// RUST_LOG=info cargo run
// /Volumes/SSD_01/zorba/fun/rust-lang-study/advent_code/aoc_2021_rust 
//  time RUST_LOG=info cargo run --bin day02
