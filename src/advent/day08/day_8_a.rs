// advent/day_6.rs
use std::collections::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

///////////////////////////////////////////
//--- Day 8: Seven Segment Search ---
///////////////////////////////////////////
//-----------------------------------------
//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
//------------------------------------------

pub fn do_day_8_a() {
    day_8_part_one();
    day_8_part_two();
}

fn handle_input(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    println!("[*] Input Filename: {}", filename);
    println!("[*] input lines count = {}", lines_count);

    // let first_line = &lines[0];
    // println!("[ ] First Line: {}", first_line);

    let input_lines = lines
        .iter()
        .map(|line| line.split('|').collect::<Vec<_>>())
        .map(|items| {
            let left = items[0]
                .trim()
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>();
            let right = items[1]
                .trim()
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>();
            // println!(" {:?} | {:?}", left, right);
            (left, right)
        })
        .collect::<Vec<(Vec<_>, Vec<_>)>>();
    // println!("[*] input_lines: {:?}", input_lines);

    input_lines
}

fn day_8_part_one() {
    println!("===============================================");
    println!("---Day 8: Seven Segment Search, Part One ---, 1/11/2022 ==> DONE");
    println!("===============================================");
    // let filename = "input/day_8-sample.txt";
    let filename = "input/day_8-input.txt";
    let input_lines = handle_input(filename);
    // input_lines.iter()
    //     .for_each(|line| println!(" {:?} | {:?}", line.0, line.1));
    println!("input_lines: {:?}", input_lines.len());

    let right_lines = input_lines
        .iter()
        .map(|line| line.1.clone())
        .collect::<Vec<_>>();

    right_lines
        .iter()
        .for_each(|line| println!(" | {:?}", line));

    let output_value = right_lines.iter().fold(0, |acc, line| {
        let mut count = 0;
        line.iter().for_each(|item| {
            let item_len = item.len();
            if item_len == 2 || item_len == 3 || item_len == 4 || item_len == 7 {
                count += 1;
            }
        });
        acc + count
    });

    println!(
        "output_value (count of 1, 4, 7, 8 digits): {:?}",
        output_value
    );

    println!("-----------------------------------------------");
}

//===============================================
// output_value (count of 1, 4, 7, 8 digits): 381

