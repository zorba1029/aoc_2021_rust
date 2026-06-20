// advent/day18/animate.rs
//
// Reduction visualization for Snailfish numbers (Day 18).
//
// A snailfish number is a nested pair like [[1,2],[3,4]]. Adding two of them
// concatenates the pairs and then *reduces* the result by repeatedly applying:
//
//   - explode: the leftmost pair nested inside four pairs (depth 5) bursts.
//     Its left value is added to the nearest regular number on the left, its
//     right value to the nearest one on the right, and the pair itself becomes 0.
//   - split: the leftmost regular number >= 10 becomes a pair [v/2, (v+1)/2].
//
// Explode always wins over split. The interesting part to watch is this
// cascade: one explode pushes a value into a neighbor, which may push it over
// 10, which splits into a deep pair, which then explodes again -- the same
// kind of chain reaction as Day 11's flashes, but on a tree instead of a grid.
//
// Instead of the Rc<RefCell> tree used by the solver, this works on a flat
// token list (Open / Close / Comma / Num). That keeps the brackets around so
// we can paint each character by its nesting depth, and it makes each explode
// or split a simple, hookable edit we can render one frame at a time.
//
// Run it with the companion binary:
//     cargo run --release --bin day18_anim
//     cargo run --release --bin day18_anim -- sample-2       (other sample)
//     cargo run --release --bin day18_anim -- input 5        (real input, first 5 lines)

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::thread::sleep;
use std::time::Duration;

/// One lexical piece of a snailfish number.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Tok {
    Open,
    Close,
    Comma,
    Num(i64),
}

/// How a token is being highlighted in the current frame.
#[derive(Clone, Copy, PartialEq)]
enum Hi {
    Target,   // the pair about to explode / the number about to split
    Neighbor, // a regular number receiving an exploded value
    Result,   // the freshly produced 0 (explode) or pair (split)
}

// Frame timings (milliseconds). Tweak to taste.
const START_MS: u64 = 900; // initial line, before we start adding
const CONCAT_MS: u64 = 650; // after concatenating the next line
const OP_MS: u64 = 320; // each explode / split half-frame (before, then after)
const SETTLE_MS: u64 = 700; // after a line is fully reduced

// Soft-wrap the rendered number at this column so long accumulators stay readable.
const WIDTH: usize = 100;

/// Flat colored-string view: the raw token representation, painted by depth.
/// `filename` is a full path; `max_lines` caps how many input numbers are summed.
pub fn run(filename: &str, max_lines: usize) {
    animate_with(filename, max_lines, render);
}

/// Hierarchical (indented) tree view of the same reduction.
pub fn run_tree(filename: &str, max_lines: usize) {
    animate_with(filename, max_lines, render_tree);
}

/// Combined view: the flat string and the tree, animating together.
pub fn run_both(filename: &str, max_lines: usize) {
    animate_with(filename, max_lines, render_both);
}

