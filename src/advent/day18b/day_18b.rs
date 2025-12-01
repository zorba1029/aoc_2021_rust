use std::cell::{RefCell};
use std::rc::{Rc, Weak};
use log::info;

#[derive(Debug)]
enum Value {
    Regular(i32),
    Pair(NodePtr, NodePtr),
}

type NodePtr = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    value: Value,
    parent: Weak<RefCell<Node>>,
}

impl Node {
    fn new_regular(v: i32) -> NodePtr {
        Rc::new(RefCell::new(Node {
            value: Value::Regular(v),
            parent: Weak::new(),
        }))
    }

    fn new_pair(left: NodePtr, right: NodePtr) -> NodePtr {
        let node = Rc::new(RefCell::new(Node {
            value: Value::Pair(left.clone(), right.clone()),
            parent: Weak::new(),
        }));
        left.borrow_mut().parent = Rc::downgrade(&node);
        right.borrow_mut().parent = Rc::downgrade(&node);
        node
    }
}

// --------------------------- Tokenizer ---------------------------------

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    OpenBracket,
    Comma,
    CloseBracket,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            // '[' - Start of a pair
            '[' => {
                tokens.push(Token::OpenBracket);
                chars.next();
            }
            // '0'-'9' - A regular number (can be multi-digit)
            '0'..='9' => {
                let mut number = 0;
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_digit() {
                        number = number * 10 + next_c.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            }
            // ',' - Comma separator
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            // ']' - End of a pair
            ']' => {
                tokens.push(Token::CloseBracket);
                chars.next();
            }
            // ' ' - Whitespace: skip
            ' ' => {
                chars.next();
            }
            // Any other character: skip
            _ => {
                chars.next();
            }
        }
    }

    tokens
}

// --------------------------- Parsing ---------------------------------

fn parse_tokens(tokens: &[Token], index: &mut usize) -> NodePtr {
    if *index >= tokens.len() {
        panic!("Unexpected end of tokens");
    }

    match &tokens[*index] {
        Token::Number(n) => {
            *index += 1;
            Node::new_regular(*n)
        }
        Token::OpenBracket => {
            *index += 1; // consume '['
            let left = parse_tokens(tokens, index);
            
            // expect ','
            if *index >= tokens.len() || !matches!(tokens[*index], Token::Comma) {
                panic!("Expected ',' in pair");
            }
            *index += 1; // consume ','
            
            let right = parse_tokens(tokens, index);
            
            // expect ']'
            if *index >= tokens.len() || !matches!(tokens[*index], Token::CloseBracket) {
                panic!("Expected ']' to close pair");
            }
            *index += 1; // consume ']'
            
            Node::new_pair(left, right)
        }
        _ => panic!("Unexpected token: {:?}", tokens[*index]),
    }
}

fn parse_snailfish(s: &str) -> NodePtr {
    let tokens = tokenize(s);
    parse_tokens(&tokens, &mut 0)
}

// ---------------------- Neighbor Finding -------------------------------

fn find_left_neighbor(node: NodePtr) -> Option<NodePtr> {
    let mut cur = node;
    loop {
        let parent = cur.borrow().parent.upgrade()?;
        if let Value::Pair(l, r) = &parent.borrow().value {
            if Rc::ptr_eq(&r, &cur) {
                let mut n = l.clone();
                loop {
                    let next = {
                        let borrowed = n.borrow();
                        match &borrowed.value {
                            Value::Regular(_) => return Some(n.clone()),
                            Value::Pair(_, r) => r.clone(),
                        }
                    };
                    n = next;
                }
            }
        }
        cur = parent;
    }
}

fn find_right_neighbor(node: NodePtr) -> Option<NodePtr> {
    let mut cur = node;
    loop {
        let parent = cur.borrow().parent.upgrade()?;
        if let Value::Pair(l, r) = &parent.borrow().value {
            if Rc::ptr_eq(&l, &cur) {
                let mut n = r.clone();
                loop {
                    let next = {
                        let borrowed = n.borrow();
                        match &borrowed.value {
                            Value::Regular(_) => return Some(n.clone()),
                            Value::Pair(l, _) => l.clone(),
                        }
                    };
                    n = next;
                }
            }
        }
        cur = parent;
    }
}

// --------------------------- Explode ---------------------------------