fn day_8_part_two() {
    println!("===============================================");
    println!("---Day 8: Seven Segment Search, Part Two ---, 1/11/2022 ==> DONE");
    // println!("===============================================");
    // let filename = "input/day_8-sample.txt";
    let filename = "input/day_8-input.txt";
    let input_lines = handle_input(filename);
    // input_lines.iter()
    //     .for_each(|line| println!(" {:?} \n    | {:?}", line.0, line.1));
    println!("input_lines: {:?}", input_lines.len());

    // let (A, B, C, D, E, F, G) = (0, 1, 2, 3, 4, 5, 6);
    // println!("output_value (count of 1, 4, 7, 8 digits): {:?}", output_value);
    let bitmap_template = make_bitmap_template();

    // let mut total_sum: i32 = 0;
    let mut output_number_list = Vec::new();
    // input_lines.iter().skip(7).take(3).enumerate()
    input_lines
        .iter()
        .enumerate()
        .for_each(|(index, (left, right))| {
            println!(
                "[{}] -------------------------------------------------",
                index
            );
            println!("  [input_line]: {:?} | ", left);
            // println!("  [input_line]: {:?} \n  | {:?} ", left, right);

            let digit_template = make_digit_template(&left);
            println!("  [digit_template] -  {:?}", digit_template);

            let mut digit_map: HashMap<char, u8> = HashMap::new();
            for (i, c) in digit_template.iter().enumerate() {
                digit_map.insert(*c as char, i as u8);
            }

            // let mut right_sorted = right.clone();
            // right_sorted.sort_by_key(|a| a.len());

            let mut output_value_list: Vec<u8> = Vec::new();
            for (_i, word) in right.iter().enumerate() {
                // [0, *1, 2, 3, *4, 5, 6, *7, 8, 9]
                if word.len() == 2 {
                    output_value_list.push(1);
                }
                if word.len() == 3 {
                    output_value_list.push(7);
                }
                if word.len() == 4 {
                    output_value_list.push(4);
                }

                // word.len() == 5: ->  (2, 3, 5)
                // [0, *1, *2, *3, *4, *5, 6, *7, 8, 9]
                if word.len() == 5 {
                    let chars_vec = word.chars().collect::<Vec<_>>();
                    let mut bitmap = 0u8;
                    for (_i, c) in chars_vec.iter().enumerate() {
                        let v = digit_map.get(c).unwrap();
                        // println!("word.len() == 5, i:{}, ==> c:[{}], v:{}", i, c, v);
                        bitmap = bitmap | (1u8 << (6 - v));
                        // println!("word.len() == 5, i:{}, c:[{}] ==> {:#010b} ({})", i, c, bitmap, bitmap);
                    }
                    // println!("word.len() == 5, {} ==> {:#010b}, dec: {}", word, bitmap, bitmap);

                    let mut output_value = 0;
                    for (k, v) in bitmap_template.iter().enumerate() {
                        if *v == bitmap {
                            output_value = k as u8;
                            break;
                        }
                    }
                    output_value_list.push(output_value);
                    // println!("word.len() == 5, {} -> {}", word, output_value);
                }

                // word.len() == 6: ->  (0, 6, 9)
                // [*0, *1, *2, *3, *4, *5, *6, *7, 8, *9]
                if word.len() == 6 {
                    let chars_vec = word.chars().collect::<Vec<_>>();
                    let mut bitmap = 0u8;
                    for (_i, c) in chars_vec.iter().enumerate() {
                        let v = digit_map.get(c).unwrap();
                        // println!("word.len() == 6, i:{}, ==> c:[{}], v:{}", i, c, v);
                        bitmap = bitmap | (1u8 << (6 - v));
                        // println!("word.len() == 6, i:{}, c:[{}] ==> {:#010b} ({})", i, c, bitmap, bitmap);
                    }
                    // println!("word.len() == 6, {} ==> {:#010b}, dec: {}", word, bitmap, bitmap);

                    let mut output_value = 0;
                    for (k, v) in bitmap_template.iter().enumerate() {
                        if *v == bitmap {
                            output_value = k as u8;
                            break;
                        }
                    }
                    output_value_list.push(output_value);
                    // println!("word.len() == 6, {} -> {}", word, output_value);
                }

                // [*0, *1, *2, *3, *4, *5, *6, *7, *8, *9]
                if word.len() == 7 {
                    // println!("word.len() == 7, {} -> {}", word, 8);
                    output_value_list.push(8);
                }
            }

            let mut output_number: u32 = 0;
            for (i, v) in output_value_list.iter().rev().enumerate() {
                output_number += (*v as u32) * (10_i32.pow(i as u32) as u32);
            }
            // println!("[*] [output]: {:?} -> {:?} ({})", right, output_value_list, output_number);
            println!(
                "\n  {:?} -> [{}] {:?} ",
                right, output_number, output_value_list
            );
            output_number_list.push(output_number as i32);
            println!("-----------------------------------------------");
        });

    let total_sum = output_number_list.iter().fold(0, |acc, v| acc + v);
    println!(
        "[**] total_sum: {} (lines: {})",
        total_sum,
        input_lines.len()
    );
    println!("-----------------------------------------------");
}

// output ---------------------------------------
//-- for input: input/day_8-sample.txt
// -----------------------------------------------
// [**] total_sum: 61229 (lines: 10)
// -----------------------------------------------

// output ---------------------------------------
//-- for input: input/day_8-input.txt
// -----------------------------------------------
// [**] total_sum: 1023686 (lines: 200) <-- CORRECT
// -----------------------------------------------

