use std::cell::RefCell;
use std::rc::Rc;
// use log::debug;
use crate::advent::day18::tokenizer::Token;

pub(crate) type TreeNodePtr = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
pub(crate) struct TreeNode {
    pub(crate) value: Option<i32>,
    pub(crate) left_child: Option<TreeNodePtr>,
    pub(crate) right_child: Option<TreeNodePtr>,
}

// 패턴 1: 리프 노드 (Leaf Node)
// TreeNode {
//     value: Some(42),      // ✅ 숫자 값
//     left_child: None,     // ❌ 자식 없음
//     right_child: None,    // ❌ 자식 없음
// }

// 패턴 2: 내부 노드 (Internal Node)
// TreeNode {
//     value: None,          // ❌ 값 없음
//     left_child: Some(...),  // ✅ 왼쪽 자식
//     right_child: Some(...), // ✅ 오른쪽 자식
// }

//-- 샘플 입력, 이 입력을 parse하여 생성한 트리구조 예시 --
// 입력: [[1,2],3]
// 출력:
//           Root
//        value: None
//       /           \
//    Node           Leaf
//  value: None     value: 3
//   /      \
// Leaf    Leaf
// value:1 value:2

// parse tokens and make a tree node.
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

    // add two trees together.
    // -- always reference the root node and call the add() function which will 
    //call the reduce() function.
    pub(crate) fn add(left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
        // debug!("add(): left = {}, right = {}", tree_to_list(&left), tree_to_list(&right));
        let new_root = TreeNode::new(None); 
        new_root.borrow_mut().left_child = Some(left);
        new_root.borrow_mut().right_child = Some(right);
        // debug!("add(): new_root = {}", tree_to_list(&new_root));
        new_root.borrow_mut().reduce();
        new_root
    }

    // reduce the tree by exploding and splitting. 
    // -- always reference the root node and call the reduce() function which will 
    // call the explode() and split() functions.
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

    // DFS & left-first traversal
    // DFS 탐색: 항상 왼쪽 자식을 먼저 재귀적으로 방문한 뒤 오른쪽 자식을 탐색 (문제 요구사항)
    fn explode(&mut self, depth: usize) -> (bool, Option<i32>, Option<i32>) {
        // let indent = "  ".repeat(depth);
        // debug!("{}explode: depth = {}", indent, depth);
        // 리프 노드인 경우 처리
        if let Some(_value) = self.value {
            return (false, None, None);
        }

        // 내부 노드인 경우 처리
        let left_child = self.left_child.as_ref().unwrap();
        let right_child = self.right_child.as_ref().unwrap();

        // 깊이가 4 이상인 경우 처리 - 실제 explode 발생 및 처리
        if depth >= 4 {
            // 왼쪽 자식과 오른쪽 자식의 값을 가져옴
            let left_value = left_child.borrow().value;
            let right_value = right_child.borrow().value;

            // 왼쪽 자식과 오른쪽 자식의 값이 모두 있는 경우 처리
            if let (Some(l), Some(r)) = (left_value, right_value) {
                // 현재 노드를 값이 0인 리프 노드로 변경 (explode!) 
                //- Explode: 이 쌍을 0으로 교체, 
                //- 핵심: 자식의 값을 바꾸는 게 아니라, 부모(self)를 완전히 다른 노드(리프 노드 0)로 바꾸는 것입니다! 
                self.value = Some(0);
                self.left_child = None;
                self.right_child = None;
                // 원래 자식들의 값(l, r)을 반환하여 양 옆에 전달
                return (true, Some(l), Some(r));
            }
        }
        // depth < 4 인 경우 재귀적으로 탐색 - 즉, 현재 level에서 다음 level로 (depth + 1) 이동
        else {
            // 1. 왼쪽 자식을 먼저 재귀 호출
            if let (true, l, r) = left_child.borrow_mut().explode(depth + 1) {
                // 오른쪽 트리의 가장 왼쪽 숫자에 r을 더함
                if let Some(r) = r {
                    right_child.borrow_mut().add_leftmost(r);
                }
                return (true, l, None);
            }

            // 2. 왼쪽에서 폭발 없으면 오른쪽 자식을 재귀 호출
            if let (true, l, r) = right_child.borrow_mut().explode(depth + 1) {
                // 왼쪽 트리의 가장 오른쪽 숫자에 l을 더함
                if let Some(l) = l {
                    left_child.borrow_mut().add_rightmost(l);
                }
                return (true, None, r);
            }
        }

        (false, None, None)
    }

    // 트리의 가장 왼쪽 숫자에 value를 더함
    fn add_leftmost(&mut self, value: i32) {
        // debug!("add_leftmost: value = {}", value);
        if let Some(v) = self.value.as_mut() {
            *v += value;
        } else if let Some(ref left) = self.left_child {
            left.borrow_mut().add_leftmost(value);
        }
    }

    // 트리의 가장 오른쪽 숫자에 value를 더함
    fn add_rightmost(&mut self, value: i32) {
        // debug!("add_rightmost: value = {}", value);
        if let Some(v) = self.value.as_mut() {
            *v += value;
        } else if let Some(ref right) = self.right_child {
            right.borrow_mut().add_rightmost(value);
        }
    }

    fn split(&mut self) -> bool {
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

// convert the tree to a list string.
// -- always reference the root node and call the to_list_string() function.
pub(crate) fn tree_to_list(node: &TreeNodePtr) -> String {
    node.borrow().to_list_string()
}

// Deep copy of a tree (필수: add()가 원본 트리를 수정하므로)
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