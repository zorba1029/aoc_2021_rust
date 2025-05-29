// advent/day_12_2nd.rs
use lazy_static::lazy_static;
use log::{info, warn};
use regex::Regex;
use std::collections::HashMap;
// use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
// How many paths through this cave system are there that visit small caves at most once?
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum NodeType {
    Big,
    Small,
}

lazy_static! {
    static ref RE_LOWERCASE: Regex = Regex::new(r"^[a-z]*$").unwrap();
    // static ref RE_UPPERCASE: Regex = Regex::new(r"^[A-Z]*$").unwrap();
}

fn check_lowercase_string(node_name: &str) -> NodeType {
    if RE_LOWERCASE.is_match(node_name) {
        NodeType::Small
    } else {
        NodeType::Big
    }
}

#[derive(Debug)]
struct PathMatrix {
    maps: Vec<Vec<i8>>,
    visited: Vec<u32>,
    stack: Vec<u32>,
    // queue: VecDeque<i16>,
    index_to_name: HashMap<i16, String>,
    node_type: HashMap<String, NodeType>,
    small_visited: Vec<u32>,
}

impl PathMatrix {
    pub fn new(node_size: usize) -> Self {
        let mut maps: Vec<Vec<i8>> = Vec::new();
        let mut visited: Vec<u32> = Vec::new();
        let stack: Vec<u32> = Vec::new();
        // let queue: VecDeque<i16> = VecDeque::new();
        let index_to_name: HashMap<i16, String> = HashMap::new();
        let node_type: HashMap<String, NodeType> = HashMap::new();
        let mut small_visited: Vec<u32> = Vec::new();

        for _i in 0..node_size {
            let line = vec![0; node_size];
            maps.push(line);
            visited.push(0);
            small_visited.push(0);
        }

        Self {
            maps,
            visited,
            stack,
            // queue,
            index_to_name,
            node_type,
            small_visited,
        }
    }

    fn put_input_data(&mut self, i: usize, j: usize) {
        self.maps[i][j] = 1;
        self.maps[j][i] = 1;
    }

    fn init_input_data(
        &mut self,
        input_lines: &Vec<(String, String)>,
        name_to_index: &HashMap<String, i16>,
    ) {
        //-- 1) adjacency matrix
        input_lines.iter().enumerate().for_each(|(i, (from, to))| {
            info!(
                "    input [{}]: {} <-> {} ({} <-> {})",
                i, from, to, name_to_index[from], name_to_index[to]
            );
            self.put_input_data(name_to_index[from] as usize, name_to_index[to] as usize);
        });

        //-- 2) node_type setup
        input_lines.iter().enumerate().for_each(|(_i, (from, to))| {
            if self.node_type.contains_key(from) != true {
                if check_lowercase_string(from) == NodeType::Small {
                    self.node_type.insert(from.to_owned(), NodeType::Small);
                } else {
                    self.node_type.insert(from.to_owned(), NodeType::Big);
                }
            }

            if self.node_type.contains_key(to) != true {
                if check_lowercase_string(to) == NodeType::Small {
                    self.node_type.insert(to.to_owned(), NodeType::Small);
                } else {
                    self.node_type.insert(to.to_owned(), NodeType::Big);
                }
            }

            // info!("    input [{}]: {} -> {} ({} -> {})", i, from, to, name_to_index[from], name_to_index[to]);
        });

        //-- 3) (key, index) --> (index, key) mapping table
        name_to_index.iter().for_each(|(key, value)| {
            self.index_to_name.insert(*value, key.to_owned());
        })
    }

    fn display_matrix(&self) {
        info!("---- maps: ----------------");
        for (r, line) in self.maps.iter().enumerate() {
            let mut format_str = format!("{:2}: {:5} [ ", r, self.index_to_name[&(r as i16)]);
            for (_c, v) in line.iter().enumerate() {
                format_str += &*format!("{} ", v);
            }
            format_str += &*format!("]");
            info!(" {} ", format_str);
        }

        info!("---- visited: ----------------");
        let format_str = format!(" {:?}", self.visited);
        info!(" {} ", format_str);

        info!("---- index_to_name: ----------------");
        let format_str = format!(" {:?}", self.index_to_name);
        info!(" {} ", format_str);

        info!("---- node_type: ----------------");
        let format_str = format!(" {:?}", self.node_type);
        info!(" {} ", format_str);
    }
}

