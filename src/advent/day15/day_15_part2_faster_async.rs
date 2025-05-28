use log::{debug, info};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn handle_input(filename: &str) -> Vec<Vec<usize>> {
    let content = std::fs::read_to_string(filename).expect("Couldn't read input");
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

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
    info!("--- Day 15: Chiton, Part Two --- Upgrade: used Priority Queue (Oct. 4, 2024)");
    info!("===============================================");
    const TILE_COUNT: i32 = 5;
    // let filename = "input/day_15-sample.txt";
    // let filename = "input/day_15-input-5x.txt";
    let filename = "input/day_15-input.txt";
    let input_lines = handle_input(filename);
    let width = input_lines[0].len();
    let height = input_lines.len();
    info!("input_lines: rows = {:?}, columns = {}", width, height);

    let (mut dist, mut prev, mut p_queue, input_graph) =
        make_init_data_with_priority(&input_lines, TILE_COUNT);
    // display_map_data(&edges, "Input Queue Table", TILE_COUNT*width as i32);

    let mut visited: HashMap<(i32, i32), usize> = HashMap::new();
    let shortest_value = dijkstra_search_with_priority(
        &input_graph,
        &input_lines,
        &mut dist,
        &mut prev,
        &mut p_queue,
        &mut visited,
        TILE_COUNT,
    );
    // display_shortest_path(&prev, &dist, &visited, &(TILE_COUNT * width as i32 -1, TILE_COUNT * height as i32 -1));

    info!("shortest_value  = {}", shortest_value);
}

// type InitDataType = (HashMap<(i32,i32), usize>, HashMap<(i32,i32), Option<(i32,i32)>>, HashMap<(i32,i32), usize>);
type InitDataType = (
    HashMap<(i32, i32), usize>,
    HashMap<(i32, i32), Option<(i32, i32)>>,
    PriorityQueue<(i32, i32), Reverse<usize>>,
    HashMap<(i32, i32), usize>,
);

fn make_init_data_with_priority(input_lines: &[Vec<usize>], tile_count: i32) -> InitDataType {
    fn expand_to_right(
        row_len: i32,
        col_len: i32,
        tile_i: i32,
        tile_j: i32,
        dist: &mut HashMap<(i32, i32), usize>,
        prev: &mut HashMap<(i32, i32), Option<(i32, i32)>>,
        priority_queue: &mut PriorityQueue<(i32, i32), Reverse<usize>>,
        input_graph: &mut HashMap<(i32, i32), usize>,
    ) {
        debug!(
            "=== A (Horizontal): COPY INPUT DATA TO RIGHT TILE -- tile = [{tile_i},{tile_j}] ==="
        );
        (0..row_len).for_each(|row| {
            (0..col_len).for_each(|col| {
                let (new_row, new_col) = (tile_i * row_len + row, tile_j * col_len + col);
                let prev_row = new_row;
                let prev_col = new_col - col_len;

                dist.insert((new_row, new_col), usize::MAX);
                prev.insert((new_row, new_col), None);
                priority_queue.push((new_row, new_col), Reverse(usize::MAX));
                
                let prev_value = *input_graph.get(&(prev_row, prev_col)).unwrap();
                let mut new_value = 1 + prev_value;
                if new_value > 9 {
                    new_value = 1;
                }
                input_graph.insert((new_row, new_col), new_value);
                if new_row % 100 == 0 && new_col % 100 == 1 {
                    debug!("--  Prev[{prev_row},{prev_col}] -> New[{new_row},{new_col}] || = {prev_value} -> {new_value}");
                }
            });
        });
    }

    fn expand_to_down(
        row_len: i32,
        col_len: i32,
        tile_i: i32,
        tile_j: i32,
        dist: &mut HashMap<(i32, i32), usize>,
        prev: &mut HashMap<(i32, i32), Option<(i32, i32)>>,
        priority_queue: &mut PriorityQueue<(i32, i32), Reverse<usize>>,
        input_graph: &mut HashMap<(i32, i32), usize>,
    ) {
        debug!("=== B (Vertical): COPY INPUT DATA TO DOWN TILE -- tile = [{tile_i},{tile_j}] ====");
        (0..row_len).for_each(|row| {
            (0..col_len).for_each(|col| {
                let (new_row, new_col) = (tile_i * row_len + row, tile_j * col_len + col);
                let prev_row = new_row - row_len;
                let prev_col = new_col;
    
                dist.insert((new_row, new_col), usize::MAX);
                prev.insert((new_row, new_col), None);
                priority_queue.push((new_row, new_col), Reverse(usize::MAX));
    
                let prev_value = *input_graph.get(&(prev_row, prev_col)).unwrap();
                let mut new_value = 1 + prev_value;
                new_value = if new_value > 9 { 1 } else { new_value };
                input_graph.insert((new_row, new_col), new_value);
                if new_row % 100 == 0 && new_col % 100 == 1 {
                    debug!("--  Prev[{prev_row},{prev_col}] -> New[{new_row},{new_col}] || {prev_value} -> {new_value}");
                }
            });
        });
    }

    //--------------------------------------------------------------------
    let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
    let mut prev: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut input_graph: HashMap<(i32, i32), usize> = HashMap::new();
    let mut priority_queue: PriorityQueue<(i32, i32), Reverse<usize>> = PriorityQueue::new();
    let row_len = input_lines.len() as i32;
    let col_len = input_lines[0].len() as i32;
    let source = (0, 0);

    // dist[(0,0)] <-- 0: 초기화
    dist.insert(source, 0);
    priority_queue.push(source, Reverse(0));

    input_lines.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, value)| {
            let vertex = (row as i32, col as i32);
            input_graph.insert(vertex, *value);
            if vertex != source {
                dist.insert(vertex, usize::MAX);
                prev.insert(vertex, None);
                //-- priority_queue.push(key, Reverse(value));
                //   Reverse(value) - 최대값이 아니라 최소값이 우선순위가 높도록 저장 한다.
                priority_queue.push(vertex, Reverse(usize::MAX));
            }
        });
    });

    //-- expand input data: horizontally and vetically
    for tile_i in 0..tile_count {
        for tile_j in 0..tile_count {
            if (tile_i, tile_j) == (0, 0) {
                continue;
            }
            if tile_j != 0 {
                // same row-position, different column-position
                expand_to_right(
                    row_len,
                    col_len,
                    tile_i,
                    tile_j,
                    &mut dist,
                    &mut prev,
                    &mut priority_queue,
                    &mut input_graph,
                );
            } else {
                // same column-position, difference row-position
                expand_to_down(
                    row_len,
                    col_len,
                    tile_i,
                    tile_j,
                    &mut dist,
                    &mut prev,
                    &mut priority_queue,
                    &mut input_graph,
                );
            }
        }
    }

    (dist, prev, priority_queue, input_graph)
}