/// Shared driver: run the reduction, calling `render` for every frame so the
/// flat and tree views differ only in how a frame is drawn.
fn animate_with(filename: &str, max_lines: usize, render: fn(&[Tok], &str, &str, &str, &[(usize, Hi)])) {
    let lines = read_lines(filename);
    if lines.is_empty() {
        println!("No snailfish numbers in {filename}");
        return;
    }
    let total = lines.len().min(max_lines.max(1));

    // Enter alternate screen, hide cursor.
    print!("\x1b[?1049h\x1b[?25l\x1b[2J");
    io::stdout().flush().ok();

    let mut acc = lines[0].1.clone();
    let info = format!("number 1/{total}:  {}", clip(&lines[0].0));
    render(&acc, &format!("add 1/{total}  ·  start"), "loaded", &info, &[]);
    sleep(Duration::from_millis(START_MS));

    let mut total_ops: u64 = 0;
    for li in 1..total {
        // Two-line context shown at the top all through this addition:
        // the running sum so far, then the raw input line being folded into it.
        let so_far = snail_to_string(&acc);
        let info = format!("so far :  {}\n  adding :  {}", clip(&so_far), clip(&lines[li].0));

        // Concatenate: result = [ acc , next ].
        let mut next = vec![Tok::Open];
        next.extend(acc.iter().copied());
        next.push(Tok::Comma);
        next.extend(lines[li].1.iter().copied());
        next.push(Tok::Close);
        acc = next;
        render(
            &acc,
            &format!("add {}/{total}  ·  concatenate", li + 1),
            "added",
            &info,
            &[],
        );
        sleep(Duration::from_millis(CONCAT_MS));

        // Reduce: explode first, else split, one operation per pair of frames.
        let mut ops: u64 = 0;
        loop {
            if let Some(op) = find_explode(&acc) {
                ops += 1;
                total_ops += 1;
                let header = format!("add {}/{total}  ·  op {ops}", li + 1);

                // Before: show the doomed pair and the neighbors it feeds.
                let mut hi = Vec::new();
                for k in op.open..op.open + 5 {
                    hi.push((k, Hi::Target));
                }
                if let Some(l) = op.left {
                    hi.push((l, Hi::Neighbor));
                }
                if let Some(r) = op.right {
                    hi.push((r, Hi::Neighbor));
                }
                render(&acc, &header, "explode", &info, &hi);
                sleep(Duration::from_millis(OP_MS));

                // After: the pair collapsed to 0.
                apply_explode(&mut acc, &op);
                render(&acc, &header, "exploded", &info, &[(op.open, Hi::Result)]);
                sleep(Duration::from_millis(OP_MS));
                continue;
            }
            if let Some(op) = find_split(&acc) {
                ops += 1;
                total_ops += 1;
                let header = format!("add {}/{total}  ·  op {ops}", li + 1);

                // Before: highlight the number about to split.
                render(&acc, &header, "split", &info, &[(op.idx, Hi::Target)]);
                sleep(Duration::from_millis(OP_MS));

                // After: the new pair occupies idx..idx+5.
                apply_split(&mut acc, &op);
                let hi: Vec<_> = (op.idx..op.idx + 5).map(|k| (k, Hi::Result)).collect();
                render(&acc, &header, "split done", &info, &hi);
                sleep(Duration::from_millis(OP_MS));
                continue;
            }
            break;
        }

        render(
            &acc,
            &format!("add {}/{total}  ·  reduced ({ops} ops)", li + 1),
            "settled",
            &info,
            &[],
        );
        sleep(Duration::from_millis(SETTLE_MS));
    }

    // Leave alternate screen, restore cursor.
    print!("\x1b[?25h\x1b[?1049l");
    io::stdout().flush().ok();

    println!("Final sum: {}", snail_to_string(&acc));
    println!(
        "Magnitude: {}   (total reduce operations: {total_ops})",
        magnitude(&acc)
    );
}

// --------------------------- Explode ---------------------------------

struct ExplodeOp {
    open: usize,          // index of the pair's '['
    a: i64,               // left value
    b: i64,               // right value
    left: Option<usize>,  // nearest regular number to the left
    right: Option<usize>, // nearest regular number to the right
}

/// Find the leftmost pair nested inside four pairs (depth 5). By the snailfish
/// invariant such a pair is always two regular numbers.
fn find_explode(toks: &[Tok]) -> Option<ExplodeOp> {
    let mut depth = 0usize;
    for i in 0..toks.len() {
        match toks[i] {
            Tok::Open => {
                depth += 1;
                if depth == 5
                    && matches!(toks.get(i + 1), Some(Tok::Num(_)))
                    && matches!(toks.get(i + 2), Some(Tok::Comma))
                    && matches!(toks.get(i + 3), Some(Tok::Num(_)))
                    && matches!(toks.get(i + 4), Some(Tok::Close))
                {
                    let a = if let Tok::Num(v) = toks[i + 1] { v } else { 0 };
                    let b = if let Tok::Num(v) = toks[i + 3] { v } else { 0 };
                    let left = (0..i).rev().find(|&k| matches!(toks[k], Tok::Num(_)));
                    let right = (i + 5..toks.len()).find(|&k| matches!(toks[k], Tok::Num(_)));
                    return Some(ExplodeOp {
                        open: i,
                        a,
                        b,
                        left,
                        right,
                    });
                }
            }
            Tok::Close => depth -= 1,
            _ => {}
        }
    }
    None
}

