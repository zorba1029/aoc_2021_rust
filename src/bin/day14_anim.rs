// Day 14: Extended Polymerization — count visualization (animated, in-terminal).
//
//   cargo run --release --bin day14_anim                    # prompts for view
//   cargo run --release --bin day14_anim -- input 40 a      # multi-column bars
//   cargo run --release --bin day14_anim -- input 40 b      # heatmap matrix
//   cargo run --release --bin day14_anim -- sample-a 10     # Part 1 point (prompts)
//
// arg1: which input  -> sample-a | input        (default: sample-a)
// arg2: max steps                               (default: 40)
// arg3: pairs view   -> a | columns | b | matrix  (omit to be prompted)

use aoc_2021_rust::advent::day14::animate::{self, PairView};
use std::io::Write;

fn main() {
    let mut args = std::env::args().skip(1);

    let which = args.next().unwrap_or_else(|| "sample-a".to_string());
    let steps: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(40);
    let view = match args.next() {
        Some(s) => parse_view(&s),
        None => prompt_view(),
    };

    let filename = match which.as_str() {
        "sample-a" | "a" => "input/day_14-sample-a.txt",
        "input" => "input/day_14-input.txt",
        other => other, // allow passing a raw path
    };

    animate::run(filename, steps, view);
}

fn parse_view(s: &str) -> PairView {
    match s.trim().to_lowercase().as_str() {
        "b" | "matrix" => PairView::Matrix,
        _ => PairView::Columns,
    }
}

/// Ask which pairs view to use (before entering the alternate screen).
fn prompt_view() -> PairView {
    println!("Day 14 animation — choose the PAIRS view:");
    println!("  [a] multi-column bars  (all pairs, exact abbreviated numbers)");
    println!("  [b] heatmap matrix     (all pairs as a colored grid)");
    print!("Select [a/b] (default a): ");
    std::io::stdout().flush().ok();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    parse_view(&line)
}
