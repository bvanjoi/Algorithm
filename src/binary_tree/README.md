# 树 - 构建二叉树

## 二叉树

所有的树都可以以某种方式转化为二叉树，因此，二叉树是树中最重要的结构。

二叉树有：

- 一个根节点；
- 每个节点最多只有两个孩子；

### 算法

Rust 中二叉树的构建是一个复杂的过程（相比于其他语言）。

### 示例

```rust
   1
 /   \
2     3
     /
    4 
```

### 实现

```rust
use std::{cell::RefCell, rc::Rc};

/// Definition for a binary tree.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
 val: T,
 left: Option<Rc<RefCell<TreeNode<T>>>>,
 right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
 #[inline]
 pub fn new(val: T) -> Self {
  TreeNode {
   val,
   left: None,
   right: None
  }
 }
}

/// 上述示例可以写为：
fn main() {
    let mut root = TreeNode::new(1);
    let left_node = TreeNode::new(2);
    let mut right_node = TreeNode::new(3);
    let f2_l_node = TreeNode::new(4);
    right_node.left = Some(Rc::new(RefCell::new(f2_l_node)));

    root.left = Some(Rc::new(RefCell::new(left_node)));
    root.right = Some(Rc::new(RefCell::new(right_node)));
}
```

## 参考

- [binary tree](https://en.wikipedia.org/wiki/Binary_tree)
