use std::{cell::RefCell, rc::Rc};

/// Definition for a binary tree.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    /// The value of node.
    pub val: T,
    /// left child.
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    /// right child
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Get a instance of binary tree.
///
///```txt
///    1
///  /  \
/// 2    3
///     /
///    4
///```
pub fn binary_tree_instance() -> Option<Rc<RefCell<TreeNode<i32>>>> {
    let mut root = TreeNode::new(1);
    let left_node = TreeNode::new(2);
    let mut right_node = TreeNode::new(3);
    let f2_l_node = TreeNode::new(4);
    right_node.left = Some(Rc::new(RefCell::new(f2_l_node)));

    root.left = Some(Rc::new(RefCell::new(left_node)));
    root.right = Some(Rc::new(RefCell::new(right_node)));
    Some(Rc::new(RefCell::new(root)))
}
