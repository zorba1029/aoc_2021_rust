// advent/day_10.rs
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

///////////////////////////////////////////
//---   Day 10: Syntax Scoring          ---
///////////////////////////////////////////

// the navigation subsystem (the input data)
// the navigation subsystem syntax is made of several lines containing chunks:
// There are one or ore chunks on each line, and
// chunks contain zero or more other chunks.
// Adjacent chunks are not separated by any delimiter;
//   if one chunk stops, the next chunk (if any) can immediately start.
// Every chunk must open and close with one of four legal pairs of matching characters:
//  - If a chunk opens with (, if must close with ).
//  - If a chunk opens with [, it must close with ].
//  - If a chunk opens with {, it must close with }.
//  - If a chunk opens with <, it must close with >.
// Some lines are imcomplete, but others are corrrupted.
// Find and discard the corrupted lines first.

pub fn do_day_10() {
    day_10_part_one();
    day_10_part_two();
}

fn day_10_part_one() {
    println!("===============================================");
    println!("--- Day 10: Syntax Scoring, Part One ---, 1/22/2022 ==> DONE");
    println!("===============================================");
    // let filename = "input/day_10-sample.txt";
    let filename = "input/day_10-input.txt";
    let input_lines = handle_input(filename);
    println!("input_lines: {:?}", input_lines.len());

    let chunks_table = make_chunks_table(&input_lines);
    let (_corrupted_lines, corrupted_values) = select_corrupted_lines(chunks_table);

    println!("-----------------------------------------");
    println!(
        "[Part 1] Total Syntax Error Score: {}",
        corrupted_values.iter().sum::<i32>()
    );
    println!("-----------------------------------------");
}
// -----------------------------------------
// -- FOR input/day_10-sample.txt
// Total Syntax Error Score: 26397
// -- FOR input/day_10-input.txt
// Total Syntax Error Score: 462693
// -----------------------------------------

fn day_10_part_two() {
    println!("===============================================");
    println!("--- Day 10: Syntax Scoring, Part Two ---, 1/22/2022 ==> DONE");
    println!("===============================================");
    // let filename = "input/day_10-sample.txt";
    let filename = "input/day_10-input.txt";
    let input_lines = handle_input(filename);
    println!("input_lines: {:?}", input_lines.len());

    let chunks_table = make_chunks_table(&input_lines);
    let incomplete_lines = select_incomplete_lines(chunks_table);
    let completion_lines = make_completion_lines(incomplete_lines);
    let sorted_completion_scores = get_completion_scores(completion_lines);

    let middle_index = sorted_completion_scores.len() / 2 as usize;
    let middle_score = sorted_completion_scores[middle_index];

    println!("-----------------------------------------");
    println!(
        "Middle Completion Score: {}, index:[{}]",
        middle_score, middle_index
    );
    println!("-----------------------------------------");
}

//-- FOR input/day_10-sample.txt
// ===============================================
// --- Day 10: Syntax Scoring, Part Two ---, 1/22/2022 ==> DONE
// ===============================================
// [*] Input Filename: input/day_10-sample.txt
// [*] input lines count = 10
// [ ] First Line: len=24, [({(<(())[]>[[{[]{<()<>>,
// input_lines: 10
// [*] completion_lines with score:  -------------
// [0]: } } ] ] ) } ) ]  - 288957
// [1]: ) } > ] } )  - 5566
// [2]: } } > } > ) ) ) )  - 1480781
// [3]: ] ] } } ] } ] } >  - 995444
// [4]: ] ) } >  - 294
// -----------------------------------------
// Total Completion Score: 2771042
// Middle Completion Score: 288957, index:[2]
// -----------------------------------------

