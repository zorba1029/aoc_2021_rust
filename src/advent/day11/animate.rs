// advent/day11/animate.rs
//
// Phase visualization for the Dumbo Octopus (Day 11) cellular automaton.
//
// Instead of logging the grid one step at a time, this redraws the terminal
// in place using ANSI truecolor escapes so you can *watch* the excitation
// spread. Each octopus' energy level is mapped to a cool -> hot color
// gradient, and the moment a cell flashes it bursts bright white.
//
// The interesting part is rendered too: the inner cascade. When one flash
// pushes its neighbors over the edge, they flash on the next inner pass --
// we draw a frame per pass, so the flash propagates outward as a visible
// wavefront, just like a reaction-diffusion wave or coupled oscillators
// pulling each other into sync.
//
// Run it with the companion binary:
//     cargo run --release --bin day11_anim
//     cargo run --release --bin day11_anim -- sample-a 8       (quick 5x5 demo)
//     cargo run --release --bin day11_anim -- input 360        (the real puzzle)

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::thread::sleep;
use std::time::Duration;

type Grid = Vec<Vec<u32>>;

// 8-neighborhood (Moore neighborhood) offsets.
const DIRS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

// Frame timings (milliseconds). Tweak to taste.
const CHARGE_MS: u64 = 90; // after the +1 "charge" pass
const FLASH_MS: u64 = 120; //--70; // per cascade ring -- this is the wavefront
const SETTLE_MS: u64 = 140; // after a step settles
const SYNC_MS: u64 = 90; // blink cadence on the synchronized flash

/// Entry point. `filename` is a full path; `total_steps` caps how long we run.
pub fn run(filename: &str, total_steps: u16) {
    let mut grid = read_grid(filename);
    let rows = grid.len();
    let cols = grid[0].len();
    let cells = rows * cols;

    // Enter alternate screen, hide cursor, clear.
    print!("\x1b[?1049h\x1b[?25l\x1b[2J");
    io::stdout().flush().ok();

    let mut total_flash: u64 = 0;
    let no_flash = vec![vec![false; cols]; rows];

    // Frame 0: the initial grid.
    render(&grid, &no_flash, 0, total_flash, "initial");
    sleep(Duration::from_millis(SETTLE_MS * 3));

    let mut first_sync: Option<u16> = None;

    for step in 1..=total_steps {
        let flashed_this_step = animate_step(&mut grid, step, &mut total_flash);

        // Did every octopus flash on the same step? (Part Two's question.)
        if flashed_this_step == cells {
            first_sync.get_or_insert(step);
            celebrate_sync(&grid, step, total_flash);
            break;
        }

        // Settle frame: the post-reset state for this step.
        render(&grid, &no_flash, step, total_flash, "settled");
        sleep(Duration::from_millis(SETTLE_MS));
    }

    // Leave alternate screen, restore cursor.
    print!("\x1b[?25h\x1b[?1049l");
    io::stdout().flush().ok();

    match first_sync {
        Some(s) => println!(
            "✨ All {cells} octopuses flashed in sync at step {s}. \
             Total flashes: {total_flash}."
        ),
        None => println!(
            "Ran {total_steps} steps without a full sync. \
             Total flashes: {total_flash}."
        ),
    }
}

/// Advance one step, rendering a frame after the charge pass and after every
/// cascade ring. Returns how many octopuses flashed this step.
fn animate_step(grid: &mut Grid, step: u16, total_flash: &mut u64) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    // 1) Charge: every energy level rises by one.
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell += 1;
        }
    }
    let mut flashed = vec![vec![false; cols]; rows];
    render(grid, &flashed, step, *total_flash, "charging");
    sleep(Duration::from_millis(CHARGE_MS));

    // 2) Cascade: anyone above 9 flashes and bumps its neighbors. New cells
    //    pushed over the edge flash on the next pass -> the wave spreads.
    loop {
        let mut ring: Vec<(usize, usize)> = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] > 9 && !flashed[i][j] {
                    ring.push((i, j));
                }
            }
        }
        if ring.is_empty() {
            break;
        }
        for &(i, j) in &ring {
            flashed[i][j] = true;
            for (di, dj) in DIRS {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 {
                    grid[ni as usize][nj as usize] += 1;
                }
            }
        }
        *total_flash += ring.len() as u64;
        render(grid, &flashed, step, *total_flash, "flashing");
        sleep(Duration::from_millis(FLASH_MS));
    }

    // 3) Reset: every octopus that flashed drops back to 0.
    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if flashed[i][j] {
                grid[i][j] = 0;
                count += 1;
            }
        }
    }
    count
}

