// Day 18: Snailfish — reduction visualization (animated, in-terminal).
//
//   cargo run --release --bin day18_anim                  # sample-1, all lines
//   cargo run --release --bin day18_anim -- sample-2      # the other sample
//   cargo run --release --bin day18_anim -- input 5       # real input, first 5 lines
//
// arg1: which input  -> sample-1 | sample-2 | input   (default: sample-1)
// arg2: max numbers to sum (the first one plus additions)   (default: all)

use aoc_2021_rust::advent::day18::animate;

fn main() {
    let mut args = std::env::args().skip(1);

    let which = args.next().unwrap_or_else(|| "sample-1".to_string());
    let max_lines: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);

    let filename = match which.as_str() {
        "sample-1" | "1" => "input/day_18-sample-1.txt",
        "sample-2" | "2" => "input/day_18-sample-2.txt",
        "input" => "input/day_18-input.txt",
        other => other, // allow passing a raw path
    };

    animate::run(filename, max_lines);
}