//-- FOR input/day_10-input.txt
// ===============================================
// --- Day 10: Syntax Scoring, Part Two ---, 1/22/2022 ==> DONE
// ===============================================
// [*] Input Filename: input/day_10-input.txt
// [*] input lines count = 90
// [ ] First Line: len=99, ({[[{{({<<<<(<{}()><()[]>)[([]{}){<>[]}]><[{<>()}{<><>}]<(()[])([]<>)>>>){<([(())<()<>>][{{}{}}[(){,
// input_lines: 90
// [*] completion_lines with score:  -------------
// [0]: ] ) ) > > } > } } )  - 4452466
// [1]: ) } ) > ) ] ) > > } } > ] >  - 2043905489
// [2]: ] ] ] ] } > > } } } } > } )  - 3054667991
// [3]: ) ) > ] > > > } ] ] > ) ) ]  - 1689429907
// [4]: ) ) ) > ] ) > ] ] ) ] ) } } ]  - 7786910842
// [5]: ) } ) > > } > ] } > ) > ] > ]  - 10251779322
// [6]: ] > ) ] ] } } ] > ] } ] ) )  - 3491686056
// [7]: ) } ] ] ] ) ) > > > ) }  - 82990608
// [8]: > ] ] > ) ) > ] }  - 1763363
// [9]: ] ] } ) > ] ) } > } > ) ] )  - 3094671161
// [10]: > } } } ) ] > ) > > ] > ) }  - 5794093483
// [11]: ] > ] > ] } ] > } } ) ) )  - 711999156
// [12]: ) ] } > } ] ) } > > ) ) > ] )  - 9506560236
// [13]: } ) ] ] ) ) } ) ) > > ] > >  - 4026037449
// [14]: ] } > } > ) ) ) > > ) ] > >  - 3406749574
// [15]: > ) ] } ] } ) ) ) > > ] > > )  - 26295421621
// [16]: ] ) ] > } > } > } ) > } } ) )  - 14149974831
// [17]: ] ] > > ) } ) ] ]  - 1013537
// [18]: } ) } ) > > ) ] } ) ) >  - 162879784
// [19]: } } } > } > ) ) ) > ] ) > >  - 4587599674
// [20]: } } } ) > > } ) }  - 1459333
// [21]: } } ] ) ) ) > > } ) ] > ) )  - 4504682231
// [22]: ) } } ) } > > > > > ] } } ) )  - 10585936081
// [23]: ) } > } ] > > > } } } ] >  - 436717964
// [24]: } ) > > ) ] > ) } ] ] > > > }  - 20718492498
// [25]: ) ) > > > ] ) ) ] } } )  - 68316591
// [26]: } > } > ) ) ] ] } )  - 7722816
// [27]: } ) ] } ] ] } ) > ] ) } ] ]  - 4038154587
// [28]: ] ) } } > ) > } > } > > ] ]  - 2869905612
// [29]: ) ) ] } ] } ] ) ] } ] } } }  - 1597055343
// [30]: > ) ] > > ] } ) > > > > } }  - 5272531243
// [31]: ) > > > > > ] } ] > ] ]  - 97649112
// [32]: ) ) } } > ) ) } } > > ) } ]  - 1648965542
// [33]: ] > > ] > ] ] > ) > ) ) } }  - 3641630793
// [34]: ] ) ) } > > } ) ] ) } ]  - 110932167
// [35]: ) > ) > ) } } } } ) ] ) } >  - 2288572794
// [36]: } > } ] > } ) > > ) ] > } >  - 4813825994
// [37]: ] ) ) > ) } ) > ) ) ) ] ] ]  - 2776707062
// [38]: ) > } > ] > > > } ) ] > ) ] )  - 11943333036
// [39]: } > ) > ) > ] ) > > > ] ) ]  - 4730265557
// [40]: > } > } ] > } ) ] ] > ] } >  - 5845570569
// [41]: > > > } } ] ) ] ] } } } > )  - 6090742971
// [42]: ) ) ) ) ) } } ] ) > ] ) ] ]  - 1526834037
// [43]: > } } } ) } } ] ] ] > > ] } )  - 28972071816
// [44]: ) > ) > ] } ) } ] > }  - 18322948
// --------------------------------------------------
// Middle Completion Score: 3094671161, index:[22]
// --------------------------------------------------

fn handle_input(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    println!("[*] Input Filename: {}", filename);
    println!("[*] input lines count = {}", lines_count);

    let first_line = &lines[0];
    println!("[ ] First Line: len={}, {}, ", first_line.len(), first_line);

    let input_lines = lines
        .iter()
        .map(|line| {
            let chunks = line
                .chars()
                .into_iter()
                // .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            // println!("{:?}", chunks);
            chunks
        })
        .collect::<Vec<Vec<_>>>();

    input_lines
}

