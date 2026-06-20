use std::cell::RefCell;
use std::rc::Rc;
// use log::debug;

// use crate::advent::day18a::tokenizer::Token;
use super::tokenizer::Token;

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
            if *index < tokens.len() && matches!(&tokens[*index], Token::Comma) {
                *index += 1; // ','를 건너뜀
            }

            let right = parse_tokens(tokens, index);
            if *index < tokens.len() && matches!(&tokens[*index], Token::CloseBracket) {
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

    pub(crate) fn add(left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
        let new_root = TreeNode::new(None);
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
        // 리프 노드인 경우 처리
        if let Some(_value) = self.value {
            return (false, None, None);
        }

        // 내부 노드는 항상 left_child와 right_child가 있어야 함
        // 하지만 안전성을 위해 명시적으로 체크
        let Some(left_child) = self.left_child.as_ref() else {
            // 자식이 없는 내부 노드는 잘못된 상태
            return (false, None, None);
        };

        let Some(right_child) = self.right_child.as_ref() else {
            // 자식이 없는 내부 노드는 잘못된 상태
            return (false, None, None);
        };

        if depth >= 4 {
            let left_value = left_child.borrow().value;
            let right_value = right_child.borrow().value;

            if let (Some(l_value), Some(r_value)) = (left_value, right_value) {
                self.value = Some(0);
                self.left_child = None;
                self.right_child = None;
                return (true, Some(l_value), Some(r_value));
            }
        }

        if let (true, l_value, r_value) = left_child.borrow_mut().explode(depth + 1) {
            if let Some(r_value) = r_value {
                right_child.borrow_mut().add_leftmost(r_value);
            }
            return (true, l_value, None);
        }

        if let (true, l_value, r_value) = right_child.borrow_mut().explode(depth + 1) {
            if let Some(l_value) = l_value {
                left_child.borrow_mut().add_rightmost(l_value);
            }
            return (true, None, r_value);
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
            value
        } else {
            let left_mag = self.left_child.as_ref().map(|left| left.borrow().magnitude()).unwrap_or(0);
            let right_mag = self.right_child.as_ref().map(|right| right.borrow().magnitude()).unwrap_or(0);
            3 * left_mag + 2 * right_mag
        }
    }

    pub(crate) fn tree_to_list(&self) -> String {
        if let Some(value) = self.value {
            value.to_string()
        } else {
            let left_str = self.left_child.as_ref().unwrap().borrow().tree_to_list();
            let right_str = self.right_child.as_ref().unwrap().borrow().tree_to_list();
            format!("[{},{}]", left_str, right_str)
        }
    }
}

// Deep copy of a tree (필수: add()가 원본 트리를 수정하므로)
#[allow(dead_code)]
pub(crate) fn clone_tree(node: &TreeNodePtr) -> TreeNodePtr {
    let borrowed = node.borrow();

    if let Some(value) = borrowed.value {
        // 리프 노드: 새 노드 생성
        TreeNode::new(Some(value))
    } else {
        // 내부 노드: 자식들을 재귀적으로 복제
        let new_node = TreeNode::new(None);

        if let Some(ref left) = borrowed.left_child {
            new_node.borrow_mut().left_child = Some(clone_tree(left));
        }
        if let Some(ref right) = borrowed.right_child {
            new_node.borrow_mut().right_child = Some(clone_tree(right));
        }

        new_node
    }
}
