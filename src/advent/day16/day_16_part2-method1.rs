// advent/day_16.rs - part two
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u64;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!{
    static ref HEX2BIN: HashMap<char, &'static str> = [
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
        ].iter().copied().collect();
 }





#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TypeLenID {
    L11,
    L15,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TypeLenValue {
    L11 { str_value: String, sub_packets_count: u32 },
    L15 { str_value: String, sub_packets_len: u32 },
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum OperatorType {
    SUM,
    PRODUCT,
    MINIMUM,
    MAXIMUM,
    GT,
    LT,
    EQ,
    UKN,
}

type OpComputeFn = fn(&OperatorPktStatus, &mut OperatorPktStatus, u32);

// lazy_static! {
// static ref OP2FUNC: HashMap<OperatorType, OpComputeFn> = [
//     (OperatorType::SUM,  compute_sum),
//     (OperatorType::PRODUCT,  compute_product),
//     (OperatorType::MAXIMUM,  compute_maximum),
//     (OperatorType::MINIMUM,  compute_minimum),
//     (OperatorType::GT,  compute_gt),
//     (OperatorType::LT,  compute_lt),
//     (OperatorType::EQ,  compute_eq),
// ].iter().copied().collect();
// }

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct OperatorPktStatus {
    pub packet_seq: u32,
    pub op_type: OperatorType,
    pub start_input_pos: usize,
    pub type_len_id: TypeLenID,
    pub type_len_value: TypeLenValue,
    pub consumed_len: u32,
    pub consumed_count: u32,
    pub value_vec: Vec<u128>,
    pub final_result: u128,
    pub finalized: bool,
}

impl OperatorPktStatus {
    pub fn new(operator_count: u32) -> Self {
        OperatorPktStatus {
            packet_seq: operator_count,
            op_type: OperatorType::UKN,
            start_input_pos: 0,
            type_len_id: TypeLenID::L11,
            type_len_value: TypeLenValue::L11 {
                str_value: "".to_string(),
                sub_packets_count: 0,
            },
            consumed_len: 0,
            consumed_count: 0,
            value_vec: Vec::new(),
            final_result: u128::MAX,
            finalized: false,
        }
    }

    pub fn set_op_type(&mut self, op_type: OperatorType) {
        self.op_type = op_type;
    }

    pub fn set_start_input_pos(&mut self, new_pos: usize) {
        self.start_input_pos = new_pos;
    }
    pub fn set_type_len_id(&mut self, type_len_id: TypeLenID) {
        self.type_len_id = type_len_id;
    }

    pub fn set_type_len_value(&mut self, type_len_value: TypeLenValue) {
        self.type_len_value = type_len_value;
    }

    pub fn add_value(&mut self, literal_value: u128) {
        self.value_vec.push(literal_value);
    }

    pub fn add_consumed_len(&mut self, consumed_len: u32) {
        self.consumed_len += consumed_len;
    }

    pub fn add_consumed_count(&mut self, consumed_count: u32) {
        self.consumed_count += consumed_count;
    }

    pub fn set_final_value(&mut self, value: u128) {
        self.final_result = value;
        self.finalized = true;
    }

    pub fn is_finalized(&self) -> bool {
        self.finalized
    }
}

fn handle_input(filename: &str) -> String {
    let file = File::open(filename).expect("Couldn't open input");
    let buf = BufReader::new(file);
    let lines = buf.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    info!("[*] Input Filename: {}", filename);

    let single_line = &lines[0];
    info!("[ ] Input (single) Line: len={}, data=[{}]", single_line.len(), single_line);

    let binary_line = single_line
        .chars()
        // .enumerate()
        // .flat_map(|(_i, ch)| (*HEX2BIN.get(&ch).unwrap()).chars())
        .flat_map(|ch| (*HEX2BIN.get(&ch).unwrap()).chars())
        .collect();

    binary_line
}

//==========================================================
// Final Data and Value
//----------------------------------------------------------
// INFO  advent_code_2021::advent::day16::day_16_part2 > [***] final_data = 🍏🍏🍏🍏 OperatorPktStatus {
//     op_type: SUM,
//     start_input_pos: 0,
//     last_input_pos: 0,
//     type_len_id: L11,
//     type_len_value: L11 {
//         str_value: "00000110101",
//         sub_packets_count: 53,
//     },
//     consumed_len: 5391,
//     consumed_count: 53,
//     value_vec: [
//         0,
//         0,
//         0,
//         47144,
//         238,
//         87,
//         196,
//         657596,
//         9,
//         3,
//         0,
//         15811746,
//         12920678784,
//         0,
//         0,
//         18432,
//         172,
//         27,
//         0,
//         56,
//         339840,
//         20925,
//         0,
//         6,
//         43,
//         375791589211,
//         176,
//         0,
//         12,
//         20626,
//         5,
//         175,
//         0,
//         10,
//         3976415,
//         840,
//         0,
//         53086862385,
//         729288,
//         1058775397181,
//         369780,
//         118,
//         19911,
//         2,
//         15,
//         26058,
//         7,
//         48428531259,
//         136,
//         0,
//         1747,
//         1189708,
//         2517,
//     ],
//     final_result: 1549026292886,
//     finalized: true,
// } 🍏🍏🍏🍏
//-----------------------------------
//  INFO  advent_code_2021::advent::day16::day_16_part2 > [***] final_value = 🍏🍏 [1549026292886] 🍏🍏
//  INFO  advent_code_2021::advent::day16::day_16_part2 > [ ] packet_count = 269, op = 100, literal = 169,
//  INFO  advent_code_2021::advent::day16::day_16_part2 > [**] version all items = [6, 3, 6, 0, 5, 7, 3, 7, 2,
// 4, 5, 4, 3, 6, 6, 0, 2, 6, 3, 7, 1, 0, 1, 3, 5, 2, 0, 6, 0, 2, 7, 7, 3, 1, 5, 1, 0, 7, 1, 7, 1, 2, 6, 1,
// 6, 2, 6, 0, 6, 2, 7, 5, 3, 0, 3, 3, 6, 4, 0, 7, 2, 3, 7, 6, 6, 7, 0, 0, 7, 0, 7, 5, 1, 6, 6, 2, 5, 6, 1, 3,
// 4, 7, 7, 2, 7, 2, 7, 5, 7, 7, 6, 6, 2, 7, 6, 4, 3, 4, 1, 2, 1, 7, 0, 1, 2, 6, 4, 0, 4, 0, 0, 3, 7, 1, 1, 0,
//  2, 6, 6, 5, 0, 3, 6, 5, 6, 6, 0, 4, 1, 7, 1, 2, 5, 4, 5, 2, 6, 6, 5, 5, 3, 1, 0, 7, 3, 6, 6, 3, 5, 7, 3, 1,
//  2, 0, 0, 4, 2, 4, 0, 7, 2, 4, 2, 2, 5, 3, 5, 5, 5, 0, 5, 5, 7, 6, 2, 3, 5, 1, 2, 6, 1, 7, 4, 2, 3, 2, 1, 0,
//  0, 1, 5, 7, 2, 2, 5, 6, 0, 7, 0, 6, 6, 5, 1, 6, 1, 1, 2, 3, 1, 6, 7, 2, 5, 0, 4, 3, 5, 5, 0, 4, 3, 1, 7, 1,
//  4, 5, 4, 6, 2, 5, 7, 1, 3, 5, 2, 3, 6, 1, 0, 7, 2, 0, 4, 7, 2, 0, 4, 4, 3, 1, 1, 3, 0, 7, 6, 1, 6, 3, 5, 6,
//  3, 4, 0, 7, 6, 1, 5, 7, 0]
//  INFO  advent_code_2021::advent::day16::day_16_part2 > [**] version sum = 963
// ==============================================================================

pub fn day_16_part_two() {
    info!("===============================================");
    info!("--- Day 16: Packet Decoder, Part Two ---, ");
    info!("===============================================");
    //--- part 2: input samples
    // let filename = "input/day_16-sample-11.txt";
    // let filename = "input/day_16-sample-12.txt";
    // let filename = "input/day_16-sample-13.txt";
    // let filename = "input/day_16-sample-14.txt";
    // let filename = "input/day_16-sample-15.txt";
    // let filename = "input/day_16-sample-16.txt";
    // let filename = "input/day_16-sample-17.txt";
    // let filename = "input/day_16-sample-18.txt";
    let filename = "input/day_16-input.txt";
    let input_line = handle_input(filename);
    let input_len = input_line.len();
    info!("input_line(binary format): len = {}, data = {:?}", input_len, input_line);

    let (version_sum, final_value) = parse_packets(&input_line);
    info!("[**] version all items = {:?}", version_sum);
    info!("[**] part-1: version sum = {}", version_sum.iter().sum::<u32>());
    info!("[**] part-2: final value = {}", final_value);
}

fn parse_packets(input_line: &str) -> (Vec<u32>, u128) {
    let input_slice = input_line;
    let mut version_vec = Vec::new();
    let mut new_pos = 0usize;
    let mut op_status_vec: Vec<OperatorPktStatus> = Vec::new();
    let mut packet_count = 1;
    let mut operator_count = 1;
    let mut literal_count = 1;

    let mut OP2FUNC: HashMap<OperatorType, OpComputeFn> = HashMap::new();
    OP2FUNC.insert(OperatorType::SUM, compute_sum);
    OP2FUNC.insert(OperatorType::PRODUCT, compute_product);
    OP2FUNC.insert(OperatorType::MAXIMUM, compute_maximum);
    OP2FUNC.insert(OperatorType::MINIMUM, compute_minimum);
    OP2FUNC.insert(OperatorType::GT, compute_gt);
    OP2FUNC.insert(OperatorType::LT, compute_lt);
    OP2FUNC.insert(OperatorType::EQ, compute_eq);
    let op2func = OP2FUNC;

    loop {
        let version = get_packet_version(input_slice, new_pos);
        let type_id = get_packet_type_id(input_slice, new_pos+3);
        let type_len_id = get_packet_type_length_id(input_slice, new_pos+6);

        version_vec.push(version);

        //-- DEBUG: status display
        if type_id != 4 {
            info!("");
            info!("🍏 [OPERATOR] 🍏 - [{}/{}]", operator_count, packet_count);
            info!("|------------------------------------------------------------");
            info!("| [INPUT] = [{} {} {} ...] (7 chars) ", &input_slice[new_pos..new_pos+3],
            &input_slice[new_pos+3..new_pos+6],
            &input_slice[new_pos+6..new_pos+7],);
            info!("|------------------------------------------------------------");
            debug!("[ ] start_pos = {}, (remained) input_len = {}", new_pos, input_slice[new_pos..].len());
        } else {
            info!("");
            info!("🍒 [DATA] 🍒  - [{}/{}]", literal_count, packet_count);
            info!("|=============================================================");
            info!("| [INPUT] = [{} {} ...] (6 chars...) ", &input_slice[new_pos..new_pos+3],
            &input_slice[new_pos+3..new_pos+6],);
            info!("|-------------------------------------------------------------");
            debug!("[ ] start_pos = {}, (remained) input_len = {}", new_pos, input_slice[new_pos..].len());
        }
        debug!("[VVV] VERSION = {}, |{}|, start_pos = ({})", version, &input_slice[new_pos..new_pos+3], new_pos);
        debug!("[TTT] TYPE_ID = {}, |{}|, start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);

        packet_count += 1;

        //--------------------------------------
        // type ID != 4, then Operator packets
        if type_id != 4 {
            // OPERATOR(CONTROL) packet
            let mut op_status = OperatorPktStatus::new(operator_count);
            set_packet_status_op_type(input_slice, &new_pos, type_id, &mut op_status);

            debug!("[I]   Type_LEN_ID = {}, |{}|, start_pos = ({})", type_len_id, &input_slice[new_pos+6..new_pos+7], new_pos+6);
            operator_packet_handler(input_slice, &mut new_pos, &mut version_vec, &mut op_status);
            info!("[OP]  OP_TYPE: 🍏🍏 ===== {:?} =====🍏🍏 ", op_status.op_type);
            debug!("[OP]  OperatorStatus = {:#?}", op_status);

            op_status_vec.push(op_status);
            compute_stack_data(&mut op_status_vec, &op2func);
            operator_count += 1;
        } else {
            // LITERAL(DATA) packet
            let mut op_status = op_status_vec.pop().unwrap();

            let _consumed_len = literal_packet_handler(input_slice, &mut new_pos, &mut version_vec, &mut op_status);
            compute_literal_packets_value(&mut op_status);
            debug!("[DATA] OperatorStatus = {:#?}", op_status);

            op_status_vec.push(op_status);
            compute_stack_data(&mut op_status_vec, &op2func);
            literal_count += 1;
        }

        //-- check loop exit condition
        if new_pos >= input_slice.len() || input_slice[new_pos..].len() <= 6 {
            info!("===== [EXIT MAIN LOOP] [All Inputs Handled] new_pos = {}, remained inputs = {:?}, input_line.len() = {}",
                    new_pos, &input_slice[new_pos..], input_slice.len());
            break;
        } else {
            info!("[LOOP] next input = [{} {} ({})=I].....", &input_slice[new_pos..new_pos+3],
                                                            &input_slice[new_pos+3..new_pos+6],
                                                            &input_slice[new_pos+6..new_pos+7]);
        }
    }

    let final_data = op_status_vec.first().unwrap();
    info!("[1] Packets in Stack = {:#?}", op_status_vec);
    info!("[2] OUTER-MOST Packet Data = 🍏🍏🍏🍏 {:#?} 🍏🍏🍏🍏", final_data);
    info!("[3] OUTER-MOST Values in final_data = {:?} 🍏🍏🍏🍏", final_data.value_vec);
    info!("[4] OUTER-MOST Final_value in final_data = 🍏🍏 [{:?}] 🍏🍏", final_data.final_result);
    info!("[5] packet_count = {}, op = {}, literal = {},", packet_count, operator_count, literal_count);

    (version_vec, final_data.final_result)
}

fn operator_packet_handler(input_slice: &str,
                           new_pos: &mut usize,
                           _version_vec: &mut Vec<u32>,
                           op_status: &mut OperatorPktStatus) -> () {
    let new_start = *new_pos;
    let current_input = &input_slice;
    let _version = get_int_number(&current_input[new_start..new_start+3]);
    let type_len_id = get_int_number(&current_input[new_start+6..new_start+7]);

    match type_len_id {
        0 => {
            op_status.set_type_len_id(TypeLenID::L15);
            let t_start = new_start+7;
            let t_end = new_start+7+15;

            if t_start < current_input.len() && t_end < current_input.len() {
                let sub_packets_len = get_subpacket_length_bits(&current_input[t_start..t_end]);
                debug!("[L-15] (I=0): sub-packets LEN in bits = {}, |{}|", sub_packets_len, &current_input[t_start..t_end]);
                *new_pos = *new_pos + 7 + 15;

                let type_len_value =  TypeLenValue::L15 {
                    str_value: current_input[t_start..t_end].to_string(),
                    sub_packets_len,
                };
                op_status.set_type_len_value(type_len_value);
            } else {
                debug!("[***] [L-15] [I=0]: not enough input remians");
                *new_pos += 7;

                let type_len_value =  TypeLenValue::L15 {
                    str_value: "".to_string(),
                    sub_packets_len: 0,
                };
                op_status.set_type_len_value(type_len_value);
            }
        },
        1 => {
            op_status.set_type_len_id(TypeLenID::L11);
            let t_start = new_start+7;
            let t_end = new_start+7+11;

            if t_start < current_input.len() && t_end < current_input.len() {
                let sub_packets_count = get_subpacket_count(&current_input[t_start..t_end]);
                debug!("[L-11] (I=1): sub-packets COUNT = {}, |{}|", sub_packets_count, &current_input[t_start..t_end]);
                *new_pos = *new_pos + 7 + 11;

                let type_len_value =  TypeLenValue::L11 {
                    str_value: current_input[t_start..t_end].to_string(),
                    sub_packets_count,
                };
                op_status.set_type_len_value(type_len_value);
            } else {
                debug!("[***] [L-11] (I=1): not enough input remians");
                *new_pos += 7;

                let type_len_value =  TypeLenValue::L11 {
                    str_value: "".to_string(),
                    sub_packets_count: 0,
                };
                op_status.set_type_len_value(type_len_value);
            }
        },
        _ => {
            error!("Error in operator_packet_handler() - type_id is not in range..")
        }
    }
}

fn literal_packet_handler(input_slice: &str,
                          new_pos: &mut usize,
                          _version_vec: &mut Vec<u32>,
                          op_status: &mut OperatorPktStatus) -> u32 {
    let mut consumed_len = 6;
    let new_start = *new_pos;
    let mut literal_start = new_start+6;
    let mut literal_end = new_start+6+5;
    let mut literal_value = &input_slice[literal_start..literal_end];
    let mut loop_count = 1;
    let mut literal_values = Vec::new();

    loop {
        debug!("    - [{}] DATA value(5 bits) = {} ({})", loop_count, literal_value, get_int_number(literal_value));
        let value = &literal_value[1..];
        literal_values.push(value);
        consumed_len += 5;

        if literal_value.chars().nth(0).unwrap() == '0' {
            break;
        }
        loop_count += 1;
        literal_start = literal_end;
        literal_end += 5;
        literal_value = &input_slice[literal_start..literal_end];
    }

    let literal_concat = literal_values.iter().copied().collect::<String>();
    let literal_single_value = get_u64_number(&literal_concat);

    debug!("[DATA] Final Value = [{}], |{}|(len={} bits)", literal_single_value, literal_concat, literal_concat.len());
    debug!("[DATA] Packet LEN = [{}] ", consumed_len);

    match op_status.type_len_id {
        TypeLenID::L11 => {
            op_status.add_value(literal_single_value as u128);
            op_status.add_consumed_count(1);
            op_status.add_consumed_len(consumed_len);
            debug!("[DATA] Acc-Consumed LEN = {} (bits), Acc-Consumed COUNT = {}", op_status.consumed_len, op_status.consumed_count);
        },
        TypeLenID::L15 => {
            op_status.add_value(literal_single_value as u128);
            op_status.add_consumed_len(consumed_len);
            op_status.add_consumed_count(1);
            debug!("[DATA] Acc-Consumed LEN = {} (bits), Acc-Consumed COUNT = {}", op_status.consumed_len, op_status.consumed_count);
        }
    }

    *new_pos = *new_pos + 6 + (loop_count * 5);
    consumed_len
}

fn compute_stack_data(op_status_vec: &mut Vec<OperatorPktStatus>, op2fn: &HashMap<OperatorType, OpComputeFn>) {
    debug!("🍏🍒 [STACK] 🍏🍒 ====>> Compute Stack Data ======");
    let mut  loop_count = 0;

    //-- while !op_status_vec.is_empty() {
    while op_status_vec.len() >= 1 {
        loop_count += 1;

        // POP the 1st data from the Stack
        let top_data = op_status_vec.pop();
        let top_data = match top_data {
            Some(data_value) => {
                if !data_value.is_finalized() {
                    op_status_vec.push(data_value);
                    debug!("[STACK] loop_count = {}", loop_count);
                    debug!("[STACK] NO-OP |-- compute_stack_data() - top_data NOT finalized");
                    debug!("[STACK] NO-OP |==> RETURN 🍏🍏🍏🍏🍏 =========");
                    return;
                }
                data_value
            },
            None => {
                debug!("[STACK] loop_count = {}", loop_count);
                warn!("[STACK] NO-OP |-- compute_stack_data() - top_data is NOT available");
                debug!("[STACK] NO-OP |==> RETURN 🍒🍒🍒🍒 =========");
                return;
            }
        };

        let top_data_length = match top_data.type_len_id {
            TypeLenID::L11 => {
                18
            },
            TypeLenID::L15 => {
                22
            }
        };

        // POP the 2nd data from the Stack
        let parent_data = op_status_vec.pop();
        let mut parent_data = match parent_data {
            Some(data_value) => {
                data_value
            },
            None => {
                debug!("[STACK] loop_count = {} ------------------", loop_count);
                warn!("[STACK] NO-OP |-- compute_stack_data() - Parent_data is NOT available");
                op_status_vec.push(top_data);
                debug!("[STACK] NO-OP |==> RETURN 🍒🍒 =========");
                return;
            }
        };

        debug!("[STACK] (count={}) [POP-1] - TOP = {:#?}", loop_count, top_data);
        debug!("[STACK] (count={}) [POP-2] - PARENT (🍒🍒 [BEFORE]) = {:#?}", loop_count, parent_data);

        // do Operation which is the Operatir inside the 2nd data
        info!("[STACK] OPERATION: 🍏----- [{:?}] -----🍏", parent_data.op_type);

        //-- approach 1:
         match parent_data.op_type {
            OperatorType::SUM => {
                compute_sum(&top_data, &mut parent_data, top_data_length);
            },
            OperatorType::PRODUCT => {
                compute_product(&top_data, &mut parent_data, top_data_length);
            },
            OperatorType::MINIMUM => {
                compute_minimum(&top_data, &mut parent_data, top_data_length);
            },
            OperatorType::MAXIMUM => {
                compute_maximum(&top_data, &mut parent_data, top_data_length);
            },
            OperatorType::GT => {
                compute_gt(&top_data, &mut parent_data, top_data_length);
            },
            OperatorType::LT => {
                compute_lt(&top_data, &mut parent_data, top_data_length);
            },
            OperatorType::EQ => {
                compute_eq(&top_data, &mut parent_data, top_data_length);
            },
            _ => {
                warn!("[STACK] TYPE_ID: - Unknown ({:?}) ----", parent_data.op_type);
            }
        }
        
        //-- approach 2:
        //-- op2fn.get(&parent_data.op_type).unwrap()(&top_data, &mut parent_data, top_data_length);
        // if let Some(op_func) = op2fn.get(&parent_data.op_type) {
        //     op_func(&top_data, &mut parent_data, top_data_length);
        // } else {
        //     warn!("[STACK] TYPE_ID: - Unknown ({:?}) ----", parent_data.op_type);
        // }
        //--------------------
        debug!("[STACK] (count={}) [PUSH] PARENT - (🍏🍏 [AFTER]) = {:#?}", loop_count, parent_data);

        op_status_vec.push(parent_data);
    }
    debug!("[STACK] loop_count = {}", loop_count);
    debug!("[STACK] |==> RETURN 🍏🍏🍏🍏🍏 =========");
}

fn compute_sum(top_data: &OperatorPktStatus, parent_data: &mut OperatorPktStatus, top_data_length: u32) {
    info!("[STACK]     |- OP TYPE ID: - SUM ({:?}) --------", parent_data.op_type);
    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            if parent_data.consumed_count < sub_packets_count {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_count >= sub_packets_count {
                    let result = parent_data.value_vec.iter().sum();
                    parent_data.set_final_value(result);
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len} => {
            if parent_data.consumed_len < sub_packets_len {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_len == sub_packets_len {
                    // let sum = parent_data.value_vec.iter().sum::<u128>();
                    let sum: u128 = parent_data.value_vec.iter()
                        .fold(0, |acc, &value| {
                            debug!("[STACK] OperatorType::SUM -- {}", value);
                            acc + value
                        });
                    parent_data.set_final_value(sum);
                }
            }
        }
    }
}

fn compute_product(top_data: &OperatorPktStatus, parent_data: &mut OperatorPktStatus, top_data_length: u32) {
    info!("[STACK]     |- OP TYPE ID: - PRODUCT ({:?}) ----", parent_data.op_type);
    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            if parent_data.consumed_count < sub_packets_count {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_count >= sub_packets_count {
                    let mut sum = 1;
                    parent_data.value_vec.iter().for_each(|value| {
                        debug!("[STACK] OperatorType::PRODUCT -- {}", value);
                        sum = sum * *value as u128;
                    });
                    parent_data.set_final_value(sum);
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len} => {
            if parent_data.consumed_len < sub_packets_len {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_len >= sub_packets_len {
                    let mut sum = 1;
                    parent_data.value_vec.iter().for_each(|value| {
                        debug!("[STACK] OperatorType::PRODUCT -- {}", value);
                        sum = sum * *value as u128;
                    });
                    parent_data.set_final_value(sum);
                }
            }
        }
    }
}

fn compute_minimum(top_data: &OperatorPktStatus, parent_data: &mut OperatorPktStatus, top_data_length: u32) {
    info!("[STACK]     |- OP TYPE ID: - MIN ({:?})  -------", parent_data.op_type);
    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            if parent_data.consumed_count < sub_packets_count {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_count >= sub_packets_count {
                    let min = parent_data.value_vec.iter().min();
                    match min {
                        Some(min_value) => {
                            parent_data.set_final_value(*min_value as u128);
                        },
                        None => {
                            warn!("[STACK]    TypeLenValue - None");
                        }
                    }
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len} => {
            if parent_data.consumed_len < sub_packets_len {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_len >= sub_packets_len {
                    let min = parent_data.value_vec.iter().min();
                    match min {
                        Some(min_value) => {
                            parent_data.set_final_value(*min_value as u128);
                        },
                        None => {
                            warn!("    TypeLenValue - None");
                        }
                    }
                }
            }
        }
    }
}

fn compute_maximum(top_data: &OperatorPktStatus, parent_data: &mut OperatorPktStatus, top_data_length: u32) {
    info!("[STACK]     |- OP TYPE ID: - MAX ({:?})  -------", parent_data.op_type);
    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            if parent_data.consumed_count < sub_packets_count {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_count >= sub_packets_count {
                    let max = parent_data.value_vec.iter().max();
                    match max {
                        Some(max_value) => {
                            parent_data.set_final_value(*max_value as u128);
                        },
                        None => {
                            warn!("[STACK]    TypeLenValue - None");
                        }
                    }
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len} => {
            if parent_data.consumed_len < sub_packets_len {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_len >= sub_packets_len {
                    let max = parent_data.value_vec.iter().max();
                    match max {
                        Some(max_value) => {
                            parent_data.set_final_value(*max_value as u128);
                        },
                        None => {
                            warn!("[STACK]    TypeLenValue - None");
                        }
                    }
                }
            }
        }
    }
}

fn compute_gt(top_data: &OperatorPktStatus, parent_data: &mut OperatorPktStatus, top_data_length: u32) {
    info!("[STACK]     |- OP TYPE ID: - GT ({:?})  --------", parent_data.op_type);
    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            if parent_data.consumed_count < sub_packets_count {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_count >= sub_packets_count {
                    let v1 = *parent_data.value_vec.first().unwrap();
                    let v2 = *parent_data.value_vec.get(1).unwrap();
                    if v1 >= v2 {
                        parent_data.set_final_value(1);
                    } else {
                        parent_data.set_final_value(0);
                    }
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len} => {
            if parent_data.consumed_len < sub_packets_len {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_len >= sub_packets_len {
                    let v1 = *parent_data.value_vec.first().unwrap();
                    let v2 = *parent_data.value_vec.get(1).unwrap();
                    if v1 >= v2 {
                        parent_data.set_final_value(1);
                    } else {
                        parent_data.set_final_value(0);
                    }
                }
            }
        }
    }
}

fn compute_lt(top_data: &OperatorPktStatus, parent_data: &mut OperatorPktStatus, top_data_length: u32) {
    info!("[STACK]     |- OP TYPE ID: - LT ({:?})  --------", parent_data.op_type);
    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            if parent_data.consumed_count < sub_packets_count {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_count >= sub_packets_count {
                    let v1 = *parent_data.value_vec.first().unwrap();
                    let v2 = *parent_data.value_vec.get(1).unwrap();
                    if v1 <= v2 {
                        parent_data.set_final_value(1);
                    } else {
                        parent_data.set_final_value(0);
                    }
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len} => {
            if parent_data.consumed_len < sub_packets_len {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_len >= sub_packets_len {
                    let v1 = *parent_data.value_vec.first().unwrap();
                    let v2 = *parent_data.value_vec.get(1).unwrap();
                    if v1 <= v2 {
                        parent_data.set_final_value(1);
                    } else {
                        parent_data.set_final_value(0);
                    }
                }
            }
        }
    }
}

fn compute_eq(top_data: &OperatorPktStatus, parent_data: &mut OperatorPktStatus, top_data_length: u32) {
    info!("[STACK]     |- OP TYPE ID: - EQ ({:?})  --------", parent_data.op_type);
    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            if parent_data.consumed_count < sub_packets_count {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_count >= sub_packets_count {
                    let v1 = *parent_data.value_vec.first().unwrap();
                    let v2 = *parent_data.value_vec.get(1).unwrap();
                    if v1 == v2 {
                        parent_data.set_final_value(1);
                    } else {
                        parent_data.set_final_value(0);
                    }
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len} => {
            if parent_data.consumed_len < sub_packets_len {
                parent_data.add_value(top_data.final_result);
                parent_data.add_consumed_count(1);
                parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                if parent_data.consumed_len >= sub_packets_len {
                    let v1 = *parent_data.value_vec.first().unwrap();
                    let v2 = *parent_data.value_vec.get(1).unwrap();
                    if v1 == v2 {
                        parent_data.set_final_value(1);
                    } else {
                        parent_data.set_final_value(0);
                    }
                }
            }
        }
    }
}

fn compute_literal_packets_value(op_status: &mut OperatorPktStatus) {
    info!("[COMPUTE DATA VALUE] 🍒----- {:?} -----🍒", op_status.op_type);
    match op_status.op_type {
        OperatorType::SUM => {
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count == *sub_packets_count {
                        // let sum: u128 = op_status.value_vec.iter().sum::<u128>() as u128;
                        let sum: u128 = op_status.value_vec.iter()
                            .fold(0, |acc, &value| {
                                debug!("[Literal Value] OperatorType::SUM -- {}", value);
                                acc + value
                            });
                        op_status.set_final_value(sum);
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len == *sub_packets_len {
                        // let sum: u128 = op_status.value_vec.iter().sum::<u128>() as u128;
                        let sum: u128 = op_status.value_vec.iter()
                            .fold(0, |acc, &value| {
                                debug!("[Literal Value] OperatorType::SUM -- {}", value);
                                acc + value
                            });
                        op_status.set_final_value(sum);
                    }
                }
            }
        },
        OperatorType::PRODUCT => {
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count == *sub_packets_count {
                        let product: u128 = op_status.value_vec.iter()
                            .fold(1, |acc, &value| {
                                debug!("[Literal Value] OperatorType::PRODUCT -- {}", value);
                                acc * value
                            });
                        op_status.set_final_value(product);

                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len == *sub_packets_len {
                        let product: u128 = op_status.value_vec.iter()
                            .fold(1, |acc, &value| {
                                debug!("[Literal Value] OperatorType::PRODUCT -- {}", value);
                                acc * value
                            });
                        op_status.set_final_value(product);
                    }
                }
            }
        },
        OperatorType::MINIMUM => {
            match &op_status.type_len_value {
                TypeLenValue::L11 { str_value: _, sub_packets_count} => {
                    if op_status.consumed_count == *sub_packets_count {
                        let min = op_status.value_vec.iter().min();
                        match min {
                            Some(min_value) => {
                                op_status.set_final_value(*min_value as u128);
                            },
                            None => {
                                warn!("[Literal Value] TypeLenValue - None");
                            }
                        }

                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len == *sub_packets_len {
                        let min = op_status.value_vec.iter().min();
                        match min {
                            Some(min_value) => {
                                op_status.set_final_value(*min_value as u128);
                            },
                            None => {
                                warn!("[Literal Value] TypeLenValue - None");
                            }
                        }
                    }
                }
            }
        },
        OperatorType::MAXIMUM => {
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count == *sub_packets_count {
                        let max = op_status.value_vec.iter().max();
                        match max {
                            Some(max_value) => {
                                op_status.set_final_value(*max_value as u128);
                            },
                            None => {
                                warn!("[Literal Value] TypeLenValue - None");
                            }
                        }

                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len == *sub_packets_len {
                        let max = op_status.value_vec.iter().max();
                        match max {
                            Some(max_value) => {
                                op_status.set_final_value(*max_value as u128);
                            },
                            None => {
                                warn!("[Literal Value] TypeLenValue - None");
                            }
                        }
                    }
                }
            }
        },
        OperatorType::GT => {
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count == *sub_packets_count {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 > v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len == *sub_packets_len {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 > v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                }
            }
        },
        OperatorType::LT => {
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count == *sub_packets_count {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 < v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len == *sub_packets_len {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 < v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                }
            }
        },
        OperatorType::EQ => {
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count == *sub_packets_count {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 == v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len == *sub_packets_len {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 == v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                }
            }
        },
        _ => {
            info!("[COMPUTE DATA VALUE] TYPE_ID: - Unknown ({:?}) ----", op_status.op_type);
        }
    }
}

