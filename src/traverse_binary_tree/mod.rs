use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::binary_tree::TreeNode;

/// 前序遍历
pub fn pre_order(root: Option<Rc<RefCell<TreeNode<i32>>>>) {}

#[test]
fn test_pre_order() {}

/// 中序遍历
pub fn in_order(root: Option<Rc<RefCell<TreeNode<i32>>>>) {}

#[test]
fn test_in_order() {}

/// 后序遍历
pub fn post_order(root: Option<Rc<RefCell<TreeNode<i32>>>>) {}

#[test]
fn test_post_order() {}

/// 层次遍历
pub fn level_order(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<Vec<i32>> {
    let mut q = VecDeque::new();
    let mut result = vec![];
		
    if let Some(n) = root {
        q.push_back(n);
    } else {
        return result;
    }

    while !q.is_empty() {
        let mut temp = vec![];
        let len = q.len();
        for _i in 0..len {
            if let Some(node) = q.pop_front() {
                temp.push(node.borrow().val);
                if let Some(n) = node.borrow_mut().left.take() {
                    q.push_back(Rc::clone(&n));
                }
                if let Some(n) = node.borrow_mut().right.take() {
                    q.push_back(Rc::clone(&n));
                }
            }
        }
        result.push(temp);
    }
    result
}

#[test]
fn test_level_order() {
	use super::binary_tree::binary_tree_instance;
	let r = binary_tree_instance();
	let v = level_order(r);
	assert_eq!(v, vec![vec![1], vec![2,3], vec![4]]);
}