fn explode(node: NodePtr, depth: i32) -> bool {
    let is_pair_of_regulars = match &node.borrow().value {
        Value::Pair(l, r) => {
            matches!(l.borrow().value, Value::Regular(_))
            && matches!(r.borrow().value, Value::Regular(_))
        }
        _ => false,
    };

    if depth >= 4 && is_pair_of_regulars {
        let (l, r) = match &node.borrow().value {
            Value::Pair(l, r) => (l.clone(), r.clone()),
            _ => unreachable!(),
        };

        // Get values first before any mutable borrow
        let l_val = match l.borrow().value {
            Value::Regular(v) => v,
            _ => 0,
        };
        let r_val = match r.borrow().value {
            Value::Regular(v) => v,
            _ => 0,
        };

        if let Some(ln) = find_left_neighbor(node.clone()) {
            let current_val = match ln.borrow().value {
                Value::Regular(v) => v,
                _ => 0,
            };
            ln.borrow_mut().value = Value::Regular(current_val + l_val);
        }

        if let Some(rn) = find_right_neighbor(node.clone()) {
            let current_val = match rn.borrow().value {
                Value::Regular(v) => v,
                _ => 0,
            };
            rn.borrow_mut().value = Value::Regular(current_val + r_val);
        }

        node.borrow_mut().value = Value::Regular(0);
        return true;
    }

    if let Value::Pair(l, r) = &node.borrow().value {
        if explode(l.clone(), depth + 1) { return true; }
        if explode(r.clone(), depth + 1) { return true; }
    }
    false
}

// --------------------------- Split -----------------------------------

fn split(node: NodePtr) -> bool {
    // First, check what kind of value we have and extract needed info
    let action = {
        let borrowed = node.borrow();
        match &borrowed.value {
            Value::Regular(v) if *v >= 10 => Some(*v),
            Value::Pair(l, r) => {
                let l = l.clone();
                let r = r.clone();
                drop(borrowed);
                if split(l) { return true; }
                if split(r) { return true; }
                return false;
            }
            _ => None,
        }
    };

    // Now perform split if needed (borrow is dropped)
    if let Some(v) = action {
        let left = Node::new_regular(v / 2);
        let right = Node::new_regular((v + 1) / 2);
        node.borrow_mut().value = Value::Pair(left.clone(), right.clone());
        left.borrow_mut().parent = Rc::downgrade(&node);
        right.borrow_mut().parent = Rc::downgrade(&node);
        return true;
    }

    false
}

// --------------------------- Reduce ---------------------------------

fn reduce(node: NodePtr) {
    loop {
        if explode(node.clone(), 0) { continue; }
        if split(node.clone()) { continue; }
        break;
    }
}

// ----------------------------- Add -----------------------------------

fn add(a: NodePtr, b: NodePtr) -> NodePtr {
    let root = Node::new_pair(a, b);
    reduce(root.clone());
    root
}

// ---------------------------- Magnitude -------------------------------

fn magnitude(node: NodePtr) -> i32 {
    match &node.borrow().value {
        Value::Regular(v) => *v,
        Value::Pair(l, r) => {
            let left_mag = magnitude(l.clone());
            let right_mag = magnitude(r.clone());
            3 * left_mag + 2 * right_mag
        }
    }
}

// ---------------------------- Clone Tree -----------------------------

fn clone_tree(node: NodePtr) -> NodePtr {
    match &node.borrow().value {
        Value::Regular(v) => Node::new_regular(*v),
        Value::Pair(l, r) => {
            Node::new_pair(clone_tree(l.clone()), clone_tree(r.clone()))
        }
    }
}

// --------------------------- Part 1 ----------------------------------

fn part1(nums: &[NodePtr]) -> i32 {
    let mut acc = clone_tree(nums[0].clone());
    for i in 1..nums.len() {
        acc = add(acc, clone_tree(nums[i].clone()));
    }
    magnitude(acc)
}

// --------------------------- Part 2 ----------------------------------

fn part2(nums: &[NodePtr]) -> i32 {
    let mut best = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j { continue; }
            let sum = add(
                clone_tree(nums[i].clone()),
                clone_tree(nums[j].clone())
            );
            best = best.max(magnitude(sum));
        }
    }
    best
}

// ----------------------------- Main ----------------------------------

// fn main() {
//     let input = std::fs::read_to_string("input.txt").unwrap();
//     let nums: Vec<_> = input.lines()
//         .map(|l| parse_snailfish(l))
//         .collect();

//     println!("Part 1: {}", part1(&nums));
//     println!("Part 2: {}", part2(&nums));
// }

pub fn do_day_18b() {
    info!("===============================================");
    info!("--- Day 18: Snailfish, Part Two ---, Nov 30, 2025 ");
    info!("===============================================");
    // let filename = "input/day_18-sample-1.txt";
    // let filename = "input/day_18-sample-2.txt";
    let filename = "input/day_18-input.txt";
    let input = std::fs::read_to_string(filename).unwrap();
    let nums: Vec<Rc<RefCell<Node>>> = input.lines().map(|l| parse_snailfish(l)).collect();

    info!("Part 1: {}", part1(&nums));
    info!("Part 2: {}", part2(&nums));
}
