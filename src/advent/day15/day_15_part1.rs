use core::str;
use log::{debug, info};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

//------------------------------
// Loop Count = [9000]
// PREV[target=(99, 99)] = Some((99, 98)), DIST[target] = Some(714)
// Total Loop Count = [10000]
// shortest_value  = 714
//-----------------------------

pub fn day_15_part_one() {
    info!("===============================================");
    info!("--- Day 15: Chiton, Part One ---, ");
    info!("===============================================");
    // let filename = "input/day_15-sample.txt";
    let filename = "input/day_15-input.txt";
    let input_lines = handle_input(filename);
    let width = input_lines[0].len();
    let height = input_lines.len();
    info!("input_lines: rows = {:?}, columns = {}", width, height);

    let (mut dist, mut prev, mut queue) = make_init_data(&input_lines);
    let shortest_value = dijkstra_search(&mut dist, &mut prev, &mut queue, &input_lines);
    info!("shortest_value  = {}", shortest_value);
    // display_shortest_path(&prev, &dist, &input_lines, &(width as i32 -1, height as i32 -1));

    // display_input_data(&input_lines);
    // display_map_data(&dist, "Distance Table", width as i32);
}

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
                //-- .map(|c| c.to_string().parse::<i32>().unwrap())
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            // debug!("{:?}", risk_levels);
            risk_levels
        })
        .collect::<Vec<Vec<_>>>();

    input_lines
}

#[allow(dead_code)]
// fn display_input_data(input_lines: &Vec<Vec<usize>>) {
fn display_input_data(input_lines: &[Vec<usize>]) {
    info!("---- 🍏🍒 Input Data 🍏🍒----");
    input_lines.iter().enumerate().for_each(|(row, line)| {
        let mut format_str = format!("{:2}: ", row);
        line.iter().enumerate().for_each(|(_col, value)| {
            format_str += &*format!("{:1} ", value);
        });
        format_str += &*format!("");
        info!(" {} ", format_str);
    });
}

#[allow(dead_code)]
fn display_map_data(data_map: &HashMap<(i32, i32), usize>, msg: &str, len: i32) {
    info!("---- 🍏🍒 {} 🍏🍒----", msg);
    for row in 0..len {
        let mut format_str = format!("{:2}: ", row);
        for col in 0..len {
            let v = *data_map.get(&(row, col)).unwrap();
            format_str += &*format!("{:3} ", v);
        }
        format_str += &*format!("");
        info!(" {} ", format_str);
    }
}

//--------------------------------------------------------------------------------------------------
type InitDataType = (
    HashMap<(i32, i32), usize>,
    HashMap<(i32, i32), Option<(i32, i32)>>,
    HashMap<(i32, i32), usize>,
);

fn make_init_data(input_lines: &[Vec<usize>]) -> InitDataType {
    let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
    let mut prev: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut queue: HashMap<(i32, i32), usize> = HashMap::new();

    input_lines.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, value)| {
            let (r, c) = (row as i32, col as i32);
            dist.insert((r, c), usize::MAX);
            prev.insert((r, c), None);
            queue.insert((r, c), *value as usize);
            // info!("queue[{},{}] = ({:?})", x, y, queue.get(&(x,y)));
        });
        // info!("input_table[{}](len:{}) = {:?}", i, input_table[i].len(), input_table[i]);
    });
    // (0,0) == source
    *dist.get_mut(&(0, 0)).unwrap() = 0;

    (dist, prev, queue)
}

// --------------------------------------------------
// Dijstra Algorithm
//  1  function Dijkstra(Graph, source):
//  2
//  3      for each vertex v in Graph.Vertices:
//  4          dist[v] ← INFINITY
//  5          prev[v] ← UNDEFINED
//  6          add v to Q
//  7      dist[source] ← 0
//  8
//  9      while Q is not empty:
// 10          u ← vertex in Q with minimum dist[u]
// 11          remove u from Q
// 12
// 13          for each neighbor v of u still in Q:
// 14              alt ← dist[u] + Graph.Edges(u, v)
// 15              if alt < dist[v]:
// 16                  dist[v] ← alt
// 17                  prev[v] ← u
// 18
// 19      return dist[], prev[]
// --------------------------------------------------
// dist - is an array that contains the current distances from the SOURECE to other vertices.
//  dist[u] is the current distance from the source to the vertex u.
// prev - an array that contains poinsters to the previous-hop nodes on the shortest path from
//        source to the fiven vertex.
// u <- vertex in Q with min dist[u] - searches for the vertex u in the vertex set Q that has
//       the smallest dist[u] value
// Graph.Edges(u, v) - returns the length of the edge joining (i.e. the distance between) the two
//       neighbor-nodes u and v.
// alt - is the lenght of the path from the source node to the neighbor node v if it were to go
//       through u. If the path is shorter than the current shortest path recored for v, the the
//       distance of V is updated to alt.

// const DELTAS: &[(i32,i32); 2] = &[(1,0), (0,1)];
// (Right, Down, Left, Up)
const DELTAS: &[(i32, i32); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn dijkstra_search(
    dist: &mut HashMap<(i32, i32), usize>,
    prev: &mut HashMap<(i32, i32), Option<(i32, i32)>>,
    queue: &mut HashMap<(i32, i32), usize>,
    // input_lines: &Vec<Vec<usize>>) -> usize {
    input_lines: &[Vec<usize>],
) -> usize {
    let width: i32 = input_lines[0].len() as i32;
    let height: i32 = input_lines.len() as i32;
    let dest_node = (width - 1, height - 1);
    let mut loop_count: i32 = 0;

    while !queue.is_empty() {
        let mut smallest_u = usize::MAX;
        let mut found_u: (i32, i32) = (0, 0);
        // find the smallest u in queue with minimum dist[u]
        // -- 매번 현재 queue (unvisited nodes)에서 최소거리 node를 찾는다.
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
        queue.remove(&found_u);

        // info!("--- for each neighbor v of found_u ({:?}", found_u);
        // for each neighbor v of found_u node, where each v is still in queue (unvisited)
        for delta in DELTAS {
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
        "PREV[target={:?}] = {:?}, DIST[target] = {:?}",
        &dest_node,
        prev.get(&dest_node).unwrap(),
        dist.get(&dest_node)
    );
    info!("Total Loop Count = [{}]", loop_count);
    *dist.get(&dest_node).unwrap()
}

#[allow(dead_code)]
fn display_shortest_path(
    prev: &HashMap<(i32, i32), Option<(i32, i32)>>,
    dist: &HashMap<(i32, i32), usize>,
    input_lines: &Vec<Vec<usize>>,
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
        info!(
            "node[{},{}] = ({}), dist({},{}) = {:?}",
            row,
            col,
            input_lines[*row as usize][*col as usize],
            row,
            col,
            dist.get(&(*row, *col))
        );
    });
}
