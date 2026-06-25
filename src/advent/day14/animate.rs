// advent/day14/animate.rs
//
// Visualization for Extended Polymerization (Day 14).
//
// The polymer grows exponentially: after 40 steps the string would be ~20
// trillion characters, so we never build it. Like day_14_6th.rs, we track the
// COUNT of each adjacent pair instead. One step turns every pair (a,b) with a
// rule (a,b)->r into two new pairs (a,r) and (r,b), carrying the same count --
// here done by rebuilding a fresh map (no clone/subtract needed).
//
// What you watch:
//   - an input-loading phase that reads the template, then each insertion rule
//     one at a time with an `i/total` counter,
//   - then per-step bar charts: pair counts (the actual data structure) on top,
//     element counts below with MAX/min and the answer (MAX - min). Bars are
//     normalized to each step's own maximum, so the shape stays readable while
//     the printed numbers explode.
//
// Run it with the companion binary:
//     cargo run --release --bin day14_anim
//     cargo run --release --bin day14_anim -- input 40     (the real puzzle)
//     cargo run --release --bin day14_anim -- sample-a 10  (Part 1 point)

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::thread::sleep;
use std::time::Duration;

type PairCounts = HashMap<(char, char), u64>;
type Rules = HashMap<(char, char), char>;
type Colors = HashMap<char, (u8, u8, u8)>;

// Frame timings (milliseconds) and layout. Tweak to taste.
const LOAD_MS: u64 = 45; // per insertion rule while reading input
const INTRO_PAUSE_MS: u64 = 700; // after the input is fully read
const STEP_MS: u64 = 420; // per polymerization step
const PAIR_TOP: usize = 14; // how many pair bars to show (rest summarized)
const BAR_W: usize = 40; // max bar length in columns

// Stable, distinct colors handed out to the elements in alphabetical order.
const PALETTE: [(u8, u8, u8); 8] = [
    (90, 170, 255),  // blue
    (120, 230, 140), // green
    (255, 200, 90),  // amber
    (255, 120, 140), // red-pink
    (190, 140, 255), // purple
    (90, 210, 210),  // teal
    (240, 150, 90),  // orange
    (200, 200, 120), // olive
];

/// Entry point. `filename` is a full path; `max_steps` caps the polymerization.
pub fn run(filename: &str, max_steps: u32) {
    let (template, rules_ordered) = read_input(filename);
    if template.is_empty() {
        println!("No polymer template in {filename}");
        return;
    }
    let rules: Rules = rules_ordered.iter().copied().collect();
    let colors = build_colors(&template, &rules_ordered);

    // Enter alternate screen, hide cursor.
    print!("\x1b[?1049h\x1b[?25l\x1b[2J");
    io::stdout().flush().ok();

    // --- Input-loading phase: read the template, then each rule one at a time.
    let total = rules_ordered.len();
    render_loading(&template, 0, total, None, &colors);
    sleep(Duration::from_millis(INTRO_PAUSE_MS));
    for (i, &(pair, r)) in rules_ordered.iter().enumerate() {
        render_loading(&template, i + 1, total, Some((pair, r)), &colors);
        sleep(Duration::from_millis(LOAD_MS));
    }
    sleep(Duration::from_millis(INTRO_PAUSE_MS));

    // --- Step phase: animate the pair/element counts growing.
    let first = template[0];
    let mut counter: PairCounts = HashMap::new();
    for w in template.windows(2) {
        *counter.entry((w[0], w[1])).or_default() += 1;
    }

    render_step(&template, 0, max_steps, &counter, &colors, first);
    sleep(Duration::from_millis(STEP_MS));
    for s in 1..=max_steps {
        counter = step(&counter, &rules);
        render_step(&template, s, max_steps, &counter, &colors, first);
        sleep(Duration::from_millis(STEP_MS));
    }

    // Leave alternate screen, restore cursor.
    print!("\x1b[?25h\x1b[?1049l");
    io::stdout().flush().ok();

    let elements = element_counts(&counter, first);
    let max = elements.values().max().copied().unwrap_or(0);
    let min = elements.values().min().copied().unwrap_or(0);
    println!(
        "Final ({max_steps} steps): MAX {} - min {} = {}",
        commas(max),
        commas(min),
        commas(max - min)
    );
}

