// advent/day_5.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::usize;

////////////////////////////////
//--- Day 5: Hydrothermal Venture ---

//-------------------------------------------------------------------------

pub fn do_day_5() {
    day_5_part_one();
    day_5_part_two();
}

fn handle_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    println!("[*] Input Filename: {}", filename);
    println!("[*] input lines count = {}", lines_count);

    lines
}

fn day_5_part_one() {
    println!("//////////////////////////////////////////////");
    println!("--- Day 5: Hydrothermal, part one ---, 1/7/2022 ==> DONE");
    println!("-----------------------------------------------");
    let filename = "input/day_5-sample.txt";
    // let filename = "input/day_5-input.txt";
    let input_lines = handle_input(filename);

    let vent_lines = input_lines
        .iter()
        .map(|line| line.split("->").collect::<Vec<_>>())
        .map(|items| {
            let from = items[0].trim().split(',').collect::<Vec<_>>();
            let to = items[1].trim().split(',').collect::<Vec<_>>();
            let (x1, y1) = (
                from[0].parse::<i32>().unwrap(),
                from[1].parse::<i32>().unwrap(),
            );
            let (x2, y2) = (to[0].parse::<i32>().unwrap(), to[1].parse::<i32>().unwrap());
            [(x1, y1), (x2, y2)]
        })
        .collect::<Vec<[(i32, i32); 2]>>();
    println!("[*] draw_numbers: {:?}", vent_lines);

    // let from_lines = input_lines.iter()
    //     .map(|line| line.split("->").collect::<Vec<_>>())
    //     .map(|items| {
    //         let from = items[0].trim().split(',').collect::<Vec<_>>();
    //         let (x1, y1) = (from[0].parse::<i32>().unwrap(), from[1].parse::<i32>().unwrap());
    //         (x1, y1)
    //     })
    //     .collect::<Vec<_>>();
    // println!("[*] from_lines: {:?}", from_lines);

    // let to_lines = input_lines.iter()
    //     .map(|line| line.split("->").collect::<Vec<_>>())
    //     .map(|items| {
    //         let to = items[1].trim().split(',').collect::<Vec<_>>();
    //         let (x2, y2) = (to[0].parse::<i32>().unwrap(), to[1].parse::<i32>().unwrap());
    //         (x2, y2)
    //     })
    //     .collect::<Vec<_>>();
    // println!("[*] to_lines: {:?}", to_lines);

    let x_size = vent_lines
        .iter()
        .fold(i32::MIN, |acc, tuple| {
            let max = acc.max(tuple[0].0.max(tuple[1].0));
            max
        });
    println!("[*] x_size: {:?}", x_size);

    let y_size = vent_lines
        .iter()
        .fold(i32::MIN, |acc, tuple| {
            let max = acc.max(tuple[0].1.max(tuple[1].1));
            max
        });
    println!("[*] y_size: {:?}", y_size);

    let mut xy_diagram: Vec<i32> = vec![0; (x_size + 1) as usize * (y_size + 1) as usize];
    // top_left_corner is (0,0),
    // bottom_right_corner is (9,9)
    // .......1..
    // ..1....1..
    // ..1....1..
    // .......1..
    // .112111211
    // ..........
    // ..........
    // ..........
    // ..........
    // 222111....

    for [(x1, y1), (x2, y2)] in vent_lines.iter() {
        println!("({},{}) -> ({},{})", x1, y1, x2, y2);
        if *x1 == *x2 {
            let v_max = y1.max(y2);
            let v_min = y1.min(y2);
            println!("  y (v_min, v_max) -> x:{}, y:({}..{})", *x1, v_min, v_max);
            for y in *v_min..=*v_max {
                // xy_diagram[(*x1 as usize * x_size as usize) + y as usize] += 1;
                xy_diagram[*x1 as usize + (y as usize * x_size as usize)] += 1;
            }
        } else if *y1 == *y2 {
            let h_max = x1.max(x2);
            let h_min = x1.min(x2);
            println!("  x (min, max) -> x:({}..{}), y:{}", h_min, h_max, *y1);
            for x in *h_min..=*h_max {
                // xy_diagram[(x as usize * x_size as usize) + *y1 as usize] += 1;
                xy_diagram[x as usize + (*y1 as usize * x_size as usize)] += 1;
            }
        }
    }

    let mut total_overlap_points_count = 0;
    for x in 0..=x_size {
        for y in 0..=y_size {
            let value = xy_diagram[(x as usize * x_size as usize) + y as usize];
            if value > 0 {
                print!(
                    "{:1}",
                    xy_diagram[(x as usize * x_size as usize) + y as usize]
                );
                if value >= 2 {
                    total_overlap_points_count += 1;
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!( "[**] Total Overlap Points Count = {}", total_overlap_points_count );
    //-- FOR  "input/day_5-input.txt",
    // [**] Total Overlap Points Count = 6572
}

fn day_5_part_two() {
    println!("//////////////////////////////////////////////");
    println!("--- Day 5: Hydrothermal, Part Two ---, 1/8/2022 ==> DONE --");
    println!("-----------------------------------------------");
    let filename = "input/day_5-sample.txt";
    // let filename = "input/day_5-input.txt";
    let input_lines = handle_input(filename);

    let vent_lines = input_lines
        .iter()
        .map(|line| line.split("->").collect::<Vec<_>>())
        .map(|items| {
            let from = items[0].trim().split(',').collect::<Vec<_>>();
            let to = items[1].trim().split(',').collect::<Vec<_>>();
            let (x1, y1) = (
                from[0].parse::<i32>().unwrap(),
                from[1].parse::<i32>().unwrap(),
            );
            let (x2, y2) = (to[0].parse::<i32>().unwrap(), to[1].parse::<i32>().unwrap());
            [(x1, y1), (x2, y2)]
        })
        .collect::<Vec<[(i32, i32); 2]>>();
    println!("[*] vent lines: {:?}", vent_lines);

    // let from_lines = lines.iter()
    //     .map(|line| line.split("->").collect::<Vec<_>>())
    //     .map(|items| {
    //         let from = items[0].trim().split(',').collect::<Vec<_>>();
    //         let (x1, y1) = (from[0].parse::<i32>().unwrap(), from[1].parse::<i32>().unwrap());
    //         (x1, y1)
    //     })
    //     .collect::<Vec<_>>();
    // println!("[*] from_vent_lines: {:?}", from_lines);

    // let to_lines = lines.iter()
    //     // .map(|line| line.split("->").collect::<Vec<_>>())
    //     .map(|line| {
    //         let items = line.split("->").collect::<Vec<_>>();
    //         let to = items[1].trim().split(',').collect::<Vec<_>>();
    //         let (x2, y2) = (to[0].parse::<i32>().unwrap(), to[1].parse::<i32>().unwrap());
    //         (x2, y2)
    //     })
    //     .collect::<Vec<_>>();
    // println!("[*] to_vent_lines: {:?}", to_lines);

    // let x_size = vent_lines
    //     .iter()
    //     .fold(i32::MIN, |acc, tuple| {
    //         let max = acc.max(tuple[0].0.max(tuple[1].0));
    //         max
    //     });
    // println!("[*] x_size: {:?}", x_size);

    // let y_size = vent_lines
    //     .iter()
    //     .fold(i32::MIN, |acc, tuple| {
    //         let max = acc.max(tuple[0].1.max(tuple[1].1));
    //         max
    //     });
    // println!("[*] y_size: {:?}", y_size);
    
    let (x_size, y_size) = vent_lines
        .iter()
        .fold((i32::MIN, i32::MIN), |acc, tuple|     {
            let x_max = acc.0.max(tuple[0].0.max(tuple[1].0));
            let y_max = acc.1.max(tuple[0].1.max(tuple[1].1));
            (x_max, y_max)
        });
    println!("[*] x_size: {:?}, y_size: {:?}", x_size, y_size);

    let mut xy_diagram: Vec<i32> = vec![0; (x_size + 1) as usize * (y_size + 1) as usize];
    // top_left_corner is (0,0),
    // bottom_right_corner is (9,9)
    // .......1..
    // ..1....1..
    // ..1....1..
    // .......1..
    // .112111211
    // ..........
    // ..........
    // ..........
    // ..........
    // 222111....

    //-- horizontal lines and vertical lines marking/counting
    for [(x1, y1), (x2, y2)] in vent_lines.iter() {
        println!("({},{}) -> ({},{})", x1, y1, x2, y2);
        if *x1 == *x2 {
            let v_max = y1.max(y2);
            let v_min = y1.min(y2);
            println!("  y (v_min, v_max) -> x:{}, y:({}..{})", *x1, v_min, v_max);
            for y in *v_min..=*v_max {
                // xy_diagram[(*x1 as usize * x_size as usize) + y as usize] += 1;
                xy_diagram[*x1 as usize + (y as usize * x_size as usize)] += 1;
            }
        } else if *y1 == *y2 {
            let h_max = x1.max(x2);
            let h_min = x1.min(x2);
            println!("  x (min, max) -> x:({}..{}), y:{}", h_min, h_max, *y1);
            for x in *h_min..=*h_max {
                // xy_diagram[(x as usize * x_size as usize) + *y1 as usize] += 1;
                xy_diagram[x as usize + (*y1 as usize * x_size as usize)] += 1;
            }
        }
    }

    //---- Part Two: ----------------
    //-- diagonal lines marking/couting
    for [(x1, y1), (x2, y2)] in vent_lines.iter() {
        let is_diagonal = if i32::abs(*y2 - *y1) == i32::abs(*x2 - *x1) {
            true
        } else {
            false
        };
        println!(
            "({},{}) -> ({},{}), diagonal: {}",
            x1, y1, x2, y2, is_diagonal
        );

        if is_diagonal == true {
            if *x1 > *x2 {
                if *y1 > *y2 {
                    // x1 > x2, y1 > y2
                    println!("  case 1 ==>> x1 > x2, y1 > y2");
                    let mut x = *x1;
                    let mut y = *y1;
                    while x >= *x2 && y >= *y2 {
                        if i32::abs(*y1 - y) == i32::abs(*x1 - x) {
                            // println!("  diagonal-1: ({},{}) => {}", x, y, x as usize + (y as usize * x_size as usize));
                            xy_diagram[x as usize + (y as usize * x_size as usize)] += 1;
                        }
                        y -= 1;
                        x -= 1;
                    }
                } else {
                    // x1 > x2, y1 < y2
                    println!("  case 2 ==>> x1 > x2, y1 < y2");
                    let mut x = *x1;
                    let mut y = *y1;
                    while x >= *x2 && y <= *y2 {
                        if i32::abs(*y1 - y) == i32::abs(*x1 - x) {
                            // println!("  diagonal-3: ({},{}) => {}", x, y, x as usize + (y as usize * x_size as usize));
                            xy_diagram[x as usize + (y as usize * x_size as usize)] += 1;
                        }
                        y += 1;
                        x -= 1;
                    }
                }
            } else {
                if *y1 > *y2 {
                    // x1 < x2, y1 > y2
                    println!("  case 3 ==>> x1 < x2, y1 > y2");
                    let mut x = *x1;
                    let mut y = *y1;
                    while x <= *x2 && y >= *y2 {
                        if i32::abs(*y1 - y) == i32::abs(*x1 - x) {
                            // println!("  diagonal-4: ({},{}) => {}", x, y, x as usize + (y as usize * x_size as usize));
                            xy_diagram[x as usize + (y as usize * x_size as usize)] += 1;
                        }
                        y -= 1;
                        x += 1;
                    }
                } else {
                    // x1 < x2, y1 < y2
                    println!("  case 4 ==> x1 < x2, y1 < y2");
                    let mut x = *x1;
                    let mut y = *y1;
                    while x <= *x2 && y <= *y2 {
                        if i32::abs(*y1 - y) == i32::abs(*x1 - x) {
                            // println!("  diagonal-4: ({},{}) => {}", x, y, x as usize + (y as usize * x_size as usize));
                            xy_diagram[x as usize + (y as usize * x_size as usize)] += 1;
                        }
                        y += 1;
                        x += 1;
                    }
                }
            }
        }
    }

    let mut total_overlap_points_count = 0;
    for x in 0..=x_size {
        for y in 0..=y_size {
            let value = xy_diagram[(x as usize * x_size as usize) + y as usize];
            if value > 0 {
                print!(
                    "{:1}",
                    xy_diagram[(x as usize * x_size as usize) + y as usize]
                );
                if value >= 2 {
                    total_overlap_points_count += 1;
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!(
        "[**] Total Overlap Points Count = {}",
        total_overlap_points_count
    );
    //-- FOR  "input/day_5-sample.txt",
    // [**] Total Overlap Points Count = 13
    // 1.1....11.
    // .111...2..
    // ..2.1.111.
    // ...1.2.2..
    // .112313211
    // 1..1.2....
    // ..1...1...
    // .1.....1.1
    // 1.......12
    // 222111....

    //-- FOR  "input/day_5-input.txt",
    // [**] Total Overlap Points Count = 21466
}