/// The synchronized flash -- blink the whole grid a few times to mark it.
fn celebrate_sync(grid: &Grid, step: u16, total_flash: u64) {
    let rows = grid.len();
    let cols = grid[0].len();
    let all_on = vec![vec![true; cols]; rows];
    let all_off = vec![vec![false; cols]; rows];
    for k in 0..6 {
        let mask = if k % 2 == 0 { &all_on } else { &all_off };
        render(grid, mask, step, total_flash, "★ SYNC ★");
        sleep(Duration::from_millis(SYNC_MS));
    }
    sleep(Duration::from_millis(SETTLE_MS * 4));
}

/// Draw one frame at the top of the screen. `flashed[i][j] == true` paints a
/// bright burst; otherwise the cell is colored by its energy level.
fn render(grid: &Grid, flashed: &[Vec<bool>], step: u16, total_flash: u64, phase: &str) {
    let mut out = String::with_capacity(grid.len() * grid[0].len() * 24);
    out.push_str("\x1b[H"); // cursor home -- redraw in place
    out.push_str(&format!(
        "  Dumbo Octopus — step {step:>4}   flashes {total_flash:>7}   [{phase}]\x1b[K\n\n"
    ));

    for (i, row) in grid.iter().enumerate() {
        out.push_str("    ");
        for (j, &e) in row.iter().enumerate() {
            let (r, g, b) = if flashed[i][j] {
                (255, 255, 215) // flash burst
            } else {
                heat(e)
            };
            out.push_str(&format!("\x1b[38;2;{r};{g};{b}m██"));
        }
        out.push_str("\x1b[0m\x1b[K\n");
        let _ = i;
    }
    out.push_str("\x1b[K\n");
    print!("{out}");
    io::stdout().flush().ok();
}

/// Map an energy level (0..=9, may briefly exceed 9 mid-cascade) to an RGB
/// heat color: deep blue (cold) -> teal -> amber -> red-orange (hot).
fn heat(e: u32) -> (u8, u8, u8) {
    // 5 gradient stops keyed at t = 0, .25, .5, .75, 1.
    const STOPS: [(f32, f32, f32); 5] = [
        (12.0, 14.0, 48.0),    // 0.00  deep blue (rested)
        (0.0, 90.0, 170.0),    // 0.25  blue
        (0.0, 168.0, 150.0),   // 0.50  teal
        (210.0, 170.0, 0.0),   // 0.75  amber
        (235.0, 70.0, 35.0),   // 1.00  red-orange (about to flash)
    ];
    let t = (e.min(9) as f32) / 9.0;
    let seg = (t * 4.0).floor().min(3.0) as usize;
    let local = (t * 4.0) - seg as f32;
    let (r0, g0, b0) = STOPS[seg];
    let (r1, g1, b1) = STOPS[seg + 1];
    let lerp = |a: f32, b: f32| (a + (b - a) * local).round() as u8;
    (lerp(r0, r1), lerp(g0, g1), lerp(b0, b1))
}

/// Read a grid of single-digit energy levels from a file.
fn read_grid(filename: &str) -> Grid {
    let file = File::open(filename)
        .unwrap_or_else(|e| panic!("couldn't open {filename}: {e}"));
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("non-digit in grid"))
                .collect()
        })
        .collect()
}
