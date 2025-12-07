// advent/day_16.rs - part one
use log::{debug, error, info};
use std::fs::File;
use std::io::{BufRead, BufReader};

// use lazy_static::lazy_static;
// lazy_static!{
//     static ref HEX2BIN: HashMap<char, &'static str> = [
//         ('0', "0000"),
//         ('1', "0001"),
//         ('2', "0010"),
//         ('3', "0011"),
//         ('4', "0100"),
//         ('5', "0101"),
//         ('6', "0110"),
//         ('7', "0111"),
//         ('8', "1000"),
//         ('9', "1001"),
//         ('A', "1010"),
//         ('B', "1011"),
//         ('C', "1100"),
//         ('D', "1101"),
//         ('E', "1110"),
//         ('F', "1111"),
//         ].iter().copied().collect();
//  }

use phf::phf_map;
static HEX2BIN: phf::Map<char, &'static str> = phf_map! {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
};

fn handle_input(filename: &str) -> String {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    info!("[*] Input Filename: {}", filename);

    let single_line = &lines[0];
    info!(
        "[ ] Input (single) Line: len={}, data=[{}]",
        single_line.len(),
        single_line
    );

    let binary_line = single_line
        .chars()
        .enumerate()
        .flat_map(|(_i, ch)| (*HEX2BIN.get(&ch).unwrap()).chars())
        .collect();

    binary_line
}

// ===============================================
//  --- Day 16: Packet Decoder, Part One ---,
//  ===============================================
//  [*] Input Filename: input/day_16-input.txt
//  [ ] Input (single) Line: len=1354, data=[C20D59802D2B0B.....]
//
// [**] version all items = [6, 3, 6, 0, 5, 7, 3, 7, 2, 4, 5, 4, 3, 6, 6, 0, 2, 6, 3, 7, 1, 0, 1, 3,
//      5, 2, 0, 6, 0, 2, 7, 7, 3, 1, 5, 1, 0, 7, 1, 7, 1, 2, 6, 1, 6, 2, 6, 0, 6, 2, 7, 5, 3, 0, 3,
//      3, 6, 4, 0, 7, 2, 3, 7, 6, 6, 7, 0, 0, 7, 0, 7, 5, 1, 6, 6, 2, 5, 6, 1, 3, 4, 7, 7, 2, 7, 2,
//      7, 5, 7, 7, 6, 6, 2, 7, 6, 4, 3, 4, 1, 2, 1, 7, 0, 1, 2, 6, 4, 0, 4, 0, 0, 3, 7, 1, 1, 0, 2,
//      6, 6, 5, 0, 3, 6, 5, 6, 6, 0, 4, 1, 7, 1, 2, 5, 4, 5, 2, 6, 6, 5, 5, 3, 1, 0, 7, 3, 6, 6, 3,
//      5, 7, 3, 1, 2, 0, 0, 4, 2, 4, 0, 7, 2, 4, 2, 2, 5, 3, 5, 5, 5, 0, 5, 5, 7, 6, 2, 3, 5, 1, 2,
//      6, 1, 7, 4, 2, 3, 2, 1, 0, 0, 1, 5, 7, 2, 2, 5, 6, 0, 7, 0, 6, 6, 5, 1, 6, 1, 1, 2, 3, 1, 6,
//      7, 2, 5, 0, 4, 3, 5, 5, 0, 4, 3, 1, 7, 1, 4, 5, 4, 6, 2, 5, 7, 1, 3, 5, 2, 3, 6, 1, 0, 7, 2,
//      0, 4, 7, 2, 0, 4, 4, 3, 1, 1, 3, 0, 7, 6, 1, 6, 3, 5, 6, 3, 4, 0, 7, 6, 1, 5, 7, 0]
//  [**] version sum = 963

pub fn day_16_part_one() {
    info!("===============================================");
    info!("--- Day 16: Packet Decoder, Part One ---, ");
    info!("===============================================");
    let filename = "input/day_16-test-01.txt";
    // let filename = "input/day_16-test-02.txt";
    // let filename = "input/day_16-test-03.txt";
    // let filename = "input/day_16-sample-01.txt";
    // let filename = "input/day_16-sample-02.txt";
    // let filename = "input/day_16-sample-03.txt";
    // let filename = "input/day_16-sample-04.txt";
    // let filename = "input/day_16-input.txt";
    let input_line = handle_input(filename);
    let input_len = input_line.len();
    info!(
        "input_line(binary format): len = {}, data = {:?}",
        input_len, input_line
    );

    let version_vec = parse_packets(&input_line);
    info!("[**] all version items = {:?}", version_vec);
    info!("[**] version sum = {}", version_vec.iter().sum::<u32>());
}