// --------------------------- Algorithm -------------------------------

/// One polymerization step: rebuild the pair counts from scratch. Each pair
/// (a,b)->r becomes (a,r) and (r,b); a pair with no rule is carried unchanged.
fn step(counter: &PairCounts, rules: &Rules) -> PairCounts {
    let mut next: PairCounts = HashMap::new();
    for (&(a, b), &c) in counter {
        if let Some(&r) = rules.get(&(a, b)) {
            *next.entry((a, r)).or_default() += c;
            *next.entry((r, b)).or_default() += c;
        } else {
            *next.entry((a, b)).or_default() += c;
        }
    }
    next
}

/// Count each element once: the second char of every pair, plus the template's
/// first char (which is never the second char of any pair, and never changes).
fn element_counts(counter: &PairCounts, first: char) -> HashMap<char, u64> {
    let mut counts: HashMap<char, u64> = HashMap::new();
    *counts.entry(first).or_default() += 1;
    for (&(_, b), &c) in counter {
        *counts.entry(b).or_default() += c;
    }
    counts
}

// --------------------------- Rendering -------------------------------

/// Render the input-loading frame: template, then the current rule + counter.
fn render_loading(template: &[char], idx: usize, total: usize, rule: Option<((char, char), char)>, colors: &Colors) {
    let mut out = String::with_capacity(256);
    out.push_str("\x1b[H\x1b[J");
    out.push_str("  Day 14 — reading input\n\n");
    out.push_str("  template :  ");
    out.push_str(&colored_chars(template, colors));
    out.push_str("\n\n");

    match rule {
        Some(((a, b), r)) => {
            let pair = [a, b];
            out.push_str(&format!("  rule {idx:>3}/{total} :  "));
            out.push_str(&colored_chars(&pair, colors));
            out.push_str("\x1b[38;2;150;150;170m -> \x1b[0m");
            out.push_str(&colored_chars(&[r], colors));
            out.push('\n');
        }
        None => out.push_str(&format!("  rule {:>3}/{total} :  ...\n", 0)),
    }

    print!("{out}");
    io::stdout().flush().ok();
}

/// Render one polymerization step: pair bars, element bars, and the answer.
fn render_step(template: &[char], step: u32, max_steps: u32, counter: &PairCounts, colors: &Colors, first: char) {
    let elements = element_counts(counter, first);
    let length: u64 = elements.values().sum();

    let mut out = String::with_capacity(2048);
    out.push_str("\x1b[H\x1b[J");
    out.push_str(&format!(
        "  Day 14 — step {step}/{max_steps}   length={}\n",
        commas(length)
    ));
    out.push_str("  template :  ");
    out.push_str(&colored_chars(template, colors));
    out.push_str("\n\n");

    // -- pair counts (top N) --
    out.push_str("  ── pairs ──\n");
    let mut pairs: Vec<(&(char, char), &u64)> = counter.iter().filter(|(_, &c)| c > 0).collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let pair_max = pairs.first().map(|(_, &c)| c).unwrap_or(1);
    for (&(a, b), &c) in pairs.iter().take(PAIR_TOP) {
        let label = format!("{a}{b}");
        out.push_str(&bar_line(&label, c, pair_max, colors[&a], ""));
        out.push('\n');
    }
    if pairs.len() > PAIR_TOP {
        out.push_str(&format!(
            "  \x1b[38;2;150;150;170m(+{} more pairs)\x1b[0m\n",
            pairs.len() - PAIR_TOP
        ));
    }
    out.push('\n');

    // -- element counts --
    out.push_str("  ── elements ──\n");
    let mut els: Vec<(&char, &u64)> = elements.iter().collect();
    els.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let el_max = els.first().map(|(_, &c)| c).unwrap_or(1);
    let max_c = el_max;
    let min_c = els.last().map(|(_, &c)| c).unwrap_or(0);
    for (&ch, &c) in &els {
        let mark = if c == max_c {
            "\x1b[1m◀ MAX\x1b[0m"
        } else if c == min_c {
            "\x1b[38;2;150;150;170m◀ min\x1b[0m"
        } else {
            ""
        };
        out.push_str(&bar_line(&ch.to_string(), c, el_max, colors[&ch], mark));
        out.push('\n');
    }
    out.push_str(&format!(
        "\n  answer (MAX - min): \x1b[1m{}\x1b[0m\n",
        commas(max_c - min_c)
    ));

    print!("{out}");
    io::stdout().flush().ok();
}

