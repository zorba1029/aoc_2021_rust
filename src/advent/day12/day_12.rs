// advent/day_12.rs
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::advent::day12::path_matrix::{AllPathSmallTwice, DFSAllPath, PathMatrix};

///////////////////////////////////////////
//---   Day 12: Passage Pathing         ---
///////////////////////////////////////////

//-----------------------
// sample input
//-----------------------
// start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end
//------------
// a list of how all of the caves are connected. You start in the cave named start,
// and your destination is the cave named end.
// An entry like b-d means that cave b is connected to cave d - that is, you can move
// between them.
//
// So, the above cave system looks roughly like this:
//     start
//     /   \
// c--A-----b--d
//     \   /
//      end
//
// Your goal is to find the number of distinct paths that start at start, end at end,
// and don't visit small caves more than once. There are two types of caves:
// big cave (written in uppercase, like A) and small caves (written in lowercase, like b).
// It would be a waste of time to visit any small cave more than once, but big caves
// are large enough that it might be worth visiting them multiple times. So, all paths
// you find should visit small caves at most once, and can visit big cabes any number
// of times.
//
// Given these rules, there are 10 paths through this example cave system:
// start,A,b,A,c,A,end
// start,A,b,A,end
// start,A,b,end
// start,A,c,A,b,A,end
// start,A,c,A,b,end
// start,A,c,A,end
// start,A,end
// start,b,A,c,A,end
// start,b,A,end
// start,b,end
//
// (Each line in the above list corresponds to a single path; the caves visited by that path
// are listed in the order they are visited and seprated by commas,)
// Note that, in the cave system, cave d is never visited by any path: to do so,
// cave b would need to be visited twice (once on the way to cave d and a second time
// when returning from cave d), and since cave b is small, this is not allowed.
// Here is a slightly larger example:
// dc-end
// HN-start
// start-kj
// dc-start
// dc-HN
// LN-dc
// HN-end
// kj-sa
// kj-HN
// kj-dc
//
// The 19 paths through it are as follows:
// start,HN,dc,HN,end
// start,HN,dc,HN,kj,HN,end
// start,HN,dc,end
// start,HN,dc,kj,HN,end
// start,HN,end
// start,HN,kj,HN,dc,HN,end
// start,HN,kj,HN,dc,end
// start,HN,kj,HN,end
// start,HN,kj,dc,HN,end
// start,HN,kj,dc,end
// start,dc,HN,end
// start,dc,HN,kj,HN,end
// start,dc,end
// start,dc,kj,HN,end
// start,kj,HN,dc,HN,end
// start,kj,HN,dc,end
// start,kj,HN,end
// start,kj,dc,HN,end
// start,kj,dc,end
//
// Finally, this even larger example has 226 paths through it:
// fs-end
// he-DX
// fs-he
// start-DX
// pj-DX
// end-zg
// zg-sl
// zg-pj
// pj-he
// RW-he
// fs-DX
// pj-RW
// zg-RW
// start-pj
// he-WI
// zg-he
// pj-fs
// start-RW
//

///////////////////////////////////////////////////////////////////////////////////

pub fn do_day_12() {
    day_12_part_one();
    day_12_part_two();
}

pub fn day_12_part_one() {
    info!("===============================================");
    info!("---Day 12: Passage Pathing, Part One ---, 2/1/2022 (Feb, 1) ==> DONE");
    info!("===============================================");
    // let filename = "input/day_12-sample-a.txt";
    // let filename = "input/day_12-sample-b.txt";
    // let filename = "input/day_12-sample-c.txt";
    let filename = "input/day_12-input.txt";
    let input_lines = handle_input_normalize(filename);
    info!("input_lines: {:?}", input_lines.len());
    display_input_lines(&input_lines);

    let name_to_index = node_name_to_index(&input_lines);
    display_node_name_index(&name_to_index);

    let node_size = name_to_index.len();
    let mut path_graph = PathMatrix::new(node_size);
    path_graph.init_input_data(&input_lines, &name_to_index);

    let mut path_count: u32 = 0;
    path_graph.start_dfs_all(&name_to_index, &mut path_count);

    info!("-----------------------------------------");
    info!("---Day 12: Passage Pathing, Part One --- ");
    info!("Input File: {}", filename);
    // info!("path_graph: {:?}", path_graph);
    info!("------------------------------------------");
    info!("[*] all_path_count: {}", path_count);
    info!("-----------------------------------------");
    path_graph.display_matrix();
}