fn make_digit_template(left: &Vec<String>) -> Vec<char> {
    // A B C D E F G
    // 0 1 2 3 4 5 6
    #[allow(non_snake_case)]
    let (A, B, C, D, E, F, G) = (0, 1, 2, 3, 4, 5, 6);
    let mut digit_template = [' '; 7];

    let mut sorted_left = left.clone();
    sorted_left.sort_by_key(|a| a.len());

    // println!("sorted-left input: {:?}", sorted_left);
    //-----------------------------------------------------------
    //-- len 2 -> 1 one
    let len2 = sorted_left
        .iter()
        .filter(|word| word.len() == 2)
        .collect::<Vec<_>>();
    let len2 = len2.first().unwrap();
    digit_template[F] = len2.chars().nth(1).unwrap();
    digit_template[C] = len2.chars().nth(0).unwrap();
    println!("  len 2: 1 one -  {:?}", len2);
    println!("  len 2 (1): digit_template -  {:?}", digit_template);

    //-----------------------------------------------------------
    //-- len 3 -> 7 seven
    let len3 = sorted_left
        .iter()
        .filter(|word| word.len() == 3)
        .collect::<Vec<_>>();
    let len3 = len3.first().unwrap();
    let word_str = len3.to_string();
    for (_i, v) in word_str.chars().enumerate() {
        if v != digit_template[C] && v != digit_template[F] {
            digit_template[A as usize] = v;
        }
    }
    println!("  len 3: 7 seven - {}", word_str);
    println!("  len 3 (7): digit_template -  {:?}", digit_template);

    //-----------------------------------------------------------
    //-- len 4 -> 4 four
    let len4 = sorted_left
        .iter()
        .filter(|word| word.len() == 4)
        .collect::<Vec<_>>();
    let len4 = len4.first().unwrap();
    let word_str = len4.to_string();
    for (_i, v) in word_str.chars().enumerate() {
        if v == digit_template[C] || v == digit_template[F] {
            continue;
        }
        if digit_template[B] == ' ' {
            digit_template[B as usize] = v;
        } else {
            digit_template[D as usize] = v;
        }
    }
    println!("  len 4: 4 four - {}", word_str);
    println!("  len 4 (4): digit_template -  {:?}", digit_template);

    //-----------------------------------------------------------
    //-- len 5 -> one of (2 two, 3 three, 5 five)
    let len5_list = sorted_left
        .iter()
        .filter(|word| word.len() == 5)
        .collect::<Vec<_>>();

    let three_common_chars = |list: &Vec<&String>| -> Vec<char> {
        let first_list = list[0].to_string();
        let second_list = list[1].to_string();
        let third_list = list[2].to_string();
        let mut common_chars = Vec::new();

        for c in first_list.chars() {
            if second_list.contains(c) && third_list.contains(c) {
                // println!("len 5: : contains({}), {:?}, {:?}", c, second_list, third_list);
                common_chars.push(c);
            }
        }

        common_chars
    };

    let comm_3_chars = three_common_chars(&len5_list);
    // println!("len 5: 3 list: {:?}", len5_list);
    println!("  len 5 (2,3,5): 3-len common chars: {:?}", comm_3_chars);

    //-----------------------------------------------------------
    //-- swap B and D value if necessary upon comm_3_chars check
    let double_check_b_d_position = |digit_template: &mut [char; 7], comm_3_chars: &Vec<char>| {
        // let (*A, B, C, *D, E, F, *G) = (*0, 1, 2, *3, 4, 5, *6);
        if comm_3_chars.contains(&digit_template[D]) != true {
            let temp = digit_template[D];
            digit_template[D] = digit_template[B];
            digit_template[B] = temp;
        }
    };
    double_check_b_d_position(&mut digit_template, &comm_3_chars);

    let find_g_pos_char = |comm_3_chars: &Vec<char>| -> char {
        let mut g_char = ' ';
        for c in comm_3_chars.iter() {
            if digit_template.contains(c) != true {
                g_char = *c;
            }
        }
        g_char
    };

    let g_position_char = find_g_pos_char(&comm_3_chars);
    digit_template[G as usize] = g_position_char;
    // println!("len 5: g_position_char -  {}", g_position_char);
    // println!("len 5 (2,3,5): digit_template -  {:?}", digit_template);

    let find_e_pos_char = |len5_list: &Vec<&String>| -> char {
        let mut e_char = ' ';
        len5_list.iter().for_each(|word| {
            for c in word.chars() {
                if digit_template.contains(&c) != true {
                    e_char = c;
                }
            }
        });
        e_char
    };

    // println!("-- BEFORE e_position: {:?}", digit_template);
    let e_position_char = find_e_pos_char(&len5_list);
    digit_template[E as usize] = e_position_char;
    // println!("len 5: e_position_char -  {}", e_position_char);
    println!("    -- AFTER  e_position: {:?}", digit_template);

    println!("    -- before len = 6, {:?}", digit_template);
    //-----------------------------------------------------------
    // -- len 6 -> one of (0 zero, 6 six, 9 nine)
    let len6_list = sorted_left
        .iter()
        .filter(|word| word.len() == 6)
        .collect::<Vec<_>>();

    let mut found_c_pos_count = 0;
    // let mut found_f_pos_count = 0;
    // let mut found_9 = false;
    len6_list.iter().for_each(|word| {
        println!("    -- word.len = 6; {}", word);
        if word.contains(digit_template[C]) {
            found_c_pos_count += 1;
        }
    });

    //-- adjust/swap temp[C] and temp[F]
    if found_c_pos_count != 2 {
        let tmp = digit_template[C];
        digit_template[C] = digit_template[F];
        digit_template[F] = tmp;
    }

    //-----------------------------------------------------------
    println!("    final [digit_template] -  {:?}", digit_template);
    let d_template = digit_template.to_vec();
    d_template
}

