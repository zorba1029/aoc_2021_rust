use std::cell::RefCell;
use std::rc::Rc;

use crate::advent::day18::tokenizer::Token;

pub(crate) type TreeNodePtr = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
pub(crate) struct TreeNode {
    pub(crate) value: Option<i32>,
    pub(crate) left_child: Option<TreeNodePtr>,
    pub(crate) right_child: Option<TreeNodePtr>,
}

pub(crate) fn parse_tokens(tokens: &[Token], index: &mut usize) -> Option<TreeNodePtr> {
    if *index >= tokens.len() {
        return None;
    }

    match &tokens[*index] {
        Token::Number(n) => {
            *index += 1;
            Some(TreeNode::new(Some(*n)))
        }
        Token::OpenBracket => {
            *index += 1; // '['를 건너뜀
            let left = parse_tokens(tokens, index);
            if *index < tokens.len() && matches!(tokens[*index], Token::Comma) {
                *index += 1; // ','를 건너뜀
            }
            let right = parse_tokens(tokens, index);
            if *index < tokens.len() && matches!(tokens[*index], Token::CloseBracket) {
                *index += 1; // ']'를 건너뜀
            }
            let node = TreeNode::new(None);
            node.borrow_mut().left_child = left;
            node.borrow_mut().right_child = right;
            Some(node)
        }
        _ => None,
    }
}

impl TreeNode {
    pub(crate) fn new(value: Option<i32>) -> TreeNodePtr {
        Rc::new(RefCell::new(TreeNode {
            value,
            left_child: None,
            right_child: None,
        }))
    }

    pub(crate) fn merge(left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
        let new_root = TreeNode::new(None); // 변경: 'mut' 제거
        new_root.borrow_mut().left_child = Some(left);
        new_root.borrow_mut().right_child = Some(right);
        new_root.borrow_mut().reduce();
        new_root
    }

    pub(crate) fn merge_option(
        left: Option<TreeNodePtr>,
        right: Option<TreeNodePtr>,
    ) -> Option<TreeNodePtr> {
        let new_root = TreeNode::new(None);
        new_root.borrow_mut().left_child = left; // left의 소유권 이동
        new_root.borrow_mut().right_child = right; // right의 소유권 이동
        Some(new_root)
    }

    pub(crate) fn in_order_traversal(&self, visit: &mut dyn FnMut(i32)) {
        if let Some(ref left) = self.left_child {
            left.borrow().in_order_traversal(visit);
        }

        if let Some(value) = self.value {
            visit(value);
        }

        if let Some(ref right) = self.right_child {
            right.borrow().in_order_traversal(visit);
        }
    }

    pub(crate) fn left_order_traversal(&self, visit: &mut dyn FnMut(i32)) {
        if let Some(ref left) = self.left_child {
            left.borrow().left_order_traversal(visit);
        }

        if let Some(value) = self.value {
            visit(value);
        }

        if let Some(ref right) = self.right_child {
            right.borrow().left_order_traversal(visit);
        }
    }
    // if let Some(root) = tree_list[0].as_ref() {
    //     root.borrow().left_order_traversal(&mut |value| {
    //         println!("{}", value);
    //     });
    // }

    fn depth(&self) -> usize {
        let left_depth = self
            .left_child
            .as_ref()
            .map_or(0, |left| left.borrow().depth());
        let right_depth = self
            .right_child
            .as_ref()
            .map_or(0, |right| right.borrow().depth());
        1 + left_depth.max(right_depth)
    }

    fn visit_at_depth(&self, depth: usize, visit: &mut dyn FnMut(i32)) {
        if depth == 1 {
            if let Some(value) = self.value {
                visit(value);
            }
        } else {
            if let Some(ref left) = self.left_child {
                left.borrow().visit_at_depth(depth - 1, visit);
            }
            if let Some(ref right) = self.right_child {
                right.borrow().visit_at_depth(depth - 1, visit);
            }
        }
    }

    pub(crate) fn left_order_depth_first(&self, visit: &mut dyn FnMut(i32)) {
        let max_depth = self.depth();
        for depth in (1..=max_depth).rev() {
            self.visit_at_depth(depth, visit);
        }
    }

    pub(crate) fn add(left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
        let new_root = TreeNode::new(None); // 변경: 'mut' 제거
        new_root.borrow_mut().left_child = Some(left);
        new_root.borrow_mut().right_child = Some(right);
        new_root.borrow_mut().reduce();
        new_root
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0).0 {
                continue;
            }
            if !self.split() {
                break;
            }
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<i32>, Option<i32>) {
        if let Some(_value) = self.value {
            return (false, None, None);
        }

        let left = self.left_child.as_ref().unwrap();
        let right = self.right_child.as_ref().unwrap();

        if depth >= 4 {
            let left_value = left.borrow().value;
            let right_value = right.borrow().value;

            if let (Some(l), Some(r)) = (left_value, right_value) {
                self.value = Some(0);
                self.left_child = None;
                self.right_child = None;
                return (true, Some(l), Some(r));
            }
        }

        if let (true, l, r) = left.borrow_mut().explode(depth + 1) {
            if let Some(r) = r {
                right.borrow_mut().add_leftmost(r);
            }
            return (true, l, None);
        }

        if let (true, l, r) = right.borrow_mut().explode(depth + 1) {
            if let Some(l) = l {
                left.borrow_mut().add_rightmost(l);
            }
            return (true, None, r);
        }

        (false, None, None)
    }

    fn add_leftmost(&mut self, value: i32) {
        if let Some(v) = self.value.as_mut() {
            *v += value;
        } else if let Some(ref left) = self.left_child {
            left.borrow_mut().add_leftmost(value);
        }
    }

    fn add_rightmost(&mut self, value: i32) {
        if let Some(v) = self.value.as_mut() {
            *v += value;
        } else if let Some(ref right) = self.right_child {
            right.borrow_mut().add_rightmost(value);
        }
    }

    fn split(&mut self) -> bool {
        // 변경: &self를 &mut self로 변경
        if let Some(value) = self.value {
            if value >= 10 {
                // Split: create new child nodes with the divided values
                self.left_child = Some(TreeNode::new(Some(value / 2)));
                self.right_child = Some(TreeNode::new(Some((value + 1) / 2)));
                self.value = None;
                return true;
            }
            return false;
        }

        if let Some(ref left) = self.left_child {
            if left.borrow_mut().split() {
                // 변경: borrow()를 borrow_mut()로 변경
                return true;
            }
        }

        if let Some(ref right) = self.right_child {
            if right.borrow_mut().split() {
                // 변경: borrow()를 borrow_mut()로 변경
                return true;
            }
        }

        false
    }

    pub(crate) fn magnitude(&self) -> i32 {
        // If this is a regular number (leaf node), return its value
        if let Some(value) = self.value {
            return value;
        }

        // Otherwise, calculate: 3 * left_magnitude + 2 * right_magnitude
        let left_mag = self
            .left_child
            .as_ref()
            .map(|left| left.borrow().magnitude())
            .unwrap_or(0);
        let right_mag = self
            .right_child
            .as_ref()
            .map(|right| right.borrow().magnitude())
            .unwrap_or(0);

        3 * left_mag + 2 * right_mag
    }
}

pub(crate) fn reduction(a: TreeNodePtr, b: TreeNodePtr) -> TreeNodePtr {
    // TreeNode::add already calls reduce(), so no need to call it again
    TreeNode::add(a, b)
}