//-------------------------------------------
// .....
// Path( 4754):  -> start -> IY -> ij -> UH -> end
//  INFO  advent_code_2021::advent::day_12 >
// -----------------------------------------
// ---Day 12: Passage Pathing, Part One ---
// Input File: input/day_12-input.txt
// ------------------------------------------
// [*] all_path_count: 4754   <<-- Right Answer
// -----------------------------------------
// ---- maps: ----------------
//   0: start [ 0 0 0 1 1 0 1 0 0 0 0 0 0 ]
//   1: vp    [ 0 0 1 0 0 0 0 1 1 0 0 0 1 ]
//   2: BY    [ 0 1 0 0 0 0 0 0 0 0 0 0 0 ]
//   3: ui    [ 1 0 0 0 1 0 1 1 0 0 1 1 0 ]
//   4: oo    [ 1 0 0 1 0 1 1 0 0 0 0 0 0 ]
//   5: kk    [ 0 0 0 0 1 0 1 1 1 0 0 1 0 ]
//   6: IY    [ 1 0 0 1 1 1 0 1 0 0 0 0 0 ]
//   7: ij    [ 0 1 0 1 0 1 1 0 1 0 0 1 0 ]
//   8: SP    [ 0 1 0 0 0 1 0 1 0 0 0 0 1 ]
//   9: kg    [ 0 0 0 0 0 0 0 0 0 0 1 0 0 ]
//  10: uj    [ 0 0 0 1 0 0 0 0 0 1 0 1 0 ]
//  11: UH    [ 0 0 0 1 0 1 0 1 0 0 1 0 1 ]
//  12: end   [ 0 1 0 0 0 0 0 0 1 0 0 1 0 ]
// ---- visited: ----------------
//   [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// ---- index_table: ----------------
//   {7: "ij", 2: "BY", 1: "vp", 12: "end", 6: "IY", 11: "UH", 9: "kg",
//   10: "uj", 0: "start", 8: "SP", 5: "kk", 3: "ui", 4: "oo"}
// ---- node_type: ----------------
//   {"BY": Big, "kg": Small, "ij": Small, "ui": Small, "IY": Big,
//    "SP": Big, "vp": Small, "uj": Small, "UH": Big, "end": Small,
//    "oo": Small, "start": Small, "kk": Small}

// fn test_dfs_search() {
//     info!("===============================================");
//     info!("--- tiral: dfs_search");
//     let mut path_graph = PathMatrix::new(6);
//     info!("{:?}", path_graph);
//     path_graph.put_input_data(1, 2);
//     path_graph.put_input_data(1, 3);
//     path_graph.put_input_data(2, 3);
//     path_graph.put_input_data(2, 5);
//     path_graph.put_input_data(3, 4);
//     path_graph.put_input_data(3, 5);
//     path_graph.put_input_data(4, 5);
//
//     // path_graph.dfs(1);
//     info!("------------------------------------------");
//     info!("{:?}", path_graph);
//
//     info!("===============================================");
//     info!("--- trial: dfs_all   ");
//     let mut path_count = 0;
//     path_graph.dfs_all(1, 4, &mut path_count);
//     info!("------------------------------------------");
//     info!("{:?}", path_graph);
// }

// --- Part Two ---
// After reviewing the available paths, you realize you might have time to visit a single small cave twice.
// Specifically, big caves can be visited any number of times,
// a single small cave can be visited at most twice,
// and the remaining small caves can be visited at most once.
// However, the caves named start and end can only be visited exactly once each:
// once you leave the start cave, you may not return to it,
// and once you reach the end cave, the path must end immediately.

fn day_12_part_two() {
    info!("===============================================");
    info!("---Day 12: Passage Pathing, Part Two ---, Feb/3/2022 (Feb, 3) ==> DONE");
    info!("===============================================");
    // let filename = "input/day_12-sample-a.txt";
    // let filename = "input/day_12-sample-b.txt";
    // let filename = "input/day_12-sample-c.txt";
    let filename = "input/day_12-input.txt";
    let input_lines = handle_input_normalize(filename);
    info!("input_lines: {:?}", input_lines.len());
    display_input_lines(&input_lines);

    let name_to_index = node_name_to_index(&input_lines);
    display_node_name_index(&name_to_index);

    let node_size = name_to_index.len();
    let mut path_graph = PathMatrix::new(node_size);
    path_graph.init_input_data(&input_lines, &name_to_index);

    let mut path_count: u32 = 0;
    path_graph.start_dfs_all_small_twice(&name_to_index, &mut path_count);

    info!("-----------------------------------------");
    info!("--- Day 12: Passage Pathing, Part Two -- ");
    info!("Input File: {}", filename);
    // info!("path_graph: {:?}", path_graph);
    info!("------------------------------------------");
    info!("[*] all_path_count (small node two times): {}", path_count);
    info!("-----------------------------------------");
    path_graph.display_matrix();
    info!("-----------------------------------------");
}