fn parse_packets(input_line: &str) -> Vec<u32> {
    let input_slice = input_line;
    let mut version_vec = Vec::new();
    let mut new_pos = 0usize;

    loop {
        info!("");
        debug!(
            "=======| start_pos = {}, (remained) input_len = {} |==============",
            new_pos,
            input_slice[new_pos..].len()
        );
        info!(
            "[INPUT] = [{} {} {} ...] (20 chars) ",
            &input_slice[new_pos..new_pos + 3],
            &input_slice[new_pos + 3..new_pos + 6],
            &input_slice[new_pos + 6..new_pos + 7],
        );
        let version = get_packet_version(input_slice, new_pos);
        let type_id = get_packet_type_id(input_slice, new_pos + 3);
        let type_len_id = get_packet_type_length_id(input_slice, new_pos + 6);

        version_vec.push(version);
        debug!(
            "[VVV] VERSION = {}, |{}|, start_pos = ({})",
            version,
            &input_slice[new_pos..new_pos + 3],
            new_pos
        );
        debug!(
            "[TTT]    TYPE = {}, |{}|, start_pos = ({})",
            type_id,
            &input_slice[new_pos + 3..new_pos + 6],
            new_pos + 3
        );

        // type_ID == 4, literal packet
        // otherwise, operator packet
        if type_id == 4 {
            info!( "     |-- LITERAL PACKET |{}| ------------", &input_slice[new_pos + 3..new_pos + 6] );
            literal_packet_handler(input_slice, &mut new_pos, &mut version_vec);
        } else {
            // type ID != 4,  operator packets
            info!( "     |== OPERATOR PACKET ({}) -----------", &input_slice[new_pos + 3..new_pos + 6] );
            debug!( "     [I] Type LEN ID (1/0) = {}, |{}|, start_pos = ({})",
                type_len_id, &input_slice[new_pos + 6..new_pos + 7], new_pos + 6 );
            operator_packet_handler(input_slice, &mut new_pos, &mut version_vec);
        }

        if new_pos >= input_slice.len() {
            info!( "[^--^] [All Inputs Handled], new_pos = {}, input_line.len() = {}", new_pos, input_slice.len());
            break;
        }

        //-- update new input_slice
        if input_slice[new_pos..].len() <= 6 {
            info!("[^--^] [All Inputs Handled] new_pos = {}, remained inputs = {:?}, input_line.len() = {}",
                    new_pos, &input_slice[new_pos..], input_slice.len());
            break;
        } else {
            // info!("[*] current input = {:?}", &input_slice[old_pos..new_pos]);
            info!(
                "[*] next input = [{} {} ({})=I].....",
                &input_slice[new_pos..new_pos + 3],
                &input_slice[new_pos + 3..new_pos + 6],
                &input_slice[new_pos + 6..new_pos + 7]
            );
        }
    }

    version_vec
}

fn get_int_number(substr: &str) -> u32 {
    let mut num = 0;
    substr.chars().rev().enumerate().for_each(|(i, c)| {
        // num += u32::pow(c.to_digit(2).unwrap() as u32, i as u32);
        num += c.to_digit(2).unwrap() * u32::pow(2, i as u32);
    });
    // debug!("  substr = |{}|(len={}), num_value = {}", substr, substr.len(), num);
    num
}

fn get_packet_version(input_line: &str, start_pos: usize) -> u32 {
    let end_pos = start_pos + 3;
    let version_str = &input_line[start_pos..end_pos];
    let version = get_int_number(version_str);
    // debug!("[**] version = {}, version_str = {}", version, version_str);
    version
}

fn get_packet_type_id(input_line: &str, start_pos: usize) -> u32 {
    let end_pos = start_pos + 3;
    let type_id_str = &input_line[start_pos..end_pos];
    let type_id = get_int_number(type_id_str);
    // debug!("   ++ PACKET_TYPE_ID = {}, (str = |{}|), (start_pos, end_pos) = ({},{})", type_id, type_id_str, start_pos, end_pos);
    type_id
}

