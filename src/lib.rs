#[cfg(test)]
mod test_helper;

mod binary_search;
mod bubble_sort;
mod insertion_sort;
mod inversion;
mod merge_sort;
mod selection_sort;

pub use binary_search::*;
pub use bubble_sort::*;
pub use insertion_sort::*;
pub use inversion::*;
pub use merge_sort::merge_sort;
pub use selection_sort::*;

// pub mod bit;
// pub mod number;
// // 大数
// pub mod n_add;

// // 字符串
// pub mod is_number;
// pub mod manacher;
// pub mod matching;

// // 数据流
// pub mod dutch_nation_flag;
// pub mod fisher_yates_shuffle;
// pub mod majority_vote;

// // 排序
// pub mod sorting;
// // 查找
// pub mod linear_search;

// // 集合论
// pub mod intersection;
// pub mod union;
// pub mod union_find;

// // 线性代数
// pub mod matrix_multiplication;

// // 动态规划
// pub mod max_subarray;

// // 栈
// pub mod infix_to_postfix;
// pub mod validate_stack_pop_order;

// // 链表
// pub mod link_list;

// // 树
// pub mod binary_tree;
// pub mod build_binary_tree_from_pre_and_in;
// pub mod traverse_binary_tree;

// // 搜索
// pub mod array_enumeration;
// pub mod graph_path;
// pub mod permutation;

// // 几何
// pub mod check_straight_line;
// pub mod is_square;
// pub mod is_triangle;