/// Apply the explode: feed the neighbors, then replace the pair with a single 0.
fn apply_explode(toks: &mut Vec<Tok>, op: &ExplodeOp) {
    if let Some(l) = op.left {
        if let Tok::Num(v) = &mut toks[l] {
            *v += op.a;
        }
    }
    if let Some(r) = op.right {
        if let Tok::Num(v) = &mut toks[r] {
            *v += op.b;
        }
    }
    toks.splice(op.open..op.open + 5, [Tok::Num(0)]);
}

// --------------------------- Split -----------------------------------

struct SplitOp {
    idx: usize,
    v: i64,
}

/// Find the leftmost regular number >= 10.
fn find_split(toks: &[Tok]) -> Option<SplitOp> {
    toks.iter().enumerate().find_map(|(idx, t)| match t {
        Tok::Num(v) if *v >= 10 => Some(SplitOp { idx, v: *v }),
        _ => None,
    })
}

/// Apply the split: replace the number with the pair [v/2, (v+1)/2].
fn apply_split(toks: &mut Vec<Tok>, op: &SplitOp) {
    let l = op.v / 2;
    let r = (op.v + 1) / 2;
    toks.splice(
        op.idx..op.idx + 1,
        [Tok::Open, Tok::Num(l), Tok::Comma, Tok::Num(r), Tok::Close],
    );
}

// --------------------------- Magnitude -------------------------------

/// Magnitude via a stack: a closed pair's two top values fold to 3*left+2*right.
fn magnitude(toks: &[Tok]) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    for t in toks {
        match t {
            Tok::Num(v) => stack.push(*v),
            Tok::Close => {
                let r = stack.pop().unwrap_or(0);
                let l = stack.pop().unwrap_or(0);
                stack.push(3 * l + 2 * r);
            }
            _ => {}
        }
    }
    stack.pop().unwrap_or(0)
}

// --------------------------- Rendering -------------------------------

/// Highlight color for a token/node, or its depth heat color when not highlighted.
fn color(kind: Option<Hi>, depth: usize) -> (u8, u8, u8) {
    match kind {
        Some(Hi::Target) => (255, 255, 180),   // bright burst
        Some(Hi::Neighbor) => (120, 255, 140), // green: received a value
        Some(Hi::Result) => (255, 140, 255),   // magenta: freshly produced
        None => heat(depth),
    }
}

/// Begin a frame: clear the screen and write the title, the live reduce text
/// (only while reducing), and the input-line context.
fn start_frame(title: &str, header: &str, phase: &str, info: &str, toks: &[Tok]) -> String {
    let mut out = String::with_capacity(256);
    out.push_str("\x1b[H\x1b[J"); // home + clear to end -- redraw in place
    out.push_str(&format!("  {title} — {header}   [{phase}]\n"));
    // Top line: the number as plain text, updated on every explode/split so the
    // reduction can be read step by step above the colored views.
    if matches!(phase, "explode" | "exploded" | "split" | "split done") {
        out.push_str(&format!(
            "  \x1b[38;2;225;225;235mreduce :  {}\x1b[0m\n",
            clip(&snail_to_string(toks))
        ));
    }
    out.push_str(&format!("  \x1b[38;2;150;150;170m{info}\x1b[0m\n\n"));
    out
}

/// Flush a finished frame to the terminal.
fn end_frame(out: &str) {
    print!("{out}");
    io::stdout().flush().ok();
}

/// Append the flat colored-string view (depth-painted, soft-wrapped) to `out`.
fn flat_body(toks: &[Tok], highlights: &[(usize, Hi)], out: &mut String) {
    out.push_str("    ");
    let mut depth = 0usize;
    let mut col = 4usize;
    for (i, t) in toks.iter().enumerate() {
        // Text of the token, plus the depth used to color it.
        let (txt, d) = match t {
            Tok::Open => {
                let d = depth;
                depth += 1;
                ("[".to_string(), d)
            }
            Tok::Close => {
                depth = depth.saturating_sub(1);
                ("]".to_string(), depth)
            }
            Tok::Comma => (",".to_string(), depth.saturating_sub(1)),
            Tok::Num(v) => (v.to_string(), depth.saturating_sub(1)),
        };

        if col + txt.len() > WIDTH {
            out.push_str("\n    ");
            col = 4;
        }

        let kind = highlights.iter().find(|(k, _)| *k == i).map(|(_, k)| *k);
        let (r, g, b) = color(kind, d);
        let bold = if kind.is_some() { "\x1b[1m" } else { "" };
        out.push_str(&format!("{bold}\x1b[38;2;{r};{g};{b}m{txt}\x1b[0m"));
        col += txt.len();
    }
    out.push('\n');
}