// trait DFSAllPath {
//     fn dfs_all(&mut self, src: usize, dest: usize, path_count: &mut i16) -> i16;
// }
//
// impl DFSAllPath for PathMatrix {
//     fn dfs_all(&mut self, src: usize, dest: usize, path_count: &mut i16) -> i16 {
//         let path_count = path_count;
//         // self.visited[src] += 1;
//         // self.stack.push(src as i16);
//
//         if src == dest {
//             let mut format_str = format!(" ");
//             let stack_len = self.stack.len();
//             for i in 0..stack_len {
//                 // info!("-> {} ", self.stack[i]);
//                 // format_str += &*format!("-> {} ", self.stack[i]);
//                 let s_index = self.stack[i];
//                 format_str += &*format!("-> {} ", self.index_to_name[&s_index]);
//             }
//             // info!("--Complete--");
//             *path_count += 1;
//             info!("Path({:5}): {} ", path_count, format_str);
//             // self.stack.pop();
//             return *path_count;
//         }
//
//         for i in 1..self.maps.len() {
//             //-- original
//             // if self.maps[src][i] == true && self.visited[i] != true {
//             //     self.dfs_all(i, dest);
//             //     self.visited[i] = false;
//             // }
//             let node_name = self.index_to_name.get(&(i as i16)).unwrap();
//             match self.node_type.get(node_name) {
//                 Some(NodeType::Small) => {
//                     if self.maps[src][i] == 1 && self.visited[i] != 1 {
//                         // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
//                         self.visited[i] = 1;
//                         self.stack.push(i as i16);
//                         self.dfs_all(i, dest, path_count);
//                         self.visited[i] = 0;
//                         self.stack.pop();
//                     }
//                 },
//                 Some(NodeType::Big) => {
//                     if self.maps[src][i] == 1 && self.visited[i] < self.maps.len() as i16 {
//                         // info!("   node-name: {} ({:?}) ", node_name, NodeType::Big);
//                         self.visited[i] += 1;
//                         self.stack.push(i as i16);
//                         self.dfs_all(i, dest, path_count);
//                         self.visited[i] -= 1;
//                         self.stack.pop();
//                     }
//                 }
//                 _ => {
//                     warn!("   node-name: {} (Unknown) ", node_name);
//                 }
//             }
//         }
//
//         // self.stack.pop();
//
//         *path_count
//     }
// }

#[allow(dead_code)]
trait DFSAllPath {
    fn start_dfs_all(&mut self, name_to_index: &HashMap<String, i16>, path_count: &mut u32) -> u32;
    fn dfs_all(&mut self, src: usize, dest: usize, path_count: &mut u32) -> u32;
}

impl DFSAllPath for PathMatrix {
    fn start_dfs_all(&mut self, name_to_index: &HashMap<String, i16>, path_count: &mut u32) -> u32 {
        self.visited[name_to_index["start"] as usize] += 1;
        self.stack.push(name_to_index["start"] as u32);
        self.dfs_all(
            name_to_index["start"] as usize,
            name_to_index["end"] as usize,
            path_count,
        )
    }

    fn dfs_all(&mut self, src: usize, dest: usize, path_count: &mut u32) -> u32 {
        let path_count = path_count;

        if src == dest {
            let mut format_str = format!(" ");
            for i in 0..self.stack.len() {
                let s_index = self.stack[i];
                format_str += &*format!("-> {} ", self.index_to_name[&(s_index as i16)]);
            }
            *path_count += 1;
            info!("Path({:5}): {} ", path_count, format_str);
            return *path_count;
        }

        for i in 1..self.maps.len() {
            //-- original
            // if self.maps[src][i] == true && self.visited[i] != true {
            //     self.dfs_all(i, dest);
            //     self.visited[i] = false;
            // }
            let node_name = self.index_to_name.get(&(i as i16)).unwrap();
            match self.node_type.get(node_name) {
                Some(NodeType::Small) => {
                    if self.maps[src][i] == 1 && self.visited[i] < 1 {
                        // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
                        self.visited[i] = 1;
                        self.stack.push(i as u32);
                        self.dfs_all(i, dest, path_count);
                        self.visited[i] = 0;
                        self.stack.pop();
                    }
                }
                Some(NodeType::Big) => {
                    if self.maps[src][i] == 1 && self.visited[i] < self.maps.len() as u32 {
                        // info!("   node-name: {} ({:?}) ", node_name, NodeType::Big);
                        self.visited[i] += 1;
                        self.stack.push(i as u32);
                        self.dfs_all(i, dest, path_count);
                        self.visited[i] -= 1;
                        self.stack.pop();
                    }
                }
                _ => {
                    warn!("   node-name: {} (Unknown) ", node_name);
                }
            }
        }

        *path_count
    }
}