// (Right, Down)
// const DELTAS: &[(i32,i32); 2] = &[(1,0), (0,1)];
//                                  (Right, Down,  Left,    Up)
const NEIGHBORS: &[(i32, i32); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn dijkstra_search_with_priority(
    input_graph: &HashMap<(i32, i32), usize>,
    input_lines: &[Vec<usize>],
    dist: &mut HashMap<(i32, i32), usize>,
    prev: &mut HashMap<(i32, i32), Option<(i32, i32)>>,
    priority_queue: &mut PriorityQueue<(i32, i32), Reverse<usize>>,
    visited: &mut HashMap<(i32, i32), usize>,
    tile_count: i32,
) -> usize {
    let width: i32 = tile_count * input_lines[0].len() as i32;
    let height: i32 = tile_count * input_lines.len() as i32;
    let dest_node = (width - 1, height - 1);
    let mut loop_count: i32 = 0;

    while let Some((current, Reverse(cost))) = priority_queue.pop() {
        // -- 매번 현재 priority queue에서 최소거리 node를 찾는다.
        visited.insert(current, cost);

        // for each neighbor v of min_vertex node, where each v is in input graph
        for delta in NEIGHBORS {
            let neighbor = (current.0 + delta.0, current.1 + delta.1);
            if let Some(&weight) = input_graph.get(&neighbor) {
                let new_cost = cost + weight;
                if new_cost < *dist.entry(neighbor).or_insert(usize::MAX) {
                    *dist.get_mut(&neighbor).unwrap() = new_cost;
                    *prev.get_mut(&neighbor).unwrap() = Some(current);
                    priority_queue.push(neighbor, Reverse(new_cost));
                }
            }
        }

        if loop_count % 1000 == 0 {
            debug!("Loop Count = [{}]", loop_count);
        }

        loop_count += 1;
    }

    info!(
        "PREV[target={:?}] = {:?}",
        &dest_node,
        prev.get(&dest_node).unwrap()
    );
    info!("DIST[target={:?}] = {:?}", dest_node, dist.get(&dest_node));
    info!("Total Loop Count = [{}]", loop_count);
    info!("Shortest Path Value  = {}", *dist.get(&dest_node).unwrap());

    *dist.get(&dest_node).unwrap()
}

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
