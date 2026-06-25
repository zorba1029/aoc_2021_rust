// Day 18: Snailfish — reduction shown as BOTH the flat string and a tree,
// animating together (in-terminal).
//
//   cargo run --release --bin day18_tree_anim                 # prompts for tree style
//   cargo run --release --bin day18_tree_anim -- sample-1 2 a # indented tree
//   cargo run --release --bin day18_tree_anim -- sample-1 2 b # top-down (textbook) tree
//   cargo run --release --bin day18_tree_anim -- input 5 a    # real input, first 5 lines
//
// Same reduction engine as day18_anim; each frame draws the flat colored string
// and, below it, the same number as a tree (indented or top-down).
//
// arg1: which input  -> sample-1 | sample-2 | input        (default: sample-1)
// arg2: max numbers to sum (the first one plus additions)  (default: all)
// arg3: tree style   -> a | indented | b | vertical        (omit to be prompted)

use aoc_2021_rust::advent::day18::animate;
use std::io::Write;

fn main() {
    let mut args = std::env::args().skip(1);

    let which = args.next().unwrap_or_else(|| "sample-1".to_string());
    let max_lines: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
    let vertical = match args.next() {
        Some(s) => parse_style(&s),
        None => prompt_style(),
    };

    let filename = match which.as_str() {
        "sample-1" | "1" => "input/day_18-sample-1.txt",
        "sample-2" | "2" => "input/day_18-sample-2.txt",
        "input" => "input/day_18-input.txt",
        other => other, // allow passing a raw path
    };

    if vertical {
        animate::run_both_vtree(filename, max_lines);
    } else {
        animate::run_both(filename, max_lines);
    }
}

/// Returns true for the top-down (textbook) tree, false for the indented one.
fn parse_style(s: &str) -> bool {
    matches!(
        s.trim().to_lowercase().as_str(),
        "b" | "vertical" | "v" | "topdown" | "top-down"
    )
}

fn prompt_style() -> bool {
    println!("Day 18 tree animation — choose the TREE style:");
    println!("  [a] indented    (├─ / └─, compact, handles any size)");
    println!("  [b] top-down    (textbook tree with ┌─┴─┐ braces; needs more width)");
    print!("Select [a/b] (default a): ");
    std::io::stdout().flush().ok();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    parse_style(&line)
}