/// Draw one frame: the flat snailfish number, painted by nesting depth.
fn render(toks: &[Tok], header: &str, phase: &str, info: &str, highlights: &[(usize, Hi)]) {
    let mut out = start_frame("Snailfish", header, phase, info, toks);
    flat_body(toks, highlights, &mut out);
    end_frame(&out);
}

/// Map a nesting depth (0..=5) to a cool -> hot color: blue -> teal -> amber.
fn heat(depth: usize) -> (u8, u8, u8) {
    const STOPS: [(f32, f32, f32); 5] = [
        (70.0, 110.0, 230.0), // 0.00  blue (shallow)
        (0.0, 150.0, 200.0),  // 0.25  cyan-blue
        (0.0, 175.0, 150.0),  // 0.50  teal
        (210.0, 170.0, 0.0),  // 0.75  amber
        (235.0, 90.0, 35.0),  // 1.00  red-orange (deep, near explode)
    ];
    let t = (depth.min(5) as f32) / 5.0;
    let seg = (t * 4.0).floor().min(3.0) as usize;
    let local = (t * 4.0) - seg as f32;
    let (r0, g0, b0) = STOPS[seg];
    let (r1, g1, b1) = STOPS[seg + 1];
    let lerp = |a: f32, b: f32| (a + (b - a) * local).round() as u8;
    (lerp(r0, r1), lerp(g0, g1), lerp(b0, b1))
}

// --------------------------- Tree rendering --------------------------

/// A display tree built from the token list. `tok` records the source token
/// index (a leaf's `Num`, or a pair's `[`) so token-index highlights map back.
struct Node {
    value: Option<i64>, // Some => leaf
    children: Option<(Box<Node>, Box<Node>)>,
    tok: usize,
}

/// Parse `toks[*i..]` into one node, advancing `*i` past it.
fn build(toks: &[Tok], i: &mut usize) -> Node {
    let start = *i;
    match toks[*i] {
        Tok::Num(v) => {
            *i += 1;
            Node {
                value: Some(v),
                children: None,
                tok: start,
            }
        }
        Tok::Open => {
            *i += 1; // consume '['
            let left = build(toks, i);
            *i += 1; // consume ','
            let right = build(toks, i);
            *i += 1; // consume ']'
            Node {
                value: None,
                children: Some((Box::new(left), Box::new(right))),
                tok: start,
            }
        }
        _ => unreachable!("snailfish node starts with '[' or a number"),
    }
}

/// Draw one node and its subtree in indented (`├─`/`└─`) form.
fn draw(node: &Node, prefix: &str, is_last: bool, is_root: bool, depth: usize, hi: &[(usize, Hi)], out: &mut String) {
    let branch = if is_root {
        String::new()
    } else {
        format!("{prefix}{}", if is_last { "└─ " } else { "├─ " })
    };

    let label = match node.value {
        Some(v) => v.to_string(),
        None => "●".to_string(),
    };
    let kind = hi.iter().find(|(k, _)| *k == node.tok).map(|(_, k)| *k);
    let (r, g, b) = color(kind, depth);
    let bold = if kind.is_some() { "\x1b[1m" } else { "" };

    // Dim gray connectors, then the colored node label.
    out.push_str(&format!(
        "    \x1b[38;2;90;90;110m{branch}\x1b[0m{bold}\x1b[38;2;{r};{g};{b}m{label}\x1b[0m\n"
    ));

    if let Some((left, right)) = &node.children {
        let child_prefix = if is_root {
            String::new()
        } else {
            format!("{prefix}{}", if is_last { "   " } else { "│  " })
        };
        draw(left, &child_prefix, false, false, depth + 1, hi, out);
        draw(right, &child_prefix, true, false, depth + 1, hi, out);
    }
}