fn make_bitmap_template() -> Vec<u8> {
    // let mut zero = 0x00;
    // let mut one = 0x00;
    // let mut two = 0x00;
    // let mut three = 0x00;
    // let mut four = 0x00;
    // let mut five = 0x00;
    // let mut six = 0x00;
    // let mut seven = 0x00;
    // let mut eight = 0x00;
    // let mut nine = 0x00;

    let one = 0b00010010u8;
    let four = 0b0111010u8;
    let seven = 0b01010010u8;

    //-- 5 chars: 2, 3, 5
    let two = 0b01011101u8;
    let three = 0b01011011u8;
    let five = 0b01101011u8;

    //-- 6 chars: 0, 6, 9
    let zero = 0b01110111u8;
    let six = 0b01101111u8;
    let nine = 0b01111011u8;

    let eight = 0b01111111u8;

    let bitmap_template: Vec<u8> = vec![zero, one, two, three, four, five, six, seven, eight, nine];

    // bitmap_template.iter().enumerate()
    //     .for_each(|(i,bitmap)| println!("   bitmap[{}]: {:#010b} ({})", i, bitmap, bitmap));

    bitmap_template
}
// fn double_check_b_d_position(digit_template: &mut [char; 7], comm_3_chars: &Vec<char>) {
//     let (A, B, C, D, E, F, G) = (0, 1, 2, 3, 4, 5, 6);
//     if comm_3_chars.contains(&digit_template[D]) != true {
//         let temp = digit_template[D];
//         digit_template[D] = digit_template[B];
//         digit_template[B] = temp;
//     }
// }

// ===============================================
// --- Day 7: Lanternfish, Part Two ---, 1/9/2022 ==> DONE
// ===============================================
// [*] Input Filename: input/day_7-input.txt
// [*] input lines count = 1
// total feul (min_distance): 95851339, min_position: 461
// -----------------------------------------------
