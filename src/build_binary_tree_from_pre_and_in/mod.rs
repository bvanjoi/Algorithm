use super::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub fn build_binary_tree_from_pre_and_in(
    pre_order: Vec<i32>,
    in_order: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    /// 构建二叉树的递归函数
    /// * `pre_order`: 前序遍历序列
    /// * `pre_index`: 当前下标
    /// * `in_order`: 中序遍历序列
    /// * `in_order_start`: 孩子起始范围
    /// * `in_order_end`: 孩子起始范围
    fn build_binary(
        pre_order: &Vec<i32>,
        pre_index: usize,
        in_order: &Vec<i32>,
        in_order_start: usize,
        in_order_end: usize,
    ) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        // 边界
        if in_order_start == in_order_end {
            return None;
        }
        // 当前节点
        let val = pre_order[pre_index];
        let mut node = TreeNode::new(val);

        // 当前 pre_order 的元素在 in_order 中的下标
        // 由限制条件可知，其值必然存在
        let in_index = in_order.iter().position(|&x| x == val).unwrap();

        // 左孩子
        node.left = build_binary(pre_order, pre_index + 1, in_order, in_order_start, in_index);
        // 右孩子
        node.right = build_binary(
            pre_order,
            pre_index + 1 + in_index - in_order_start,
            in_order,
            in_index + 1,
            in_order_end,
        );

        return Some(Rc::new(RefCell::new(node)));
    }

    if pre_order.len() == 0 {
        return None;
    }
    return build_binary(&pre_order, 0, &in_order, 0, in_order.len());
}

#[test]
fn test_build_binary_tree_from_pre_and_in() {
    // root:
    //    1
    //	 /  \
    // 2    3
    let mut root = TreeNode::new(1);
    let left_node = TreeNode::new(2);
    let right_node = TreeNode::new(3);

    root.left = Some(Rc::new(RefCell::new(left_node)));
    root.right = Some(Rc::new(RefCell::new(right_node)));

    assert_eq!(
        build_binary_tree_from_pre_and_in(vec![1, 2, 3], vec![2, 1, 3]),
        Some(Rc::new(RefCell::new(root)))
    );
}