fn make_chunks_table(input_lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let lines_count = input_lines.len();
    let line_len = input_lines[0].len();
    let mut chunks_table: Vec<Vec<char>> = Vec::with_capacity(lines_count);

    //-- chunks_table vector of vector
    input_lines.iter().enumerate().for_each(|(i, line)| {
        let v: Vec<char> = Vec::with_capacity(line_len);
        chunks_table.push(v.clone());
        line.iter().enumerate().for_each(|(_j, item)| {
            chunks_table[i].push(*item);
        });
        // println!("chunks_table[{}](len:{}) = {:?}", i, chunks_table[i].len(), chunks_table[i]);
    });

    // println!("Compact Form: chunks_table[i][j]  ---------");
    // chunks_table.iter().enumerate().for_each(|(i, line)| {
    //     line.iter().enumerate().for_each(|(j, _item)| {
    //         print!("{} ", chunks_table[i][j]);
    //     });
    //     println!();
    // });

    chunks_table
}

fn select_corrupted_lines(chunks_table: Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<i32>) {
    //-- 1) corrupted line: one where a chunk closes with the wroing character
    //   ex) (], {()()()>, (((()))}, and <([]){()}[{}]).
    //-- 2) incomplete lines
    //-- problem) score each corrupted line which has syntax error

    let open_chars = "([{<".to_string();
    // let close_chars = ")]}>".to_string();
    // let mut valid_lines: Vec<Vec<char>> = vec![];
    let mut corrupted_lines: Vec<Vec<char>> = vec![];
    let mut corrupted_values: Vec<i32> = vec![];
    // let mut incomplete_lines: Vec<Vec<char>> = vec![];

    for (i, line) in chunks_table.iter().enumerate() {
        let mut stack: Vec<char> = vec![];
        // let mut corrupted = false;

        // println!(" [{}] line-len: {}, {:?}", i, line.len(), line);

        stack.push(*line.first().unwrap());
        for (j, in_ch) in line.iter().skip(1).enumerate() {
            // println!(" [{}][{}] B-stack: {:?}", i, j+1, stack);
            // println!(" [{}][{}] in_char: {}", i, j+1, in_ch);
            let top_ch = stack.last();

            if open_chars.contains(*in_ch) {
                stack.push(*in_ch);
            } else {
                if let Some(top_ch) = top_ch {
                    match *in_ch {
                        ')' => {
                            if *top_ch == '(' {
                                stack.pop();
                            } else {
                                // corrupted = true;
                                corrupted_values.push(3);
                                corrupted_lines.push(line.to_vec());
                                // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                break;
                            }
                        }
                        ']' => {
                            if *top_ch == '[' {
                                stack.pop();
                            } else {
                                // corrupted = true;
                                corrupted_values.push(57);
                                corrupted_lines.push(line.to_vec());
                                // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                break;
                            }
                        }
                        '}' => {
                            if *top_ch == '{' {
                                stack.pop();
                            } else {
                                // corrupted = true;
                                corrupted_values.push(1_197);
                                corrupted_lines.push(line.to_vec());
                                // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                break;
                            }
                        }
                        '>' => {
                            if *top_ch == '<' {
                                stack.pop();
                            } else {
                                // corrupted = true;
                                corrupted_values.push(25_137);
                                corrupted_lines.push(line.to_vec());
                                // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                break;
                            }
                        }
                        //-- mismatch
                        _ => {
                            println!(" ** [{}][{}] mismatch in_char: {}", i, j + 1, in_ch);
                            break;
                        }
                    }
                }
            }
            // println!(" [{}][{}] A- stack: {:?} -------\n", i, j+1, stack);
        }

        // if stack.len() <= 0 && corrupted != true {
        //     println!("  --> valid line: ");
        //     // valid_lines.push(line.to_vec());
        // } else {
        //     if corrupted {
        //         // println!("  --> corrupted line: ");
        //         // corrupted_lines.push(line.to_vec());
        //     } else  {
        //         // println!("  --> incomplete line: ");
        //         incomplete_lines.push(line.to_vec());
        //     }
        // }
    }

    //-- DEBUG PRINT
    // println!("[*] incomplete_lines: {} -----------", incomplete_lines.len());
    // for i in 0..incomplete_lines.len() {
    //     for j in 0..incomplete_lines[i].len() {
    //         print!("{}", incomplete_lines[i][j]);
    //     }
    //     println!();
    // }

    // println!("[*] corrupted_lines: {} -------------", corrupted_lines.len());
    // for i in 0..corrupted_lines.len() {
    //     for j in 0..corrupted_lines[i].len() {
    //         print!("{}", corrupted_lines[i][j]);
    //     }
    //     println!();
    // }

    // let total_syntax_score = corrupted_values.iter().sum::<i32>();
    // println!("[*] Total Syntax Error Score: {}", total_syntax_score);

    (corrupted_lines, corrupted_values)
}

