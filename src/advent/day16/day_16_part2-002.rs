// advent/day_16.rs
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
    TBD,
    SUM,
    PRODUCT,
    MINIMUM,
    MAXIMUM,
    GT,
    LT,
    EQ,
    UKN,
 }

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct OperatorPktStatus {
    pub op_type: OperatorType,
    pub start_input_pos: usize,
    pub last_input_pos: usize,
    pub type_len_id: TypeLenID,
    pub type_len_value: TypeLenValue,
    // type_len_15: TypeLenValue,
    pub consumed_len: u32,
    pub consumed_count: u32,
    pub value_vec: Vec<u128>,
    pub final_result: u128,
    pub finalized: bool,
 }

 impl OperatorPktStatus {
    pub fn new() -> Self {
        OperatorPktStatus {
            op_type: OperatorType::TBD,
            start_input_pos: 0,
            last_input_pos: 0,
            type_len_id: TypeLenID::L11,
            type_len_value: TypeLenValue::L11 {
                str_value: "".to_string(),
                sub_packets_count: 0,
            },
            //    type_len_15: TypeLenValue::L15 {
            //     str_value: "".to_string(),
            //     sub_packets_len: 0,
            //   },
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

pub fn day_16_part_two() {
    info!("===============================================");
    info!("--- Day 16: Packet Decoder, Part Two ---, ");
    info!("===============================================");
    // let filename = "input/day_16-test-01.txt";
    // let filename = "input/day_16-test-02.txt";
    // let filename = "input/day_16-test-03.txt";
    // let filename = "input/day_16-sample-01.txt";
        // len = 18, data=[8A004A801A8002F478]
        // len = 72, data = "100010100000000001001010100000000001101010000000000000101111010001111000"
    // let filename = "input/day_16-sample-02.txt";
        // len=26, data=[620080001611562C8802118E34]
        // len = 104, data = "01100010000000001000000000000000000101100001000101010110001011001000100000000010000100011000111000110100"
    // let filename = "input/day_16-sample-03.txt";
    // let filename = "input/day_16-sample-04.txt";
    //--- part 2: input samples
    // let filename = "input/day_16-sample-11.txt";    // len=10, data=[C200B40A82]
    // let filename = "input/day_16-sample-12.txt";
    // let filename = "input/day_16-sample-13.txt";
    // let filename = "input/day_16-sample-14.txt";
    // let filename = "input/day_16-sample-15.txt";
    // let filename = "input/day_16-sample-16.txt";
    // let filename = "input/day_16-sample-17.txt";
    // let filename = "input/day_16-sample-18.txt";
    // let filename = "input/day_16-input-small.txt";
    let filename = "input/day_16-input.txt";
        //-- packet_count = 269, op = 100, literal = 169,
    let input_line = handle_input(filename);
    let input_len = input_line.len();
    info!("input_line(binary format): len = {}, data = {:?}", input_len, input_line);

    let version_sum = parse_packets(&input_line);
    info!("[**] version all items = {:?}", version_sum);
    info!("[**] version sum = {}", version_sum.iter().sum::<u32>());
}

fn parse_packets(input_line: &str) -> Vec<u32> {
    let input_slice = input_line;
    let mut version_vec = Vec::new();
    let mut new_pos = 0usize;
    // let mut op_status = OperatorPktStatus::new();
    let mut op_status_vec: Vec<OperatorPktStatus> = Vec::new();
    let mut packet_count = 0;
    let mut operator_count = 0;
    let mut literal_count = 0;

    loop {
        // info!("");
        // debug!("=======| start_pos = {}, (remained) input_len = {} |==============", new_pos, input_slice[new_pos..].len());
        // info!("[INPUT] = [{} {} {} ...] (20 chars) ", &input_slice[new_pos..new_pos+3],
        //                                             &input_slice[new_pos+3..new_pos+6],
        //                                             &input_slice[new_pos+6..new_pos+7],);
        let version = get_packet_version(input_slice, new_pos);
        let type_id = get_packet_type_id(input_slice, new_pos+3);
        let type_len_id = get_packet_type_length_id(input_slice, new_pos+6);
    
        version_vec.push(version);

        info!("");
        if type_id != 4{
            debug!("🍏[OPERATOR]🍏 ==| start_pos = {}, (remained) input_len = {} |==============", new_pos, input_slice[new_pos..].len());
        } else {
            debug!("🍒[DATA]🍒     --| start_pos = {}, (remained) input_len = {} |==============", new_pos, input_slice[new_pos..].len());
        }
        info!("[INPUT] = [{} {} {} ...] (20 chars) ", &input_slice[new_pos..new_pos+3],
                                                    &input_slice[new_pos+3..new_pos+6],
                                                    &input_slice[new_pos+6..new_pos+7],);
        debug!("[VVV] VERSION = {}, |{}|, start_pos = ({})", version, &input_slice[new_pos..new_pos+3], new_pos);
        debug!("[TTT]    TYPE = {}, |{}|, start_pos = ({})", type_id, &input_slice[new_pos+3..new_pos+6], new_pos+3);
       
        packet_count += 1;
        // type ID != 4, operator packets
        if type_id != 4 {  
            // OPERATOR/CONTROL packet
            operator_count += 1;
            let mut op_status = OperatorPktStatus::new();
            info!("[  ] ----------| OPERATOR ({}) |-----------", &input_slice[new_pos+3..new_pos+6]);
            set_packet_status_op_type(input_slice, &new_pos, type_id, &mut op_status);

            debug!("[I] Type LEN ID (1/0) = {}, |{}|, start_pos = ({})", type_len_id, &input_slice[new_pos+6..new_pos+7], new_pos+6);
            operator_packet_handler(input_slice, &mut new_pos, &mut version_vec, &mut op_status);
            debug!("[OP] OperatorStatus = {:#?}", op_status);

            op_status_vec.push(op_status);
            compute_stack_data(&mut op_status_vec);
        } else { 
            // DATA/LITERAL packet
            literal_count += 1;
            info!("[  ] ----------| LITERAL |{}| |------------", &input_slice[new_pos+3..new_pos+6]);
            let mut op_status = op_status_vec.pop().unwrap();

            let _consumed_len = literal_packet_handler(input_slice, &mut new_pos, &mut version_vec, &mut op_status);
            // debug!("     [*] LITERAL PACKET: consumed_len = {}", consumed_len);
            compute_literal_packets_value(&mut op_status);
            debug!("[DATA] OperatorStatus = {:#?}", op_status);

            op_status_vec.push(op_status);
            compute_stack_data(&mut op_status_vec);
        }
        
        if new_pos >= input_slice.len() {
            info!("[^--^] [All Inputs Handled], new_pos = {}, input_line.len() = {}", new_pos, input_slice.len());
            break;
        }

        //-- update new input_slice
        if input_slice[new_pos..].len() <= 6 {
            info!("[^--^] [All Inputs Handled] new_pos = {}, remained inputs = {:?}, input_line.len() = {}",
                    new_pos, &input_slice[new_pos..], input_slice.len());
            break;
        } else {
            // info!("[*] current input = {:?}", &input_slice[old_pos..new_pos]);
            info!("[*] next input = [{} {} ({})=I].....", &input_slice[new_pos..new_pos+3],
                                                        &input_slice[new_pos+3..new_pos+6],
                                                        &input_slice[new_pos+6..new_pos+7]);
        }
    }

    // compute_stack_data(&mut op_status_vec);
    compute_stack_final_data(&mut op_status_vec);
    info!("[***] op_status_vec = {:#?}", op_status_vec);
    info!("[ ] packet_count = {}, op = {}, literal = {},", packet_count, operator_count, literal_count);

    version_vec
}

fn operator_packet_handler(input_slice: &str,
                           new_pos: &mut usize,
                           _version_vec: &mut Vec<u32>,
                           op_status: &mut OperatorPktStatus) -> () {
    let new_start = *new_pos;
    let current_input = &input_slice;
    let _version = get_int_number(&current_input[new_start..new_start+3]);
    // let type_id = get_packet_type_id(input_slice, new_start+3);
    let type_len_id = get_int_number(&current_input[new_start+6..new_start+7]);
    // version_vec.push(version);

    // debug!("[VVV] operator packet: Version = {}", version);
    // debug!("[TTT] operator packet: Packet Type (ID) = {}", type_id);
    // debug!("  [I] operator packet: Type len ID = {}", type_len_id);
    debug!("[OP] new_start pos = {},", new_start);
    debug!("[OP] current input_len = {}, current_input = [{:?}]...",
                                        &current_input[new_start..].len(), &current_input[new_start..new_start+7]);

    match type_len_id {
        0 => {
            op_status.set_type_len_id(TypeLenID::L15);
            let t_start = new_start+7;
            let t_end = new_start+7+15;
            debug!("[L-15] (I=0) length: ({},{}) = {}", t_start, t_end, t_end-t_start);
            if t_start < current_input.len() && t_end < current_input.len() {
                let sub_packets_len = get_subpacket_length_bits(&current_input[t_start..t_end]);
                debug!("[L-15] (I=0: sub-packets len in bits = {}", sub_packets_len);
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
            // op_status.add_consumed_len(22);
        },
        1 => {
            op_status.set_type_len_id(TypeLenID::L11);
            let t_start = new_start+7;
            let t_end = new_start+7+11;
            debug!("[L-11] (I=1) count: ({},{}) = {}", t_start, t_end, t_end-t_start);
            if t_start < current_input.len() && t_end < current_input.len() {
                let sub_packets_count = get_subpacket_count(&current_input[t_start..t_end]);
                debug!("[L-11] (I=1): count of sub-packets = {}", sub_packets_count);
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
            // op_status.add_consumed_len(18);
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
    debug!("[DATA] new_start pos = {},", new_start);

    // let current_input = &input_slice;
    // let version = get_int_number(&current_input[new_start..new_start+3]);
    // version_vec.push(version);
    // debug!("   + DATA: Version = {} [*]", version);

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

    debug!("[DATA] Final Value = [{}]** (== |{}|(len={} bits))", literal_single_value, literal_concat, literal_concat.len());
    // debug!("[DATA] Single Value = {}", literal_single_value);
    debug!("[DATA] Packet LEN = ({})**", consumed_len);

    match op_status.type_len_id {
        TypeLenID::L11 => {
            op_status.add_value(literal_single_value as u128);
            op_status.add_consumed_count(1);
            op_status.add_consumed_len(consumed_len);
            debug!("[DATA] Consumed LEN = {} (bits), Consumed COUNT = {}", op_status.consumed_len, op_status.consumed_count);
        },
        TypeLenID::L15 => {
            op_status.add_value(literal_single_value as u128);
            op_status.add_consumed_len(consumed_len);
            op_status.add_consumed_count(1);
            debug!("[DATA] Consumed LEN = {} (bits), Consumed COUNT = {}", op_status.consumed_len, op_status.consumed_count);
        }
    }

    *new_pos = *new_pos + 6 + (loop_count * 5);
    consumed_len
}

fn compute_stack_data(op_status_vec: &mut Vec<OperatorPktStatus>) {
    debug!("🍏🍒[STACK]🍏🍒 ====>> Compute Stack Data ======");

    while !op_status_vec.is_empty() {
        let top_data = op_status_vec.pop();

        let top_data = match top_data {
            Some(data_value) => {
                if !data_value.is_finalized() {
                    op_status_vec.push(data_value);
                    debug!("[STACK] NO-OP |==> RETURN ");
                    return;
                }
                data_value
            },
            None => {
                warn!("[STACK] NO-OP|-- compute_stack_data() - data2 is not ready");
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

        let parent_data = op_status_vec.pop();
        let mut parent_data = match parent_data {
            Some(data_value) => {
                data_value
            },
            None => {
                warn!("[STACK] NO-OP |-- compute_stack_data() - result is not ready");
                op_status_vec.push(top_data);
                return;
            }
        };

        debug!("[STACK] TOP ITEM - {:#?}", top_data);
        debug!("[STACK] Parent ITEM - BEFORE - {:#?}", parent_data);

        match parent_data.type_len_value.clone() {
            TypeLenValue::L11 { str_value: _, sub_packets_count } => {
                match parent_data.op_type {
                    OperatorType::SUM => {
                        info!("[STACK]     |- OP TYPE ID: - SUM ({:?}) --------", parent_data.op_type);
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
                    OperatorType::PRODUCT => {
                        info!("[STACK]     |- OP TYPE ID: - PRODUCT ({:?}) ----", parent_data.op_type);
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
                    OperatorType::MINIMUM => {
                        info!("[STACK]     |- OP TYPE ID: - MIN ({:?})  -------", parent_data.op_type);
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
                    OperatorType::MAXIMUM => {
                        info!("[STACK]     |- OP TYPE ID: - MAX ({:?})  -------", parent_data.op_type);
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
                    OperatorType::GT => {
                        info!("[STACK]     |- OP TYPE ID: - GT ({:?})  --------", parent_data.op_type);
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
                    OperatorType::LT => {
                        info!("[STACK]     |- OP TYPE ID: - LT ({:?})  --------", parent_data.op_type);
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
                    OperatorType::EQ => {
                        info!("[STACK]     |- OP TYPE ID: - EQ ({:?})  --------", parent_data.op_type);
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
                    _ => {
                        info!("[STACK]     |- OP TYPE ID: - Unknown ({:?}) ----", parent_data.op_type);
                    }
                }
            },
            TypeLenValue::L15 { str_value: _, sub_packets_len } => {
                match parent_data.op_type {
                    OperatorType::SUM => {
                        info!("[STACK]     |- OP TYPE ID: - SUM ({:?}) --------", parent_data.op_type);
                        if parent_data.consumed_len < sub_packets_len {
                            parent_data.add_value(top_data.final_result);
                            parent_data.add_consumed_count(1);
                            parent_data.add_consumed_len(top_data.consumed_len + top_data_length);
                            if parent_data.consumed_len >= sub_packets_len {
                                let result = parent_data.value_vec.iter().sum();
                                parent_data.set_final_value(result);
                            }
                        }
                    },
                    OperatorType::PRODUCT => {
                        info!("[STACK]     |- OP TYPE ID: - PRODUCT ({:?}) ----", parent_data.op_type);

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
                    },
                    OperatorType::MINIMUM => {
                        info!("[STACK]     |- OP TYPE ID: - MIN ({:?})  -------", parent_data.op_type);
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
                    },
                    OperatorType::MAXIMUM => {
                        info!("[STACK]     |- OP TYPE ID: - MAX ({:?})  -------", parent_data.op_type);
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
                    },
                    OperatorType::GT => {
                        info!("[STACK]     |- OP TYPE ID: - GT ({:?})  --------", parent_data.op_type);
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
                    },
                    OperatorType::LT => {
                        info!("[STACK]     |- OP TYPE ID: - LT ({:?})  --------", parent_data.op_type);
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
                    },
                    OperatorType::EQ => {
                        info!("[STACK]     |- OP TYPE ID: - EQ ({:?})  --------", parent_data.op_type);
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
                    },
                    _ => {
                        info!("[STACK]     |- OP TYPE ID: - Unknown ({:?}) ----", parent_data.op_type);
                    }
                }
            }
        }
        debug!("[STACK] Parent ITEM - AFTER - {:#?}", parent_data);

        op_status_vec.push(parent_data);
    }
}

fn compute_stack_final_data(op_status_vec: &mut Vec<OperatorPktStatus>) {
    debug!("[*****] ====>> FINAL: Compute Stack Data [*****]");

    let last_data = op_status_vec.pop();
    let last_data = op_status_vec.pop();
    let mut parent_data = match last_data {
        Some(ref data_value) => {
            data_value
        },
        None => {
            warn!("     |- compute_stack_data() - result is not ready");
            return;
        }
    };

    // debug!("[Compute Stack Data] top_data - {:#?}", top_data);
    debug!("[Compute Stack Data] Parent_data BEFORE - {:#?}", parent_data);

    match parent_data.type_len_value {
        TypeLenValue::L11 { str_value: _, sub_packets_count } => {
            match parent_data.op_type {
                OperatorType::SUM => {
                    info!("     |- OP TYPE ID: - SUM ({:?}) --------", parent_data.op_type);
                    let mut result = 0u128;
                    parent_data.value_vec.iter().for_each(|value| {
                        result += value;
                    });
                    info!("     |- OP TYPE ID: - SUM ({:?}) --------", result);
                    // parent_data.set_final_value(result);
                },
                OperatorType::PRODUCT => {
                    info!("     |- OP TYPE ID: - PRODUCT ({:?}) ----", parent_data.op_type);

                },
                OperatorType::MINIMUM => {
                    info!("     |- OP TYPE ID: - MIN ({:?})  -------", parent_data.op_type);

                },
                OperatorType::MAXIMUM => {
                    info!("     |- OP TYPE ID: - MAX ({:?})  -------", parent_data.op_type);

                },
                OperatorType::GT => {
                    info!("     |- OP TYPE ID: - GT ({:?})  --------", parent_data.op_type);

                },
                OperatorType::LT => {
                    info!("     |- OP TYPE ID: - LT ({:?})  --------", parent_data.op_type);

                },
                OperatorType::EQ => {
                    info!("     |- OP TYPE ID: - EQ ({:?})  --------", parent_data.op_type);

                },
                _ => {
                    info!("     |- OP TYPE ID: - Unknown ({:?}) ----", parent_data.op_type);
                }
            }
        },
        TypeLenValue::L15 { str_value: _, sub_packets_len } => {
            match parent_data.op_type {
                OperatorType::SUM => {
                    info!("     |- OP TYPE ID: - SUM ({:?}) --------", parent_data.op_type);

                },
                OperatorType::PRODUCT => {
                    info!("     |- OP TYPE ID: - PRODUCT ({:?}) ----", parent_data.op_type);


                },
                OperatorType::MINIMUM => {
                    info!("     |- OP TYPE ID: - MIN ({:?})  -------", parent_data.op_type);

                },
                OperatorType::MAXIMUM => {
                    info!("     |- OP TYPE ID: - MAX ({:?})  -------", parent_data.op_type);

                },
                OperatorType::GT => {
                    info!("     |- OP TYPE ID: - GT ({:?})  --------", parent_data.op_type);

                },
                OperatorType::LT => {
                    info!("     |- OP TYPE ID: - LT ({:?})  --------", parent_data.op_type);

                },
                OperatorType::EQ => {
                    info!("     |- OP TYPE ID: - EQ ({:?})  --------", parent_data.op_type);

                },
                _ => {
                    info!("     |- OP TYPE ID: - Unknown ({:?}) ----", parent_data.op_type);
                }
            }
        }
    }
    debug!("[Compute Stack Data] Parent_data AFTER - {:#?}", parent_data);
}

// fn compute_stack_data(op_status_vec: &mut Vec<OperatorPktStatus>) {
//     debug!("[*****] ====>> Compute Stack Data ======");
//     while !op_status_vec.is_empty() {
//         let data1 = op_status_vec.pop();

//         let data1 = match data1 {
//             Some(data_value) => {
//                 if !data_value.is_finalized() {
//                     op_status_vec.push(data_value);
//                     return;
//                 }
//                 data_value
//             },
//             None => {
//                 warn!("     |- compute_stack_data() - data2 is not ready");
//                 return;
//             }
//         };

//         let data2 = op_status_vec.pop();
//         let data2 = match data2 {
//             Some(data_value) => {
//                 if !data_value.is_finalized() {
//                     op_status_vec.push(data_value);
//                     op_status_vec.push(data1);
//                     return;
//                 }
//                 data_value
//             },
//             None => {
//                 warn!("     |- compute_stack_data() - data2 is not ready");
//                 op_status_vec.push(data1);
//                 return;
//             }
//         };

//         let result = op_status_vec.pop();
//         let mut result = match result {
//             Some(data_value) => {
//                 // if !data_value.is_finalized() {
//                 //     op_status_vec.push(data_value);
//                 //     op_status_vec.push(data2);
//                 //     op_status_vec.push(data1);
//                 //     return;
//                 // }
//                 data_value
//             },
//             None => {
//                 warn!("     |- compute_stack_data() - result is not ready");
//                 op_status_vec.push(data2);
//                 op_status_vec.push(data1);
//                 return;
//             }
//         };

//         match result.op_type {
//             OperatorType::SUM => {
//                 info!("     |- OP TYPE ID: - SUM ({:?}) --------", result.op_type);
//                 let value = data1.final_result + data2.final_result;
//                 result.set_final_value(value);
//             },
//             OperatorType::PRODUCT => {
//                 info!("     |- OP TYPE ID: - PRODUCT ({:?}) ----", result.op_type);
//                 info!("     |- OP TYPE ID: - data1.final_result = {}, data2.final_result = {} ----",
//                         data1.final_result, data2.final_result);
//                 let value: u128 = data1.final_result * data2.final_result;
//                 result.set_final_value(value);
//             },
//             OperatorType::MINIMUM => {
//                 info!("     |- OP TYPE ID: - MIN ({:?})  -------", result.op_type);
//                 let value = u128::min(data1.final_result, data2.final_result);
//                 result.set_final_value(value);
//             },
//             OperatorType::MAXIMUM => {
//                 info!("     |- OP TYPE ID: - MAX ({:?})  -------", result.op_type);
//                 let value = u128::max(data1.final_result, data2.final_result);
//                 result.set_final_value(value);
//             },
//             OperatorType::GT => {
//                 info!("     |- OP TYPE ID: - GT ({:?})  --------", result.op_type);
//                 let v1 = data1.final_result;
//                 let v2 = data2.final_result;
//                 if v1 >= v2 {
//                     result.set_final_value(1);
//                 } else {
//                     result.set_final_value(0);
//                 }
//             },
//             OperatorType::LT => {
//                 info!("     |- OP TYPE ID: - LT ({:?})  --------", result.op_type);
//                 let v1 = data1.final_result;
//                 let v2 = data2.final_result;
//                 if v1 <= v2 {
//                     result.set_final_value(1);
//                 } else {
//                     result.set_final_value(0);
//                 }
//             },
//             OperatorType::EQ => {
//                 info!("     |- OP TYPE ID: - EQ ({:?})  --------", result.op_type);
//                 let v1 = data1.final_result;
//                 let v2 = data2.final_result;
//                 if v1 == v2 {
//                     result.set_final_value(1);
//                 } else {
//                     result.set_final_value(0);
//                 }
//             },
//             _ => {
//                 info!("     |- OP TYPE ID: - Unknown ({:?}) ----", result.op_type);
//             }
//         }

//         op_status_vec.push(result);
//     }
// }

fn compute_literal_packets_value(op_status: &mut OperatorPktStatus) {
    match op_status.op_type {
        OperatorType::SUM => {
            info!("     |- OP TYPE ID: - SUM ({:?}) --------", op_status.op_type);
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count >= *sub_packets_count {
                        // let sum: u128 = op_status.value_vec.iter().sum() as u128;
                        let mut sum: u128 = 0;
                        op_status.value_vec.iter().for_each(|v| {
                            debug!(" SUMMATION -- {}", v);
                            sum += *v as u128;
                        });
                        op_status.set_final_value(sum);
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len >= *sub_packets_len {
                        // let sum: u128 = op_status.value_vec.iter().sum() as u128;
                        let mut sum = 0;
                        op_status.value_vec.iter().for_each(|v| {
                            debug!(" SUMMATION -- {}", v);
                            sum += *v as u128;
                        });
                        op_status.set_final_value(sum);
                    }
                }
            }
        },
        OperatorType::PRODUCT => {
            info!("     |- OP TYPE ID: - PRODUCT ({:?}) ----", op_status.op_type);
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count >= *sub_packets_count {
                        let mut sum: u128 = 1;
                        op_status.value_vec.iter().for_each(|value| {
                            debug!(" OperatorType::PRODUCT -- {}", value);
                            sum = sum * *value as u128;
                        });
                        op_status.set_final_value(sum);
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len >= *sub_packets_len {
                        let mut sum: u128 = 1;
                        op_status.value_vec.iter().for_each(|value| {
                            debug!(" OperatorType::PRODUCT -- {}", value);
                            sum = sum * *value as u128;
                        });
                        op_status.set_final_value(sum);
                    }
                }
            }
        },
        OperatorType::MINIMUM => {
            info!("     |- OP TYPE ID: - MIN ({:?})  -------", op_status.op_type);
            match &op_status.type_len_value {
                TypeLenValue::L11 { str_value: _, sub_packets_count} => {
                    if op_status.consumed_count >= *sub_packets_count {
                        let min = op_status.value_vec.iter().min();
                        match min {
                            Some(min_value) => {
                                op_status.set_final_value(*min_value as u128);
                            },
                            None => {
                                warn!("    TypeLenValue - None");
                            }
                        }
                        
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len >= *sub_packets_len {
                        let min = op_status.value_vec.iter().min();
                        match min {
                            Some(min_value) => {
                                op_status.set_final_value(*min_value as u128);
                            },
                            None => {
                                warn!("    TypeLenValue - None");
                            }
                        }
                    }
                }
            }
        },
        OperatorType::MAXIMUM => {
            info!("     |- OP TYPE ID: - MAX ({:?})  -------", op_status.op_type);
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count >= *sub_packets_count {
                        let max = op_status.value_vec.iter().max();
                        match max {
                            Some(max_value) => {
                                op_status.set_final_value(*max_value as u128);
                            },
                            None => {
                                warn!("    TypeLenValue - None");
                            }
                        }
                        
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len >= *sub_packets_len {
                        let max = op_status.value_vec.iter().max();
                        match max {
                            Some(max_value) => {
                                op_status.set_final_value(*max_value as u128);
                            },
                            None => {
                                warn!("    TypeLenValue - None");
                            }
                        }
                    }
                }
            }
        },
        OperatorType::GT => {
            info!("     |- OP TYPE ID: - GT ({:?})  --------", op_status.op_type);
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count >= *sub_packets_count {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 >= v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len >= *sub_packets_len {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 >= v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                }
            }
        }, 
        OperatorType::LT => {
            info!("     |- OP TYPE ID: - LT ({:?})  --------", op_status.op_type);
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count >= *sub_packets_count {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 <= v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                },
                TypeLenValue::L15{ str_value: _, sub_packets_len } => {
                    if op_status.consumed_len >= *sub_packets_len {
                        let v1 = *op_status.value_vec.first().unwrap();
                        let v2 = *op_status.value_vec.get(1).unwrap();
                        if v1 <= v2 {
                            op_status.set_final_value(1);
                        } else {
                            op_status.set_final_value(0);
                        }
                    }
                }
            }
        },
        OperatorType::EQ => {
            info!("     |- OP TYPE ID: - EQ ({:?})  --------", op_status.op_type);
            match &op_status.type_len_value {
                TypeLenValue::L11{ str_value: _, sub_packets_count} => {
                    if op_status.consumed_count >= *sub_packets_count {
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
                    if op_status.consumed_len >= *sub_packets_len {
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
            info!("     |- OP TYPE ID: - Unknown ({:?}) ----", op_status.op_type);
        }
    }
}

fn set_packet_status_op_type(input_slice: &str, new_pos: &usize, type_id: u32, op_status: &mut OperatorPktStatus) {
    let new_pos = *new_pos;
    match type_id {
        0 => {
            info!("[OP] TYPE ID: 🍏🍒 SUM 🍏🍒 |{}| ({}) ", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::SUM);
        },
        1 => {
            info!("[OP] TYPE ID: 🍏🍒 PRODUCT 🍏🍒 |{}| ({}) ", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::PRODUCT);
        },
        2 => {
            info!("[OP] TYPE ID: 🍏🍒 MIN 🍏🍒 |{}| ({}) ", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::MINIMUM);
        },
        3 => {
            info!("[OP] TYPE ID: 🍏🍒 MAX 🍏🍒 |{}| ({}) ", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::MAXIMUM);
        },
        4 => {
            info!("[OP] TYPE ID: 🍏🍒 -- DATA PACKET -- 🍏🍒 |{}| ({}) - GT --------", &input_slice[new_pos+3..new_pos+6], type_id);
        },
        5 => {
            info!("[OP] TYPE ID: 🍏🍒 GT 🍏🍒 |{}| ({}) ", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::GT);
        }, 
        6 => {
            info!("[OP] TYPE ID: 🍏🍒 LT 🍏🍒 |{}| ({}) ", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::LT);
        },
        7 => {
            info!("[OP] TYPE ID: 🍏🍒 EQ 🍏🍒 |{}| ({}) ", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::EQ);
        },
        _ => {
            info!("[OP] TYPE ID: 🍏🍒 Unknown 🍏🍒 |{}| ({}) -----", &input_slice[new_pos+3..new_pos+6], type_id);
            op_status.set_op_type(OperatorType::UKN);
        }
    }
    op_status.set_start_input_pos(new_pos);
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

fn get_u64_number(substr: &str) -> u64 {
    let mut num: u64 = 0;
    substr.chars().rev().enumerate().for_each(|(i, c)| {
        // num += u32::pow(c.to_digit(2).unwrap() as u32, i as u32);
        num += (c.to_digit(2).unwrap() as u64 * u64::pow(2, i as u32)) as u64;
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
    let type_len_flag_str = &input_slice[start_pos..start_pos+1];
    let type_len_flag = get_int_number(type_len_flag_str);
    // debug!("  type_len_flag = {}, type_len_flag_str = {}", type_len_flag, type_len_flag_str);

    type_len_flag
}

fn get_subpacket_length_bits(input_slice: &str) -> u32 {
    //-- input_slice : 15 bits (total length of sub-packets in bits)
    let subpacket_len_bits = get_int_number(input_slice);
    debug!("    + sub-packets LEN (bits) = {}, |{}|(=15)", subpacket_len_bits, input_slice);

    subpacket_len_bits
}

fn get_subpacket_count(input_slice: &str) -> u32 {
    //-- input_slice: 11 bits (# of sub-packets)
    let subpacket_count = get_int_number(input_slice);
    debug!("    + sub-packets COUNT = {}, |{}|(=11)", subpacket_count, input_slice);
    
    subpacket_count
}