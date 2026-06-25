// Day 14: Extended Polymerization — count visualization (animated, in-terminal).
//
//   cargo run --release --bin day14_anim                      # prompts for view + mode
//   cargo run --release --bin day14_anim -- input 40 a        # multi-column bars (prompts mode)
//   cargo run --release --bin day14_anim -- input 40 b auto   # heatmap matrix, auto-paced
//   cargo run --release --bin day14_anim -- input 40 a step   # bars, advance on keypress
//
// arg1: which input  -> sample-a | input            (default: sample-a)
// arg2: max steps                                   (default: 40)
// arg3: pairs view   -> a | columns | b | matrix    (omit to be prompted)
// arg4: step mode    -> auto | step | interactive   (omit to be prompted)

use aoc_2021_rust::advent::day14::animate::{self, PairView, StepMode};
use std::io::Write;

fn main() {
    let mut args = std::env::args().skip(1);

    let which = args.next().unwrap_or_else(|| "sample-a".to_string());
    let steps: u32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(40);
    let view = match args.next() {
        Some(s) => parse_view(&s),
        None => prompt_view(),
    };
    let mode = match args.next() {
        Some(s) => parse_mode(&s),
        None => prompt_mode(),
    };

    let filename = match which.as_str() {
        "sample-a" | "a" => "input/day_14-sample-a.txt",
        "input" => "input/day_14-input.txt",
        other => other, // allow passing a raw path
    };

    animate::run(filename, steps, view, mode);
}

fn parse_view(s: &str) -> PairView {
    match s.trim().to_lowercase().as_str() {
        "b" | "matrix" => PairView::Matrix,
        _ => PairView::Columns,
    }
}

fn parse_mode(s: &str) -> StepMode {
    match s.trim().to_lowercase().as_str() {
        "step" | "interactive" | "i" => StepMode::Interactive,
        _ => StepMode::Auto,
    }
}

/// Ask which pairs view to use (before entering the alternate screen).
fn prompt_view() -> PairView {
    println!("Day 14 animation — choose the PAIRS view:");
    println!("  [a] multi-column bars  (all pairs, exact abbreviated numbers)");
    println!("  [b] heatmap matrix     (all pairs as a colored grid)");
    parse_view(&read_line("Select [a/b] (default a): "))
}

/// Ask how to pace the steps (before entering the alternate screen).
fn prompt_mode() -> StepMode {
    println!("Choose the STEP mode:");
    println!("  [auto] advance automatically");
    println!("  [step] pause each step; press a key to advance");
    parse_mode(&read_line("Select [auto/step] (default auto): "))
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    std::io::stdout().flush().ok();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line
}
