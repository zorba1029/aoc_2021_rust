use core::str;
use log::{debug, info};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn handle_input(filename: &str) -> Vec<Vec<usize>> {
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
    info!("[ ] First Line: len={}, [{}]", first_line.len(), first_line);
    info!("[row, column] = [{}, {}]", lines_count, first_line.len());

    let input_lines = lines
        .iter()
        .map(|line| {
            let risk_levels = line
                .chars()
                // .map(|c| c.to_string().parse::<i32>().unwrap())
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            // debug!("{:?}", risk_levels);
            risk_levels
        })
        .collect::<Vec<Vec<_>>>();

    input_lines
}

#[allow(dead_code)]
fn display_map_data(data_map: &HashMap<(i32, i32), usize>, msg: &str, len: i32) {
    debug!("---- 🍏🍒 {} 🍏🍒----", msg);
    for row in 0..len {
        let mut format_str = format!("{:3}] ", row);
        for col in 0..len {
            let v = *data_map.get(&(row, col)).unwrap();
            if col % 100 == 99 {
                format_str += &*format!("{:1} | ", v);
            } else if col % 10 == 9 {
                format_str += &*format!("{:1} ", v);
            } else {
                format_str += &*format!("{:1}", v);
            }
        }
        debug!("{}", format_str);
    }
}

//--------------------------------------------------------------------------------------------------
// *** Found DestNode = (499, 499)
//  PREV[target=(499, 499)] = Some((498, 499)), DIST[target=(499, 499)] = Some(2948)
//  Total Loop Count = [249999]
//  Shortest Path Value  = 2948
//---------------------------------

pub fn day_15_part_two() {
    info!("===============================================");
    info!("--- Day 15: Chiton, Part Two ---, ");
    info!("===============================================");
    const TILE_COUNT: i32 = 5;
    // let filename = "input/day_15-sample.txt";
    let filename = "input/day_15-input.txt";
    let input_lines = handle_input(filename);
    let width = input_lines[0].len();
    let height = input_lines.len();
    info!("input_lines: rows = {:?}, columns = {}", width, height);

    let (mut dist, mut prev, mut queue) = make_init_data(&input_lines, TILE_COUNT);
    // display_map_data(&queue, "Input Queue Table", TILE_COUNT*width as i32);

    let mut visited: HashMap<(i32, i32), usize> = HashMap::new();
    let shortest_value = dijkstra_search(
        &mut dist,
        &mut prev,
        &mut queue,
        &mut visited,
        &input_lines,
        TILE_COUNT,
    );
    // display_shortest_path(&prev, &dist, &visited, &(TILE_COUNT * width as i32 -1, TILE_COUNT * height as i32 -1));

    info!("shortest_value  = {}", shortest_value);
}

type InitDataType = (
    HashMap<(i32, i32), usize>,
    HashMap<(i32, i32), Option<(i32, i32)>>,
    HashMap<(i32, i32), usize>,
);

fn make_init_data(input_lines: &[Vec<usize>], tile_count: i32) -> InitDataType {
    let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
    let mut prev: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut queue: HashMap<(i32, i32), usize> = HashMap::new();
    let row_len = input_lines.len() as i32;
    let col_len = input_lines[0].len() as i32;

    input_lines.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, value)| {
            let (r, c) = (row as i32, col as i32);
            dist.insert((r, c), usize::MAX);
            prev.insert((r, c), None);
            queue.insert((r, c), *value);
        });
    });

    for tile_i in 0..tile_count {
        for tile_j in 0..tile_count {
            if (tile_i, tile_j) == (0, 0) {
                continue;
            }
            if tile_j != 0 {
                // same row-position, but different column-position
                debug!("=============== A: COPY INPUT DATA TO RIGHT TILE -- tile = [{tile_i},{tile_j}] ===============");
                (0..row_len).for_each(|row| {
                    (0..col_len).for_each(|col| {
                        let (new_row, new_col) = (tile_i*row_len + row, tile_j*col_len + col);
                        let prev_row = new_row;
                        let prev_col = new_col - col_len;

                        dist.insert((new_row, new_col), usize::MAX);
                        prev.insert((new_row, new_col), None);

                        let prev_value = *queue.get(&(prev_row, prev_col)).unwrap();
                        let mut new_value = 1 + prev_value;
                        if new_value > 9 {
                            new_value = 1;
                        }
                        queue.insert((new_row, new_col),  new_value);
                        if new_row % 100 == 0 && new_col % 100 == 1 {
                            debug!("--  Prev[{prev_row},{prev_col}] -> New[{new_row},{new_col}] || = {prev_value} -> {new_value}");
                        }
                    });
                });
            } else {
                // same column-position, but difference row-position
                debug!("=============== B: COPY INUPT DATA TO DOWN TILE -- tile = [{tile_i},{tile_j}] ===============");
                (0..row_len).for_each(|row| {
                    (0..col_len).for_each(|col| {
                        let (new_row, new_col) = (tile_i*row_len + row, tile_j*col_len + col);
                        let prev_row = new_row - row_len;
                        let prev_col = new_col;

                        dist.insert((new_row, new_col), usize::MAX);
                        prev.insert((new_row, new_col), None);

                        let prev_value = *queue.get(&(prev_row, prev_col)).unwrap();
                        let mut new_value = 1 + prev_value;
                        if new_value > 9 {
                            new_value = 1;
                        }
                        queue.insert((new_row, new_col),  new_value);
                        if new_row % 100 == 0 && new_col % 100 == 1 {
                            debug!("--  Prev[{prev_row},{prev_col}] -> New[{new_row},{new_col}] || {prev_value} -> {new_value}");
                        }
                    });
                });
            }
        }
    }

    //-- dist(source=(0,0)) <-- 0 으로 초기화
    *dist.get_mut(&(0, 0)).unwrap() = 0;

    (dist, prev, queue)
}