trait AllPathSmallTwice {
    fn start_dfs_all_small_twice(
        &mut self,
        name_to_index: &HashMap<String, i16>,
        path_count: &mut u32,
    ) -> u32;
    fn dfs_all_small_twice(&mut self, src: usize, dest: usize, path_count: &mut u32, call_type: u8);
}

impl AllPathSmallTwice for PathMatrix {
    fn start_dfs_all_small_twice(
        &mut self,
        name_to_index: &HashMap<String, i16>,
        path_count: &mut u32,
    ) -> u32 {
        self.dfs_all_small_twice(
            name_to_index["start"] as usize,
            name_to_index["end"] as usize,
            path_count,
            0,
        );
        *path_count
    }

    fn dfs_all_small_twice(
        &mut self,
        src: usize,
        dest: usize,
        path_count: &mut u32,
        call_type: u8,
    ) {
        let path_count = path_count;

        // info!("  -- src[{}], stack: {:?}", src, self.stack);

        match call_type {
            0 => {
                self.visited[src] = 1;
                self.stack.push(src as u32);
            }
            1 => {
                self.small_visited[src] += 1;
                self.visited[src] = 1;
                self.stack.push(src as u32);
            }
            2 => {
                self.small_visited[src] += 1;
                self.visited[src] += 1;
                self.stack.push(src as u32);
            }
            3 => {
                self.visited[src] += 1;
                self.stack.push(src as u32);
            }
            _ => {}
        }

        if src == dest {
            // let mut format_str = format!("");
            // let stack_len = self.stack.len();
            // for i in 0..stack_len-1 {
            //     let s_index = self.stack[i];
            //     format_str += &*format!("{}:{} -> ", self.stack[i], self.index_to_name[&(s_index as i16)]);
            // }
            // let s_index = self.stack[stack_len-1];
            // format_str += &*format!("{}:{} ",  self.stack[stack_len-1], self.index_to_name[&(s_index as i16)],);
            // info!("Path({:5}): {} ", path_count, format_str);
            // info!("    [{:5}]: {:?} ",  path_count, self.stack);

            *path_count += 1;
            // return *path_count;
            match call_type {
                0 => {
                    self.visited[src] = 0;
                    self.stack.pop();
                }
                1 => {
                    self.small_visited[src] -= 1;
                    self.visited[src] = 0;
                    self.stack.pop();
                }
                2 => {
                    self.small_visited[src] -= 1;
                    self.visited[src] -= 1;
                    self.stack.pop();
                }
                3 => {
                    self.visited[src] -= 1;
                    self.stack.pop();
                }
                _ => {}
            }
            return;
        }

        for i in 1..self.maps.len() {
            //-- original
            // if self.maps[src][i] == true && self.visited[i] != true {
            //     self.dfs_all(i, dest);
            //     self.visited[i] = false;
            // }
            //-- i = 0 means "start" node, so we do not need to check i = 0.
            let node_name = self.index_to_name.get(&(i as i16)).unwrap();
            // info!("   [{}] ({:?}) ", i, node_name);
            match self.node_type.get(node_name) {
                Some(NodeType::Small) => {
                    match &node_name[..] {
                        "start" | "end" => {
                            if self.maps[src][i] == 1 && self.visited[i] < 1 {
                                // info!("   start|end, node-name: {} ({:?}) ", node_name, NodeType::Small);
                                self.dfs_all_small_twice(i, dest, path_count, 0);
                            }
                        }
                        _ => {
                            if self.small_visited.contains(&2) {
                                if self.maps[src][i] == 1 && self.visited[i] < 1 {
                                    // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
                                    self.dfs_all_small_twice(i, dest, path_count, 1);
                                }
                            } else {
                                if self.maps[src][i] == 1 && self.visited[i] < 2 {
                                    // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
                                    self.dfs_all_small_twice(i, dest, path_count, 2);
                                }
                            }
                        }
                    }
                }
                Some(NodeType::Big) => {
                    if self.maps[src][i] == 1 && self.visited[i] < self.maps.len() as u32 {
                        // info!("   node-name: {} ({:?}) ", node_name, NodeType::Big);
                        self.dfs_all_small_twice(i, dest, path_count, 3);
                    }
                }
                _ => {
                    warn!("   node-name: {} (Unknown) ", node_name);
                }
            }
        }

        match call_type {
            0 => {
                self.visited[src] = 0;
                self.stack.pop();
            }
            1 => {
                self.small_visited[src] -= 1;
                self.visited[src] = 0;
                self.stack.pop();
            }
            2 => {
                self.small_visited[src] -= 1;
                self.visited[src] -= 1;
                self.stack.pop();
            }
            3 => {
                self.visited[src] -= 1;
                self.stack.pop();
            }
            _ => {}
        }
        // *path_count
    }
}