// -----------------------------------------
// --- Day 12: Passage Pathing, Part Two --
// Input File: input/day_12-input.txt
// ------------------------------------------
// [*] all_path_count (small node two times): 143562
// -----------------------------------------

// ....
// Path(143562):  -> start -> IY -> ij -> UH -> end
// -----------------------------------------
// --- Day 12: Passage Pathing, Part Two --
// Input File: input/day_12-input.txt
// ------------------------------------------
// [*] all_path_count (small node two times): 143562
// -----------------------------------------
// ---- maps: ----------------
//   0: start [ 0 0 0 1 1 0 1 0 0 0 0 0 0 ]
//   1: vp    [ 0 0 1 0 0 0 0 1 1 0 0 0 1 ]
//   2: BY    [ 0 1 0 0 0 0 0 0 0 0 0 0 0 ]
//   3: ui    [ 1 0 0 0 1 0 1 1 0 0 1 1 0 ]
//   4: oo    [ 1 0 0 1 0 1 1 0 0 0 0 0 0 ]
//   5: kk    [ 0 0 0 0 1 0 1 1 1 0 0 1 0 ]
//   6: IY    [ 1 0 0 1 1 1 0 1 0 0 0 0 0 ]
//   7: ij    [ 0 1 0 1 0 1 1 0 1 0 0 1 0 ]
//   8: SP    [ 0 1 0 0 0 1 0 1 0 0 0 0 1 ]
//   9: kg    [ 0 0 0 0 0 0 0 0 0 0 1 0 0 ]
//  10: uj    [ 0 0 0 1 0 0 0 0 0 1 0 1 0 ]
//  11: UH    [ 0 0 0 1 0 1 0 1 0 0 1 0 1 ]
//  12: end   [ 0 1 0 0 0 0 0 0 1 0 0 1 0 ]
// ---- visited: ----------------
//   [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// ---- index_table: ----------------
//   {7: "ij", 4: "oo", 0: "start", 9: "kg", 5: "kk", 11: "UH",
//    6: "IY", 12: "end", 2: "BY", 8: "SP", 1: "vp", 10: "uj", 3: "ui"}
// ---- node_type: ----------------
//   {"end": Small, "oo": Small, "SP": Big, "vp": Small, "kk": Small,
//     "kg": Small, "UH": Big, "IY": Big, "start": Small, "uj": Small,
//     "BY": Big, "ui": Small, "ij": Small}
// -----------------------------------------

fn handle_input_normalize(filename: &str) -> Vec<(String, String)> {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let lines_count = lines.len();
    info!("[*] Input Filename: {}", filename);
    info!("[*] input lines count = {}", lines_count);

    let first_line = &lines[0];
    info!("[ ] First Line: len={}, {}, ", first_line.len(), first_line);

    let input_lines = lines
        .iter()
        .map(|line| line.split('-').collect::<Vec<_>>())
        .map(|items| {
            let from = items[0].trim().to_owned();
            let to = items[1].trim().to_owned();
            info!("line []: {} --> {}", from, to);
            if to == "start" || from == "end" {
                (to, from)
            } else {
                (from, to)
            }
        })
        .collect::<Vec<(String, String)>>();

    input_lines
}

fn display_input_lines(input_lines: &Vec<(String, String)>) {
    //-- chunks_table vector of vector
    input_lines.iter().enumerate().for_each(|(i, (from, to))| {
        info!("    line[{i}]: {from}, {to}");
    });
}

fn node_name_to_index(input_lines: &Vec<(String, String)>) -> HashMap<String, i16> {
    let mut name_to_index = HashMap::<String, i16>::new();

    let mut index = 0;
    name_to_index.insert("start".to_owned(), index);
    index += 1;
    input_lines.iter().enumerate().for_each(|(i, (from, to))| {
        info!("    line [{}]: {}, {}", i, from, to);
        if from == "start" || to == "start" {
            return;
        }
        if from == "end" || to == "end" {
            return;
        }
        if name_to_index.contains_key(from) != true {
            name_to_index.insert(from.to_owned(), index);
            index += 1;
        }
        if name_to_index.contains_key(to) != true {
            name_to_index.insert(to.to_owned(), index);
            index += 1;
        }
    });
    name_to_index.insert("end".to_owned(), index);

    name_to_index
}

fn display_node_name_index(name_to_index: &HashMap<String, i16>) {
    //-- chunks_table vector of vector
    name_to_index
        .iter()
        .enumerate()
        .for_each(|(_i, (key, value))| {
            info!("    map[{}] -> {}", key, value);
        });
}