fn set_packet_status_op_type(input_slice: &str, new_pos: &usize, type_id: u32, op_status: &mut OperatorPktStatus) {
    let new_pos = *new_pos;
    match type_id {
        0 => {
            info!("      TYPE_ID = {}, |{}| => [SUM], start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::SUM);
        },
        1 => {
            info!("      TYPE_ID = {}, |{}| => [PRODUCT], start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::PRODUCT);
        },
        2 => {
            info!("      TYPE_ID = {}, |{}| => [MIN], start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::MINIMUM);
        },
        3 => {
            info!("      TYPE_ID = {}, |{}| => [MAX], start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::MAXIMUM);
        },
        4 => {
            info!("      TYPE_ID = |{}| -- DATA PACKET --, start_pos = ({})", &input_slice[new_pos+3..new_pos+6], new_pos+3);
        },
        5 => {
            info!("      TYPE_ID = {}, |{}| => [GT], start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::GT);
        },
        6 => {
            info!("      TYPE_ID = {}, |{}| => [LT], start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::LT);
        },
        7 => {
            info!("      TYPE_ID = {}, |{}| => [EQ], start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::EQ);
        },
        _ => {
            info!("      TYPE_ID = {}, |{}| => UNKNOWN, start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
            op_status.set_op_type(OperatorType::UKN);
        }
    }
    op_status.set_start_input_pos(new_pos);
}

fn get_int_number(substr: &str) -> u32 {
    let num = substr.chars().rev().enumerate()
        .fold(0,|acc, (i, c)| acc + c.to_digit(2).unwrap() * u32::pow(2, i as u32));
    num
}

fn get_u64_number(substr: &str) -> u64 {
    let num = substr.chars().rev().enumerate()
        .fold(0,|acc, (i, c)| acc + c.to_digit(2).unwrap() as u64 * u64::pow(2, i as u32));
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
    let type_len_flag_str = &input_slice[start_pos..start_pos+1];
    let type_len_flag = get_int_number(type_len_flag_str);
    // debug!("  type_len_flag = {}, type_len_flag_str = {}", type_len_flag, type_len_flag_str);
    type_len_flag
}

fn get_subpacket_length_bits(input_slice: &str) -> u32 {
    //-- input_slice : 15 bits (total length of sub-packets in bits)
    let subpacket_len_bits = get_int_number(input_slice);
    // debug!("       sub-packets LEN (bits) = {}, |{}|(=15)", subpacket_len_bits, input_slice);
    subpacket_len_bits
}

fn get_subpacket_count(input_slice: &str) -> u32 {
    //-- input_slice: 11 bits (# of sub-packets)
    let subpacket_count = get_int_number(input_slice);
    // debug!("       sub-packets COUNT = {}, |{}|(=11)", subpacket_count, input_slice);
    subpacket_count
}