///////////////////////////////////////////////////////////////////////////////////

pub fn do_day_12() {
    // day_12_2nd_part_one();
    day_12_2nd_part_two();
}

// fn day_12_2nd_part_one() {
//     info!("===============================================");
//     info!("---Day 12 [2nd]: Passage Pathing, Part One ---, 2/1/2022 (Feb, 1) ==> DONE");
//     info!("===============================================");
//     let filename = "input/day_12-sample-a.txt";
//     // let filename = "input/day_12-sample-b.txt";
//     // let filename = "input/day_12-sample-c.txt";
//     // let filename = "input/day_12-input.txt";
//     let input_lines = handle_input_normalize(filename);
//     info!("input_lines: {:?}", input_lines.len());
//     display_input_lines(&input_lines);
//
//     let name_to_index = node_name_to_index(&input_lines);
//     display_node_name_index(&name_to_index);
//
//     let node_size = name_to_index.len();
//     let mut path_graph = PathMatrix::new(node_size);
//     path_graph.init_input_data(&input_lines, &name_to_index);
//
//     let mut path_count = 0;
//     path_graph.start_dfs_all(&name_to_index, &mut path_count);
//
//     info!("-----------------------------------------");
//     info!("---Day 12 [2nd]: Passage Pathing, Part One --- ");
//     info!("Input File: {}", filename);
//     // info!("path_graph: {:?}", path_graph);
//     info!("------------------------------------------");
//     info!("[*] all_path_count: {}", path_count);
//     info!("-----------------------------------------");
//     path_graph.display_matrix();
// }

// --- Part Two ---
// After reviewing the available paths, you realize you might have time to visit a single small cave twice.
// Specifically, big caves can be visited any number of times,
// a single small cave can be visited at most twice,
// and the remaining small caves can be visited at most once.
// However, the caves named start and end can only be visited exactly once each:
// once you leave the start cave, you may not return to it,
// and once you reach the end cave, the path must end immediately.

fn day_12_2nd_part_two() {
    info!("===============================================");
    info!("---Day 12 [2nd]: Passage Pathing, Part Two ---, Feb/3/2022 (Feb,3) ==> DONE");
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
    // path_graph.all_path_small_twice_iterative_start(&name_to_index, &mut path_count);

    info!("-----------------------------------------");
    info!("--- Day 12 [2nd]: Passage Pathing, Part Two -- ");
    info!("Input File: {}", filename);
    // info!("path_graph: {:?}", path_graph);
    info!("------------------------------------------");
    info!("[*] all_path_count (small node two times): {}", path_count);
    info!("-----------------------------------------");
    path_graph.display_matrix();
    info!("stack: {:?}", path_graph.stack);
    info!("-----------------------------------------");
}
// ------------------------------------------
// Input File: input/day_12-input.txt
// [*] all_path_count (small node two times): 143562  <<-- Right Answer
// -----------------------------------------

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
//   [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// ---- index_to_name: ----------------
//   {12: "end", 6: "IY", 3: "ui", 7: "ij", 0: "start", 11: "UH", 9: "kg",
//     1: "vp", 10: "uj", 4: "oo", 8: "SP", 5: "kk", 2: "BY"}
// ---- node_type: ----------------
//   {"end": Small, "IY": Big, "ui": Small, "SP": Big, "uj": Small, "kk": Small,
//     "ij": Small, "vp": Small, "kg": Small, "BY": Big, "oo": Small, "start": Small, "UH": Big}
// stack: []
// ------------------------------------------

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
            if to == "start" {
                (to, from)
            } else if from == "end" {
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
