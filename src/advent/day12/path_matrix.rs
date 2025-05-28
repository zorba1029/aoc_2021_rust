// advent/day_12.rs
use std::collections::HashMap;
// use std::collections::VecDeque;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use regex::Regex;

///////////////////////////////////////////
//---   Day 12: Passage Pathing         ---
///////////////////////////////////////////

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
pub struct PathMatrix {
    maps: Vec<Vec<i8>>,
    visited: Vec<i16>,
    stack: Vec<i16>,
    // queue: VecDeque<i16>,
    index_table: HashMap<i16, String>,
    node_type: HashMap<String, NodeType>,
    small_visited: Vec<i16>,
}

impl PathMatrix {
    pub fn new(node_size: usize) -> Self {
        let mut maps: Vec<Vec<i8>> = Vec::new();
        let mut visited: Vec<i16> = Vec::new();
        let stack: Vec<i16> = Vec::new();
        // let queue: VecDeque<i16> = VecDeque::new();
        let index_table: HashMap<i16, String> = HashMap::new();
        let node_type: HashMap<String, NodeType> = HashMap::new();
        let mut small_visited: Vec<i16> = Vec::new();

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
            index_table,
            node_type,
            small_visited,
        }
    }

    pub fn put_input_data(&mut self, i: usize, j: usize) {
        self.maps[i][j] = 1;
        self.maps[j][i] = 1;
    }

    pub fn init_input_data(
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
            self.index_table.insert(*value, key.to_owned());
        })
    }

    pub fn display_matrix(&self) {
        info!("---- maps: ----------------");
        for (r, line) in self.maps.iter().enumerate() {
            let mut format_str = format!("{:2}: {:5} [ ", r, self.index_table[&(r as i16)]);
            for (_c, v) in line.iter().enumerate() {
                format_str += &*format!("{} ", v);
            }
            format_str += &*format!("]");
            info!(" {} ", format_str);
        }

        info!("---- visited: ----------------");
        let format_str = format!(" {:?}", self.visited);
        info!(" {} ", format_str);

        info!("---- index_table: ----------------");
        let format_str = format!(" {:?}", self.index_table);
        info!(" {} ", format_str);

        info!("---- node_type: ----------------");
        let format_str = format!(" {:?}", self.node_type);
        info!(" {} ", format_str);
    }
}

pub trait DFSAllPath {
    fn start_dfs_all(&mut self, name_to_index: &HashMap<String, i16>, path_count: &mut u32) -> u32;
    fn dfs_all(&mut self, src: usize, dest: usize, path_count: &mut u32) -> u32;
}

impl DFSAllPath for PathMatrix {
    fn start_dfs_all(&mut self, name_to_index: &HashMap<String, i16>, path_count: &mut u32) -> u32 {
        self.visited[name_to_index["start"] as usize] += 1;
        self.stack.push(name_to_index["start"] as i16);
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
                format_str += &*format!("-> {} ", self.index_table[&s_index]);
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
            let node_name = self.index_table.get(&(i as i16)).unwrap();
            match self.node_type.get(node_name) {
                Some(NodeType::Small) => {
                    if self.maps[src][i] == 1 && self.visited[i] < 1 {
                        // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
                        self.visited[i] = 1;
                        self.stack.push(i as i16);
                        self.dfs_all(i, dest, path_count);
                        self.visited[i] = 0;
                        self.stack.pop();
                    }
                }
                Some(NodeType::Big) => {
                    if self.maps[src][i] == 1 && self.visited[i] < self.maps.len() as i16 {
                        // info!("   node-name: {} ({:?}) ", node_name, NodeType::Big);
                        self.visited[i] += 1;
                        self.stack.push(i as i16);
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

pub trait AllPathSmallTwice {
    fn start_dfs_all_small_twice(
        &mut self,
        name_to_index: &HashMap<String, i16>,
        path_count: &mut u32,
    ) -> u32;
    fn dfs_all_small_twice(&mut self, src: usize, dest: usize, path_count: &mut u32) -> u32;
}

impl AllPathSmallTwice for PathMatrix {
    fn start_dfs_all_small_twice(
        &mut self,
        name_to_index: &HashMap<String, i16>,
        path_count: &mut u32,
    ) -> u32 {
        self.visited[name_to_index["start"] as usize] += 1;
        self.stack.push(name_to_index["start"] as i16);
        self.dfs_all_small_twice(
            name_to_index["start"] as usize,
            name_to_index["end"] as usize,
            path_count,
        )
    }

    fn dfs_all_small_twice(&mut self, src: usize, dest: usize, path_count: &mut u32) -> u32 {
        debug!("=== dfs_all_small_twice : path_count={:?} ===", &path_count);
        let path_count = path_count;

        if src == dest {
            // let mut format_str = format!(" ");
            // let stack_len = self.stack.len();
            // for i in 0..stack_len {
            //     let s_index = self.stack[i];
            //     format_str += &*format!("-> {} ", self.index_table[&s_index]);
            // }
            // info!("Path({:5}): {} ", path_count, format_str);
            *path_count += 1;
            return *path_count;
        }

        for i in 1..self.maps.len() {
            //-- original
            // if self.maps[src][i] == true && self.visited[i] != true {
            //     self.dfs_all(i, dest);
            //     self.visited[i] = false;
            // }
            let node_name = self.index_table.get(&(i as i16)).unwrap();
            match self.node_type.get(node_name) {
                Some(NodeType::Small) => {
                    match &node_name[..] {
                        "start" | "end" => {
                            if self.maps[src][i] == 1 && self.visited[i] < 1 {
                                // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
                                self.visited[i] = 1;
                                self.stack.push(i as i16);
                                self.dfs_all_small_twice(i, dest, path_count);
                                self.visited[i] = 0;
                                self.stack.pop();
                            }
                        }
                        _ => {
                            if self.small_visited.contains(&2) {
                                if self.maps[src][i] == 1 && self.visited[i] < 1 {
                                    // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
                                    self.small_visited[i] += 1;
                                    self.visited[i] = 1;
                                    self.stack.push(i as i16);
                                    self.dfs_all_small_twice(i, dest, path_count);
                                    self.small_visited[i] -= 1;
                                    self.visited[i] = 0;
                                    self.stack.pop();
                                }
                            } else if self.maps[src][i] == 1 && self.visited[i] < 2 {
                                // info!("   node-name: {} ({:?}) ", node_name, NodeType::Small);
                                self.small_visited[i] += 1;
                                self.visited[i] += 1;
                                self.stack.push(i as i16);
                                self.dfs_all_small_twice(i, dest, path_count);
                                self.small_visited[i] -= 1;
                                self.visited[i] -= 1;
                                self.stack.pop();
                            }
                        }
                    }
                }
                Some(NodeType::Big) => {
                    if self.maps[src][i] == 1 && self.visited[i] < self.maps.len() as i16 {
                        // info!("   node-name: {} ({:?}) ", node_name, NodeType::Big);
                        self.visited[i] += 1;
                        self.stack.push(i as i16);
                        self.dfs_all_small_twice(i, dest, path_count);
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