fn get_packet_type_length_id(input_slice: &str, start_pos: usize) -> u32 {
    let end_pos = start_pos + 1;
    let type_len_flag_str = &input_slice[start_pos..end_pos];
    let type_len_flag = get_int_number(type_len_flag_str);
    // debug!("  type_len_flag = {}, type_len_flag_str = {}", type_len_flag, type_len_flag_str);
    type_len_flag
}

fn get_subpacket_length_bits(input_slice: &str) -> u32 {
    //-- input_slice : 15 bits (total length of sub-packets in bits)
    let subpacket_len_bits = get_int_number(input_slice);
    debug!(
        "         - sub-packet LEN (bits) = {}, subpacket_len_str = {}",
        subpacket_len_bits, input_slice
    );
    subpacket_len_bits
}

fn get_subpacket_count(input_slice: &str) -> u32 {
    //-- input_slice: 11 bits (# of sub-packets)
    let subpacket_count = get_int_number(input_slice);
    debug!(
        "         - sub-packet COUNT = {}, subpacket_count_str = {}",
        subpacket_count, input_slice
    );
    subpacket_count
}

fn literal_packet_handler(input_slice: &str, new_pos: &mut usize, _version_vec: &mut Vec<u32>) {
    let new_start = *new_pos;
    let current_input = &input_slice;
    let version = get_int_number(&current_input[new_start..new_start + 3]);
    let mut literal_start = new_start + 6;
    let mut literal_end = new_start + 6 + 5;
    let mut literal_value = &input_slice[literal_start..literal_end];
    let mut loop_count = 1;
    debug!("     + LITERAL packet: new_start = {},", new_start);
    debug!("     + LITERAL packet: Version = {} [*]", version);

    loop {
        debug!(
            "       - [{}] LITERAL value(5 bits) = {} ({})",
            loop_count,
            literal_value,
            get_int_number(literal_value)
        );
        if literal_value.chars().nth(0).unwrap() == '0' {
            break;
        }
        loop_count += 1;
        literal_start = literal_end;
        literal_end += 5;
        literal_value = &input_slice[literal_start..literal_end];
    }

    *new_pos = *new_pos + 6 + (loop_count * 5);
}

fn operator_packet_handler(input_slice: &str, new_pos: &mut usize, _version_vec: &mut Vec<u32>) {
    let new_start = *new_pos;
    let current_input = &input_slice;
    // let version = get_int_number(&current_input[new_start..new_start+3]);
    // let type_id = get_packet_type_id(input_slice, new_start+3);
    let type_len_id = get_int_number(&current_input[new_start + 6..new_start + 7]);
    // debug!("[VVV] operator packet: Version = {}", version);
    // debug!("[TTT] operator packet: Packet Type (ID) = {}", type_id);
    // debug!("  [I] operator packet: Type len ID = {}", type_len_id);
    debug!("         operator packet: new_start = {},", new_start);
    debug!(
        "         operator packet: current input_len = {}, current_input = [{:?}]...",
        &current_input[new_start..].len(),
        &current_input[new_start..new_start + 7]
    );

    match type_len_id {
        0 => {
            let t_start = new_start + 7;
            let t_end = new_start + 7 + 15;
            debug!(
                "     [L] operator packet (I=0, L-15): ({},{}) = {}",
                t_start,
                t_end,
                t_end - t_start
            );
            *new_pos = if t_start < current_input.len() && t_end < current_input.len() {
                let sub_packet_len = get_subpacket_length_bits(&current_input[t_start..t_end]);
                debug!(
                    "         - operator packet (I=0, L-15): total sub-packet size in BITS = {}",
                    sub_packet_len
                );
                *new_pos + 7 + 15
            } else {
                debug!("[***] operator packet [I=0]: not enough input remians");
                *new_pos + 7
            };
        }
        1 => {
            let t_start = new_start + 7;
            let t_end = new_start + 7 + 11;
            debug!(
                "     [L] operator packet (I=1, L-11): ({},{}) = {}",
                t_start,
                t_end,
                t_end - t_start
            );
            *new_pos = if t_start < current_input.len() && t_end < current_input.len() {
                let sub_packet_count = get_subpacket_count(&current_input[t_start..t_end]);
                debug!(
                    "         - operator packet (I=1, L-11): # of sub-packets = {}",
                    sub_packet_count
                );
                *new_pos + 7 + 11
            } else {
                debug!("[***] operator packet [I=1]: not enough input remians");
                *new_pos + 7
            };
        }
        _ => {
            error!("Error in operator_packet_handler() - type_id is not in range..")
        }
    }
}
