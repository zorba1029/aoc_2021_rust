// Day 14: Extended Polymerization — count visualization (animated, in-terminal).
//
//   cargo run --release --bin day14_anim                  # sample-a, 40 steps
//   cargo run --release --bin day14_anim -- input 40      # the real puzzle
//   cargo run --release --bin day14_anim -- sample-a 10   # the Part 1 point
//
// arg1: which input  -> sample-a | input   (default: sample-a)
// arg2: max steps     (default: 40)

use aoc_2021_rust::advent::day14::animate;

fn main() {
    let mut args = std::env::args().skip(1);

    let which = args.next().unwrap_or_else(|| "sample-a".to_string());
    let steps: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(40);

    let filename = match which.as_str() {
        "sample-a" | "a" => "input/day_14-sample-a.txt",
        "input" => "input/day_14-input.txt",
        other => other, // allow passing a raw path
    };

    animate::run(filename, steps);
}
