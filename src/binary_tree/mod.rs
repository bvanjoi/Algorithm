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
