// advent/day_3.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

//--- Day 3: Binary Diagnostic ---
pub fn do_day_3() {
    day_3_part_one();
    day_3_part_two();
}

fn handle_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    println!("input lines count = {}", lines_count);

    lines
}

fn day_3_part_one() {
    println!("--- Day 3: Binary Diagnostic, Part 1 ---");
    // let filename = "input/day_3-sample.txt";
    let filename = "input/day_3-input.txt";
    let input_lines = handle_input(filename);

    let first_line = &input_lines[0];
    println!("first_line = {}", first_line);

    let input_width = first_line.len();
    println!("input_width = {}", input_width);

    println!("----- part one: -----------------");
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut gamma_rate_vec: Vec<i32> = Vec::new();
    let mut epsilon_rate_vec: Vec<i32> = Vec::new();

    for i in 0..input_width {
        let mut one_count = 0;
        let mut zero_count = 0;
        #[allow(unused_assignments)]
        let mut most_common_bit: i32 = 0;
        #[allow(unused_assignments)]
        let mut least_common_bit: i32 = 0;

        for line in input_lines.iter() {
            let line: Vec<char> = line.chars().collect();
            let bit_str = line[i];
            // print!("{} ", bit_str);
            match bit_str {
                '1' => one_count += 1,
                '0' => zero_count += 1,
                _ => (),
            }
        }

        // if one_count >= zero_count {
        //     most_common_bit = 1;
        //     least_common_bit = 0;
        // } else {
        //     most_common_bit = 0;
        //     least_common_bit = 1;
        // }
        (most_common_bit, least_common_bit) = if one_count >= zero_count {
            (1, 0)
        } else {
            (0, 1)
        };

        gamma_rate_vec.push(most_common_bit);
        epsilon_rate_vec.push(least_common_bit);
        // println!();
        // println!("most_common_bit[{}]: {:?}", i, most_common_bit);
        // println!("least_common_bit[{}]: {:?}", i, least_common_bit);
    }
    println!("  gamma_rate_vec:   {:?}", gamma_rate_vec);
    println!("  epsilon_rate_vec: {:?}", epsilon_rate_vec);

    for (i, value) in gamma_rate_vec.iter().rev().enumerate() {
        gamma_rate = gamma_rate + value * 2_i32.pow(i as u32);
    }

    for (i, value) in epsilon_rate_vec.iter().rev().enumerate() {
        epsilon_rate = epsilon_rate + value * 2_i32.pow(i as u32);
    }

    println!("  gamma_rate:   {}", gamma_rate);
    println!("  epsilon_rate:   {}", epsilon_rate);
    let power_consumption = gamma_rate * epsilon_rate;
    println!("[*] power_consumption: {}", power_consumption);
    //-- output
    // input lines count = 1000
    // first_line = 100000101101
    // input_width = 12
    // gamma_rate_vec:   [0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1]
    // epsilon_rate_vec: [1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0]
    // gamma_rate:   1337
    // epsilon_rate:   2758
    // power_consumption: 3687446  -- right answer
}

