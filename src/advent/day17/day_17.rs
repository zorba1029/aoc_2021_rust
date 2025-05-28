// advent/day_17.rs - part 1 and part 2
use log::{debug, info};
use std::fs::File;
use std::io::{BufRead, BufReader};

//-- input data
// target area: x=20..30, y=-10..-5
// target area: x=150..193, y=-136..-86
fn handle_input(filename: &str) -> ((i32, i32), (i32, i32)) {
    let file = File::open(filename).expect("Couldn't open input file.");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let single_line = &lines[0];

    info!("[*] Input Filename: {}", filename);
    info!("[*] Input line: {}", single_line);

    let items = single_line
        .split(":")
        .map(|item| item.trim())
        .collect::<Vec<_>>();
    let items = items[1]
        .split(",")
        .map(|item| item.trim())
        .collect::<Vec<_>>();
    let x_range = items[0].split("=").collect::<Vec<_>>();
    let y_range = items[1].split("=").collect::<Vec<_>>();

    let x_range_items = x_range[1]
        .split("..")
        .map(|item| item.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let y_range_items = y_range[1]
        .split("..")
        .map(|item| item.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let x_target = (x_range_items[0], x_range_items[1]);
    let y_target = (y_range_items[0], y_range_items[1]);
    info!("x_target = {:?}, y_target = {:?}", x_target, y_target);

    (x_target, y_target)
}

#[warn(dead_code)]
pub fn do_day_17() {
    info!("===============================================");
    info!("--- Day 17: Trick Shot, Part One ---, ");
    info!("===============================================");
    // let filename = "input/day_17-sample.txt";
    let filename = "input/day_17-input.txt";
    let (target_x, target_y) = handle_input(filename);
    info!("(target_x, target_y) = ({:?},{:?})", target_x, target_y);

    // -------------
    //           1         2         3
    // 0    5    0    5    0    5    0
    // +----+----+----+----+----+----+
    // .............#....#............
    // .......#..............#........
    // ...............................
    // S........................#.....
    // ...............................
    // ...............................
    // ...........................#...
    // ...............................
    // ....................TTTTTTTTTTT
    // ....................TTTTTTTTTTT
    // ....................TTTTTTTT#TT
    // ....................TTTTTTTTTTT
    // ....................TTTTTTTTTTT
    // ....................TTTTTTTTTTT

    // let x_target_range = i32::abs(target_x.1) - i32::abs(target_x.0) + 1;
    // let y_target_range = i32::abs(target_y.0) - i32::abs(target_y.1) + 1;
    // info!("(x_target_range, y_target_range) = ({x_target_range},{y_target_range})");
    info!("-------------------------");

    let (highest_y_pos, found_trajectories_count) = find_initial_velocity(&target_x, &target_y);
    //-- for sample data
    // part 1: highest y pos = 45,
    // part 2: found_trajectories count=112
    //-- for input data
    // part 1: highest y pos = 9180,
    // part 2: found_trajectories count = 3767
    info!("----------------------------------------");
    info!("part 1: highest y pos = {}", highest_y_pos);
    info!(
        "part 2: found trajectories count = {}",
        found_trajectories_count
    );
    info!("----------------------------------------");
}

fn find_initial_velocity(target_x: &(i32, i32), target_y: &(i32, i32)) -> (i32, i32) {
    let x_vel_range = 0..=target_x.1;
    let y_vel_range = target_y.0..=-target_y.0;
    let mut global_y_max = i32::MIN;
    let mut found_count = 0;

    for x_cur_init_vel in x_vel_range {
        for y_cur_init_vel in y_vel_range.clone() {
            // info!("[x_cur_init_vel, y_cur_init_vel] = ({x_cur_init_vel},{y_cur_init_vel})");
            if let Some(y_local_max) =
                compute_next_values(target_x, target_y, &x_cur_init_vel, &y_cur_init_vel)
            {
                found_count += 1;
                if y_local_max > global_y_max {
                    global_y_max = y_local_max;
                    debug!(
                        "  [*] current global_y_max = {}, found_count = {}",
                        global_y_max, found_count
                    );
                }
            }
        }
    }
    info!(
        "FINAL global_y_max = {}, found_trajectories_count = {}",
        global_y_max, found_count
    );

    (global_y_max, found_count)
}

fn compute_next_values(
    target_x: &(i32, i32),
    target_y: &(i32, i32),
    x_cur_init_vel: &i32,
    y_cur_init_vel: &i32,
) -> Option<i32> {
    let (mut x_pos, mut y_pos) = (0, 0);
    let (mut x_vel, mut y_vel) = (*x_cur_init_vel, *y_cur_init_vel);
    let mut loop_count = 0;
    let mut y_local_max = i32::MIN;
    let mut x_of_y_local_max = i32::MIN;

    loop {
        x_pos += x_vel;
        y_pos += y_vel;

        // keep the maximum y-value and its corresponding x-value
        if y_pos > y_local_max {
            y_local_max = y_pos;
            x_of_y_local_max = x_pos;
        }
        // debug!("     (x_pos, y_pos) = ({x_pos},{y_pos})");

        if (target_x.0 <= x_pos && x_pos <= target_x.1)
            && (target_y.0 <= y_pos && y_pos <= target_y.1)
        {
            debug!("FOUND [{}] -- [x_cur_init_vel, y_cur_init_vel] = ({},{}), **(x_vel, y_vel) = ({},{})", loop_count, x_cur_init_vel, y_cur_init_vel, x_vel, y_vel);
            debug!(
                "      (x_pos, y_pos) = ({x_pos},{y_pos}) IN target_x=({:?}), target_y=({:?})",
                target_x, target_y
            );
            debug!(
                "      y_local_max_pos = {}, ({},{})",
                y_local_max, x_of_y_local_max, y_local_max
            );
            return Some(y_local_max);
        }

        if x_pos < 0 || y_pos < target_y.0 {
            // debug!("NOT FOUND [{}] -- [x_cur_init_vel, y_cur_init_vel] = ({},{}), **(x_vel, y_vel) = ({},{})", loop_count, x_cur_init_vel, y_cur_init_vel, x_vel, y_vel);
            // debug!("      (x_pos, y_pos) = ({x_pos},{y_pos}) IN target_x=({:?}), target_y=({:?})", target_x, target_y);
            return None;
        }

        // update x_vel and y_vel
        x_vel = match Some(x_vel) {
            Some(x_vel) if x_vel > 0 => x_vel - 1,
            Some(x_vel) if x_vel < 0 => x_vel + 1,
            _ => 0,
        };

        y_vel -= 1;

        // debug!("     (x_vel, y_vel) = ({x_vel},{y_vel})");

        loop_count += 1;
    }
}