// (Right, Down)
// const DELTAS: &[(i32,i32); 2] = &[(1,0), (0,1)];
// (Right, Down, Left, Up)
const NEIGHBORS: &[(i32, i32); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn dijkstra_search(
    dist: &mut HashMap<(i32, i32), usize>,
    prev: &mut HashMap<(i32, i32), Option<(i32, i32)>>,
    queue: &mut HashMap<(i32, i32), usize>,
    visited: &mut HashMap<(i32, i32), usize>,
    // input_lines: &Vec<Vec<usize>>,
    input_lines: &[Vec<usize>],
    tile_count: i32,
) -> usize {
    let width: i32 = tile_count * input_lines[0].len() as i32;
    let height: i32 = tile_count * input_lines.len() as i32;
    let dest_node = (width - 1, height - 1);
    let mut loop_count: i32 = 0;

    while !queue.is_empty() {
        let mut smallest_u = usize::MAX;
        let mut found_u: (i32, i32) = (0, 0);
        // find the smallest u in queue with minimum dist[u]
        // -- 매번 현재 queue (unvisited nodes)에서 최소거리 node를 찾는다.
        // ---> 이 부분이 시간을 많이 소비 하는 부분
        //    (-> priority queue로 개선 가능 함).
        for node in queue.keys() {
            let dist_u = *dist.get(node).unwrap();
            if dist_u < smallest_u {
                smallest_u = dist_u;
                found_u = *node;
            }
        }

        if loop_count % 1000 == 0 {
            debug!("Loop Count = [{}]", loop_count);
        }

        // remove u from Q
        let q = queue.remove(&found_u);
        visited.insert(found_u, q.unwrap());

        if found_u == dest_node {
            // (i.e.) found_u.0 == dest_node.0 && found_u.1 == dest_node.1
            //  queue가 empty이면 종료하게 되므로, 필요 없다.
            info!("*** Found DestNode = {:?}", found_u);
            break;
        }

        // info!("--- for each neighbor v of found_u ({:?}", found_u);
        // for each neighbor v of found_u node, where each v is still in queue (unvisited)
        for delta in NEIGHBORS {
            let v = (found_u.0 + delta.0, found_u.1 + delta.1);
            if queue.contains_key(&v) {
                // info!("v({:?}) = {:?}", v, queue.get(&v).unwrap());
                let alt = smallest_u + queue.get(&v).unwrap();
                // info!("alt : {:?}", alt);
                if alt < *dist.get(&v).unwrap() {
                    *dist.get_mut(&v).unwrap() = alt;
                    *prev.get_mut(&v).unwrap() = Some(found_u);
                }
            }
        }
        loop_count += 1;
    }

    info!(
        "PREV[target={:?}] = {:?}, DIST[target={:?}] = {:?}",
        &dest_node,
        prev.get(&dest_node).unwrap(),
        &dest_node,
        dist.get(&dest_node)
    );
    info!("Total Loop Count = [{}]", loop_count);
    info!("Shortest Path Value  = {}", *dist.get(&dest_node).unwrap());

    *dist.get(&dest_node).unwrap()
}

#[allow(dead_code)]
fn display_shortest_path(
    prev: &HashMap<(i32, i32), Option<(i32, i32)>>,
    dist: &HashMap<(i32, i32), usize>,
    visited: &HashMap<(i32, i32), usize>,
    dest_node: &(i32, i32),
) {
    let mut s_path: Vec<(i32, i32)> = Vec::new();
    let source_node: (i32, i32) = (0, 0);
    let mut u: (i32, i32) = *dest_node;

    if prev.get(&u).is_some() || u == source_node {
        while prev.get(&u).is_some() && u >= (0, 0) && u <= *dest_node {
            s_path.push(u);
            let option_u = *prev.get(&u).unwrap();
            if option_u.is_some() {
                u = option_u.unwrap();
            } else {
                break;
            }
        }
    }

    s_path.iter().for_each(|(row, col)| {
        debug!(
            "node[{},{}] = ({}), dist({},{}) = {:?}",
            row,
            col,
            visited.get(&(*row, *col)).unwrap(),
            row,
            col,
            dist.get(&(*row, *col))
        );
    });
}
