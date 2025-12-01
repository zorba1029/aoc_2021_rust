use std::cell::RefCell;
use std::rc::Rc;
// use log::debug;

use crate::advent::day18a::tokenizer::Token;

pub(crate) type TreeNodePtr = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
pub(crate) struct TreeNode {
    pub(crate) value: Option<i32>,
    pub(crate) left_child: Option<TreeNodePtr>,
    pub(crate) right_child: Option<TreeNodePtr>,
}

impl TreeNode {
    pub(crate) fn new(value: Option<i32>) -> TreeNodePtr {
        Rc::new(RefCell::new(TreeNode {
            value,
            left_child: None,
            right_child: None,
        }))
    }

    // add: merge two trees into a new tree
    pub(crate) fn add(left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
        // debug!("add(): left = {}, right = {}", tree_to_list(&left), tree_to_list(&right));
        let new_root = TreeNode::new(None);
        new_root.borrow_mut().left_child = Some(left);
        new_root.borrow_mut().right_child = Some(right);
        new_root.borrow_mut().reduce();
        new_root
    }

    // reduce: reduce the tree by exploding and splitting operations.
    fn reduce(&mut self) {
        loop {
            if self.explode(0).0 {
                continue;
            }
            if !self.split() {
                // if no more splits, break the loop
                break;
            }
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<i32>, Option<i32>) {
        // 1. leaf node case
        if let Some(_value) = self.value {
            return (false, None, None);
        }

        // 2.internal node case
        let left_child = self.left_child.as_ref().unwrap();
        let right_child = self.right_child.as_ref().unwrap();

        // 2-1. node depth >= 4 case
        if depth >= 4 {
            let left_value = left_child.borrow().value;
            let right_value = right_child.borrow().value;

            // left child and right child have values - candidate for explode operation
            if let (Some(l), Some(r)) = (left_value, right_value) {
                self.value = Some(0);
                self.left_child = None;
                self.right_child = None;

                // return to parent node with the values of the left and right children
                return (true, Some(l), Some(r))
            }
        }

        // 2-2. node depth < 4 case
        // 2-2-1. left child case
        if let (true, l, r) = left_child.borrow_mut().explode(depth + 1) {
            if let Some(r) = r {
                // add the value of the right child to the leftmost leaf node of the right subtree
                right_child.borrow_mut().add_leftmost(r);
            }
            return (true, l, None);
        }

        // 2-2-2. right child case
        if let (true, l, r) = right_child.borrow_mut().explode(depth + 1) {
            if let Some(l) = l {
                // add the value of the left child to the rightmost leaf node of the left subtree
                left_child.borrow_mut().add_rightmost(l);
            }
            return (true, None, r);
        }

        // 3. no explode operation case
        (false, None, None)
    }

    // here, 
    // - self: is the right child node of the parent node
    // - value: is the value of the left child node which is exploded
    fn add_leftmost(&mut self, value: i32) {
        if let Some(v) = self.value.as_mut() {
            *v += value;
        } else if let Some(ref left) = self.left_child {
            left.borrow_mut().add_leftmost(value);
        }
    }

    // here, 
    // - self: is the left child node of the parent node
    // - value: is the value of the right child node which is exploded
    fn add_rightmost(&mut self, value: i32) {
        if let Some(v) = self.value.as_mut() {
            *v += value;
        } else if let Some(ref right) = self.right_child {
            right.borrow_mut().add_rightmost(value);
        }
    }

    fn split(&mut self) -> bool {
        if let Some(value) = self.value {
            if value >= 10 {
                self.left_child = Some(TreeNode::new(Some(value / 2)));
                self.right_child = Some(TreeNode::new(Some((value + 1) / 2)));
                self.value = None;
                return true;
            }
            return false;
        }

        if let Some(ref left) = self.left_child {
            if left.borrow_mut().split() {
                return true;
            }
        }

        if let Some(ref right) = self.right_child {
            if right.borrow_mut().split() {
                return true;
            }
        }

        false
    }

    pub(crate) fn magnitude(&self) -> i32 {
        if let Some(value) = self.value {
            return value;
        }

        let left_mag = self.left_child.as_ref()
            .map(|left| left.borrow().magnitude())
            .unwrap_or(0);
        let right_mag = self.right_child.as_ref()
            .map(|right| right.borrow().magnitude())
            .unwrap_or(0);

        return 3 * left_mag + 2 * right_mag;
    }

    // convert the tree to a list string.
    fn to_list_string(&self) -> String {
        if let Some(value) = self.value {
            value.to_string()
        } else {
            let left_str = self
                .left_child
                .as_ref()
                .map(|left| left.borrow().to_list_string())
                .expect("Missing left child when converting tree to list.");
            let right_str = self
                .right_child
                .as_ref()
                .map(|right| right.borrow().to_list_string())
                .expect("Missing right child when converting tree to list.");
            format!("[{},{}]", left_str, right_str)
        }
    }
}

pub(crate) fn parse_tokens(tokens: &[Token], index: &mut usize) -> Option<TreeNodePtr> {
    if *index >= tokens.len() {
        return None;
    }

    match &tokens[*index] {
        Token::OpenBracket => {
            *index += 1;
            let left = parse_tokens(tokens, index);
            if *index < tokens.len() && matches!(tokens[*index], Token::Comma) {
                *index += 1;
            }
            let right = parse_tokens(tokens, index);
            if *index < tokens.len() && matches!(tokens[*index], Token::CloseBracket) {
                *index += 1;
            }
            let node = TreeNode::new(None);
            node.borrow_mut().left_child = left;
            node.borrow_mut().right_child = right;
            Some(node)
        }
        Token::Number(n) => {
            *index += 1;
            Some(TreeNode::new(Some(*n)))
        }
        _ => None,
    }
}

pub(crate) fn tree_to_list(node: &TreeNodePtr) -> String {
    node.borrow().to_list_string()
}