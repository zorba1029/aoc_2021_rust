// Day 11: Dumbo Octopus — phase visualization (animated, in-terminal).
//
//   cargo run --release --bin day11_anim                  # sample-b, 200 steps
//   cargo run --release --bin day11_anim -- sample-a 8    # quick 5x5 demo
//   cargo run --release --bin day11_anim -- input 360     # the real puzzle
//
// arg1: which input  -> sample-a | sample-b | input   (default: sample-b)
// arg2: max steps     (default: 200)

use aoc_2021_rust::advent::day11::animate;

fn main() {
    let mut args = std::env::args().skip(1);

    let which = args.next().unwrap_or_else(|| "sample-b".to_string());
    let steps: u16 = args.next().and_then(|s| s.parse().ok()).unwrap_or(200);

    let filename = match which.as_str() {
        "sample-a" | "a" => "input/day_11-sample-a.txt",
        "sample-b" | "b" => "input/day_11-sample-b.txt",
        "input" => "input/day_11-input.txt",
        other => other, // allow passing a raw path
    };

    animate::run(filename, steps);
}