fn day_3_part_two() {
    println!("--- Day 3: Binary Diagnostic, Part 2 ---");
    let filename = "input/day_3-sample.txt";
    // let filename = "input/day_3-input.txt";
    let input_lines = handle_input(filename);

    let lines_count = input_lines.len();
    println!("input lines count = {}", lines_count);

    let first_line = &input_lines[0];
    println!("first_line = {}", first_line);

    let input_width = first_line.len();
    println!("input_width = {}", input_width);

    //-------------------------------------------------
    //--- part two --- life support rating
    //-------------------------------------------------
    // life support rating = oxygen generator rating * CO2 scrubber rating
    //
    // 1> oxygen generator rating value
    println!("1> oxygen generator rating value --");
    let mut lines_cloned: Vec<&String> = input_lines.iter().map(|item| item).collect();
    let mut oxygen_generator_rating_line: String = String::new();

    for i in 0..input_width {
        let mut one_count = 0;
        let mut zero_count = 0;

        for line in lines_cloned.iter() {
            let bit_str = line.chars().nth(i).unwrap();
            // print!("{} ", bit_str);
            match bit_str {
                '1' => one_count += 1,
                '0' => zero_count += 1,
                _ => (),
            }
        }

        // println!("i: {}, one_count: {}, zero_count: {}", i, one_count, zero_count);
        let mut filtered_values = Vec::new();
        if one_count >= zero_count {
            for line in lines_cloned.iter() {
                let bit_str = line.chars().nth(i).unwrap();
                // print!("{} ", bit_str);
                match bit_str {
                    '1' => {
                        filtered_values.push(line);
                    }
                    _ => (),
                }
            }
        } else {
            for line in lines_cloned.iter() {
                let bit_str = line.chars().nth(i).unwrap();
                // print!("{} ", bit_str);
                match bit_str {
                    '0' => {
                        filtered_values.push(line);
                    }
                    _ => (),
                }
            }
        }

        // println!("filtered_values: {:?}", filtered_values);
        if filtered_values.len() == 1 {
            oxygen_generator_rating_line = (*filtered_values[0]).clone();
            break;
        }

        lines_cloned = filtered_values.iter().map(|item| **item).collect();
    }
    println!( "  oxygen_generator_rating_line: {}", oxygen_generator_rating_line );
    
    let mut oxygen_generating_rate_value = 0;

    for (i, bit) in oxygen_generator_rating_line.chars().rev().enumerate() {
        // print!("i:{}, bit:{} / ", i, bit);
        let value = match bit {
            '1' => 1,
            _ => 0,
        };
        oxygen_generating_rate_value = oxygen_generating_rate_value + value * 2i32.pow(i as u32);
    }
    println!( "  oxygen_generating_rate_value: {}", oxygen_generating_rate_value );

    //------------------------------------------
    // 2> CO2 scrubber rating value
    //
    println!("2> CO2 scrubber rating valu ---");
    let mut lines_cloned: Vec<&String> = input_lines.iter().map(|item| item).collect();
    let mut co2_scrubber_rating_line: String = String::new();

    for i in 0..(input_width) {
        let mut one_count = 0;
        let mut zero_count = 0;

        for line in lines_cloned.iter() {
            let bit_str = line.chars().nth(i).unwrap();
            // print!("{} ", bit_str);
            match bit_str {
                '1' => one_count += 1,
                '0' => zero_count += 1,
                _ => (),
            }
        }

        // println!("i: {}, one_count: {}, zero_count: {}", i, one_count, zero_count);
        let mut filtered_values = Vec::new();
        if zero_count <= one_count {
            for line in lines_cloned.iter() {
                let bit_str = line.chars().nth(i).unwrap();
                match bit_str {
                    '0' => {
                        filtered_values.push(line);
                    }
                    _ => (),
                }
            }
        } else {
            for line in lines_cloned.iter() {
                let bit_str = line.chars().nth(i).unwrap();
                match bit_str {
                    '1' => {
                        filtered_values.push(line);
                    }
                    _ => (),
                }
            }
        }

        // println!("filtered_values: {:?}", filtered_values);
        if filtered_values.len() == 1 {
            co2_scrubber_rating_line = (*filtered_values[0]).clone();
            break;
        }

        lines_cloned = filtered_values.iter().map(|item| **item).collect();
    }
    println!("  co2_scrubber_rating_line: {}", co2_scrubber_rating_line);

    let mut co2_scrubber_rate_value = 0;

    for (i, bit) in co2_scrubber_rating_line.chars().rev().enumerate() {
        // print!("i:{}, bit:{} / ", i, bit);
        let value = match bit {
            '1' => 1,
            _ => 0,
        };
        co2_scrubber_rate_value = co2_scrubber_rate_value + value * 2i32.pow(i as u32);
    }
    println!("  co2_scrubber_rate_value: {}", co2_scrubber_rate_value);

    let life_support_rating = oxygen_generating_rate_value * co2_scrubber_rate_value;
    println!("[*] life_support_rating: {}", life_support_rating);
    // ----- part two: -----------------
    // ----- part two: 1> oxygen generator rating value --
    // oxygen_generator_rating_string: 011000111111
    // oxygen_generating_rate_value: 1599
    // ----- part two: 2> CO2 scrubber rating valu ---
    // co2_scrubber_rating_string: 101011000100
    // co2_scrubber_rate_value: 2756
    // [*] life_support_rating: 4406844 <<-- Right Answer
}