/// One labelled bar: `LBL ████…   1,234,567   <mark>`, normalized to `max`.
fn bar_line(label: &str, count: u64, max: u64, rgb: (u8, u8, u8), mark: &str) -> String {
    let filled = if max == 0 {
        0
    } else {
        let f = (count as f64 / max as f64 * BAR_W as f64).round() as usize;
        if count > 0 {
            f.clamp(1, BAR_W)
        } else {
            0
        }
    };
    let (r, g, b) = rgb;
    let bar = "█".repeat(filled);
    let pad = " ".repeat(BAR_W - filled);
    format!(
        "  {label:<4}\x1b[38;2;{r};{g};{b}m{bar}\x1b[0m{pad}  {:>18}  {mark}",
        commas(count)
    )
}

/// Render a run of characters, each painted by its element color.
fn colored_chars(chars: &[char], colors: &Colors) -> String {
    let mut s = String::new();
    for &c in chars {
        let (r, g, b) = colors.get(&c).copied().unwrap_or((220, 220, 220));
        s.push_str(&format!("\x1b[38;2;{r};{g};{b}m{c} \x1b[0m"));
    }
    s
}

/// Group a number with thousands separators: 1234567 -> "1,234,567".
fn commas(n: u64) -> String {
    let digits = n.to_string();
    let len = digits.len();
    let mut out = String::with_capacity(len + len / 3);
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// Assign each distinct element a stable color (alphabetical order -> palette).
fn build_colors(template: &[char], rules: &[((char, char), char)]) -> Colors {
    let mut elements: Vec<char> = Vec::new();
    let mut push = |c: char| {
        if !elements.contains(&c) {
            elements.push(c);
        }
    };
    for &c in template {
        push(c);
    }
    for &((a, b), r) in rules {
        push(a);
        push(b);
        push(r);
    }
    drop(push);
    elements.sort_unstable();
    elements
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, PALETTE[i % PALETTE.len()]))
        .collect()
}

// --------------------------- Input ----------------------------------

/// Read the polymer template (line 0) and the ordered insertion rules.
fn read_input(filename: &str) -> (Vec<char>, Vec<((char, char), char)>) {
    let file = File::open(filename).unwrap_or_else(|e| panic!("couldn't open {filename}: {e}"));
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let template: Vec<char> = lines.first().map(|l| l.trim().chars().collect()).unwrap_or_default();

    let mut rules = Vec::new();
    for line in lines.iter().skip(1) {
        if let Some((left, right)) = line.split_once("->") {
            let mut lc = left.trim().chars();
            if let (Some(a), Some(b)) = (lc.next(), lc.next()) {
                if let Some(r) = right.trim().chars().next() {
                    rules.push(((a, b), r));
                }
            }
        }
    }
    (template, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_steps(filename: &str, steps: u32) -> u64 {
        let (template, rules_ordered) = read_input(filename);
        let rules: Rules = rules_ordered.into_iter().collect();
        let first = template[0];
        let mut counter: PairCounts = HashMap::new();
        for w in template.windows(2) {
            *counter.entry((w[0], w[1])).or_default() += 1;
        }
        for _ in 0..steps {
            counter = step(&counter, &rules);
        }
        let elements = element_counts(&counter, first);
        elements.values().max().unwrap() - elements.values().min().unwrap()
    }

    #[test]
    fn sample_part1_step10() {
        assert_eq!(run_steps("input/day_14-sample-a.txt", 10), 1588);
    }

    #[test]
    fn sample_part2_step40() {
        assert_eq!(run_steps("input/day_14-sample-a.txt", 40), 2188189693529);
    }

    #[test]
    fn commas_groups_thousands() {
        assert_eq!(commas(0), "0");
        assert_eq!(commas(1588), "1,588");
        assert_eq!(commas(2188189693529), "2,188,189,693,529");
    }
}