fn select_incomplete_lines(chunks_table: Vec<Vec<char>>) -> Vec<Vec<char>> {
    //-- 1) corrupted line: one where a chunk closes with the wroing character
    //   ex) (], {()()()>, (((()))}, and <([]){()}[{}]).
    //-- 2) incomplete lines
    //-- problem) score each corrupted line which has syntax error

    let open_chars = "([{<".to_string();
    // let close_chars = ")]}>".to_string();
    // let mut valid_lines: Vec<Vec<char>> = vec![];
    let mut corrupted_lines: Vec<Vec<char>> = vec![];
    let mut corrupted_values: Vec<i32> = vec![];
    let mut incomplete_lines: Vec<Vec<char>> = vec![];

    for (i, line) in chunks_table.iter().enumerate() {
        let mut stack: Vec<char> = vec![];
        let mut corrupted = false;

        // println!(" [{}] line-len: {}, {:?}", i, line.len(), line);

        stack.push(*line.first().unwrap());
        for (j, in_ch) in line.iter().skip(1).enumerate() {
            // println!(" [{}][{}] B-stack: {:?}", i, j+1, stack);
            // println!(" [{}][{}] in_char: {}", i, j+1, in_ch);
            let top_ch = stack.last();

            if open_chars.contains(*in_ch) {
                stack.push(*in_ch);
            } else {
                match top_ch {
                    Some(top_ch) => {
                        match *in_ch {
                            ')' => {
                                if *top_ch == '(' {
                                    stack.pop();
                                } else {
                                    corrupted = true;
                                    corrupted_values.push(3);
                                    corrupted_lines.push(line.to_vec());
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            ']' => {
                                if *top_ch == '[' {
                                    stack.pop();
                                } else {
                                    corrupted = true;
                                    corrupted_values.push(57);
                                    corrupted_lines.push(line.to_vec());
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            '}' => {
                                if *top_ch == '{' {
                                    stack.pop();
                                } else {
                                    corrupted = true;
                                    corrupted_values.push(1_197);
                                    corrupted_lines.push(line.to_vec());
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            '>' => {
                                if *top_ch == '<' {
                                    stack.pop();
                                } else {
                                    corrupted = true;
                                    corrupted_values.push(25_137);
                                    corrupted_lines.push(line.to_vec());
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            //-- mismatch
                            _ => {
                                println!(" ** [{}][{}] mismatch in_char: {}", i, j + 1, in_ch);
                                break;
                            }
                        }
                    }
                    None => {}
                }
            }
            // println!(" [{}][{}] A- stack: {:?} -------\n", i, j+1, stack);
        }

        if stack.len() <= 0 && corrupted != true {
            println!("  --> valid line: ");
            // valid_lines.push(line.to_vec());
        } else {
            if corrupted {
                // println!("  --> corrupted line: ");
                // corrupted_lines.push(line.to_vec());
            } else {
                // println!("  --> incomplete line: ");
                incomplete_lines.push(line.to_vec());
            }
        }
    }

    //-- DEBUG PRINT
    // println!("[*] incomplete_lines: {} -----------", incomplete_lines.len());
    // for i in 0..incomplete_lines.len() {
    //     for j in 0..incomplete_lines[i].len() {
    //         print!("{}", incomplete_lines[i][j]);
    //     }
    //     println!();
    // }

    // println!("[*] corrupted_lines: {} -------------", corrupted_lines.len());
    // for i in 0..corrupted_lines.len() {
    //     for j in 0..corrupted_lines[i].len() {
    //         print!("{}", corrupted_lines[i][j]);
    //     }
    //     println!();
    // }

    // let total_syntax_score = corrupted_values.iter().sum::<i32>();
    // println!("[*] Total Syntax Error Score: {}", total_syntax_score);

    incomplete_lines
}

fn make_completion_lines(incomplete_lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let open_chars = "([{<".to_string();
    let mut completion_lines: Vec<Vec<char>> = vec![];

    for (i, line) in incomplete_lines.iter().enumerate() {
        let mut stack: Vec<char> = vec![];
        let mut completion_stack: Vec<char> = vec![];

        // println!(" [{}] line-len: {}, {:?}", i, line.len(), line);

        stack.push(*line.first().unwrap());
        for (j, in_ch) in line.iter().skip(1).enumerate() {
            // println!(" [{}][{}] B-stack: {:?}", i, j+1, stack);
            // println!(" [{}][{}] in_char: {}", i, j+1, in_ch);
            let top_ch = stack.last();

            if open_chars.contains(*in_ch) {
                stack.push(*in_ch);
            } else {
                match top_ch {
                    Some(top_ch) => {
                        match *in_ch {
                            ')' => {
                                if *top_ch == '(' {
                                    stack.pop();
                                } else {
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            ']' => {
                                if *top_ch == '[' {
                                    stack.pop();
                                } else {
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            '}' => {
                                if *top_ch == '{' {
                                    stack.pop();
                                } else {
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            '>' => {
                                if *top_ch == '<' {
                                    stack.pop();
                                } else {
                                    // println!(" * [{}][{}] corrupted in_char: {}", i, j+1, in_ch);
                                    break;
                                }
                            }
                            //-- mismatch
                            _ => {
                                println!(" ** [{}][{}] mismatch in_char: {}", i, j + 1, in_ch);
                                break;
                            }
                        }
                    }
                    None => {}
                }
            }
            // println!(" [{}][{}] A- stack: {:?} -------\n", i, j+1, stack);
        }

        if stack.len() <= 0 {
            println!("  --> valid line: ");
            // valid_lines.push(line.to_vec());
        } else {
            // println!("  --> incomplete line: ");
            stack
                .iter()
                .rev()
                .enumerate()
                .for_each(|(_i, top_ch)| match *top_ch {
                    '(' => completion_stack.push(')'),
                    '[' => completion_stack.push(']'),
                    '{' => completion_stack.push('}'),
                    '<' => completion_stack.push('>'),
                    _ => {}
                });
        }

        completion_lines.push(completion_stack.to_vec());
    }

    // DEBUG
    // println!("[*] completion_lines: {} -------------", completion_lines.len());
    // for i in 0..completion_lines.len() {
    //     print!("[{i}]: ");
    //     for j in 0..completion_lines[i].len() {
    //         print!("{} ", completion_lines[i][j]);
    //     }
    //     println!();
    // }

    completion_lines
}

fn get_completion_scores(completion_lines: Vec<Vec<char>>) -> Vec<u64> {
    // let mut total_completion_score: u64 = 0;
    let mut completion_scores_vec = vec![];

    completion_lines.iter().enumerate().for_each(|(_i, line)| {
        let mut line_completion_score = 0;

        line.iter().enumerate().for_each(|(_j, ch)| match *ch {
            ')' => line_completion_score = (line_completion_score * 5) + 1,
            ']' => line_completion_score = (line_completion_score * 5) + 2,
            '}' => line_completion_score = (line_completion_score * 5) + 3,
            '>' => line_completion_score = (line_completion_score * 5) + 4,
            _ => {}
        });
        completion_scores_vec.push(line_completion_score);
        // total_completion_score += line_completion_score;
    });

    println!("[*] completion_lines with score:  -------------");
    for i in 0..completion_lines.len() {
        print!("[{i}]: ");
        for j in 0..completion_lines[i].len() {
            print!("{} ", completion_lines[i][j]);
        }
        println!(" - {}", completion_scores_vec[i]);
    }

    let mut sorted_completion_scores = completion_scores_vec.clone();
    sorted_completion_scores.sort_by(|a, b| b.cmp(a));

    sorted_completion_scores
}