/// Append the hierarchical (indented) tree view to `out`.
fn tree_body(toks: &[Tok], highlights: &[(usize, Hi)], out: &mut String) {
    let root = build(toks, &mut 0);
    draw(&root, "", true, true, 0, highlights, out);
    out.push('\n');
}

/// Draw one frame as a hierarchical tree (same signature as the flat `render`).
fn render_tree(toks: &[Tok], header: &str, phase: &str, info: &str, highlights: &[(usize, Hi)]) {
    let mut out = start_frame("Snailfish tree", header, phase, info, toks);
    tree_body(toks, highlights, &mut out);
    end_frame(&out);
}

/// Draw one frame with BOTH views: the flat string, then the tree below it.
fn render_both(toks: &[Tok], header: &str, phase: &str, info: &str, highlights: &[(usize, Hi)]) {
    let mut out = start_frame("Snailfish", header, phase, info, toks);
    flat_body(toks, highlights, &mut out);
    out.push_str("\n  \x1b[38;2;90;90;110m── tree ──\x1b[0m\n\n");
    tree_body(toks, highlights, &mut out);
    end_frame(&out);
}

// --------------------------- Parsing / IO ----------------------------

/// Plain (uncolored) string form of a token list, for the final printout.
fn snail_to_string(toks: &[Tok]) -> String {
    let mut s = String::with_capacity(toks.len() * 2);
    for t in toks {
        match t {
            Tok::Open => s.push('['),
            Tok::Close => s.push(']'),
            Tok::Comma => s.push(','),
            Tok::Num(v) => s.push_str(&v.to_string()),
        }
    }
    s
}

fn tokenize(line: &str) -> Vec<Tok> {
    let mut toks = Vec::new();
    let mut chars = line.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '[' => {
                toks.push(Tok::Open);
                chars.next();
            }
            ']' => {
                toks.push(Tok::Close);
                chars.next();
            }
            ',' => {
                toks.push(Tok::Comma);
                chars.next();
            }
            '0'..='9' => {
                let mut n = 0i64;
                while let Some(&d) = chars.peek() {
                    if let Some(digit) = d.to_digit(10) {
                        n = n * 10 + digit as i64;
                        chars.next();
                    } else {
                        break;
                    }
                }
                toks.push(Tok::Num(n));
            }
            _ => {
                chars.next();
            }
        }
    }
    toks
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Non-animated reduce, mirroring run()'s loop, for verification.
    fn reduce(toks: &mut Vec<Tok>) {
        loop {
            if let Some(op) = find_explode(toks) {
                apply_explode(toks, &op);
                continue;
            }
            if let Some(op) = find_split(toks) {
                apply_split(toks, &op);
                continue;
            }
            break;
        }
    }

    fn add(a: &str, b: &str) -> Vec<Tok> {
        let mut toks = vec![Tok::Open];
        toks.extend(tokenize(a));
        toks.push(Tok::Comma);
        toks.extend(tokenize(b));
        toks.push(Tok::Close);
        reduce(&mut toks);
        toks
    }

    #[test]
    fn tokenize_roundtrips() {
        let s = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        assert_eq!(snail_to_string(&tokenize(s)), s);
    }

    #[test]
    fn magnitude_examples() {
        let cases = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ];
        for (s, want) in cases {
            assert_eq!(magnitude(&tokenize(s)), want, "magnitude of {s}");
        }
    }

    #[test]
    fn add_reduces_to_known_result() {
        let sum = add("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]");
        assert_eq!(snail_to_string(&sum), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn full_homework_sum_magnitude() {
        let nums = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];
        let mut acc = tokenize(nums[0]);
        for n in &nums[1..] {
            acc = add(&snail_to_string(&acc), n);
        }
        assert_eq!(magnitude(&acc), 4140);
    }
}

/// Clip a long input line so the header line stays readable.
fn clip(s: &str) -> String {
    const MAX: usize = 120;
    if s.chars().count() <= MAX {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(MAX - 1).collect();
        out.push('…');
        out
    }
}

/// Read snailfish numbers (raw line + tokens), skipping blanks and '#' comments.
fn read_lines(filename: &str) -> Vec<(String, Vec<Tok>)> {
    let file = File::open(filename).unwrap_or_else(|e| panic!("couldn't open {filename}: {e}"));
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let toks = tokenize(&line);
            (line, toks)
        })
        .collect()
}
