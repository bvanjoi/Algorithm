# Algorithm implemented by Rust

## 字符串与数字

- [是否为数字](./src/is_number/README.md)
- [n 进制加法](./src/n_add/README.md)

## 数据流

- [多数投票](./src/majority_vote/README.md)

## 排序

> - 输入: 一个长度为 $n$ 的序列 $A$, 且 $A = <a_1, a_2, ..., a_n>$
> - 输出: 序列 $a_1', a_2', ..., a_n'$ 且满足 $a_1' \le a_2' \le ... \le a_n'$.

- [归并排序](./src/merge_sort/README.md)
- [冒泡排序](./src/bubble_sort/README.md)
- [插入排序](./src/insertion_sort/README.md)
- [选择排序](./src/selection_sort/README.md)

## 查找

> - 输入: 一个长度为 $n$ 的序列 $A$, 且 $A = <a_1, a_2, ..., a_n>$ 和一个元素值 $v$.
> - 输出: 当序列 $A$ 中存在元素 $v$ 时，输出一个下标 $i$ 满足 $0 \le i \le A.len()$, 且 $v == A[i]$ 成立；否则输出 -1.

- [线性查找](./src/linear_search/README.md)
- [二分查找](./src/binary_search/README.md)

## 集合论

> 集合论是研究由一堆抽象对象构成的整体的理论，包含集合、元素、关系等。

- [数组的交集](./src/intersection/README.md)
- [数组的并集](./src/union/README.md)
- [并查集](./src/)

## 位运算

- [判断整数是否为 2 的幂](./src/power_of_two/README.md)

## 数论

> 数论是研究整数性质的数学分支。

- [组合数](./src/combination/README.md)
- [逆序数](./src/inversion/README.md)
- [卡特兰数](./src/catalan_number/README.md)
- [霍纳算法](./src/horner_method/README.md)
- [欧几里得算法](./src/euclidean_algorithm/README.md)
- [蔡勒公式](./src/zeller_congruence/README.md)

## 线性代数

- [矩阵乘法](./src/matrix_multiplication/README.md)

## 动态规划

> 对于具有重叠子问题和最优子结构和问题，动态规划往往能给出最优解。

- [最大子序和的三种解法](./src/max_subarray/README.md)

## 贪心

> 局部最优解能够得到全局最优解。

## 栈

> 栈(Stack)是一种后进先出(LIFO, Last In First Out)的数据结构。

- [有效的出栈顺序](./src/validate_stack_pop_order/README.md)
- [中缀表达式转后缀](./src/infix_to_postfix/README.md)

## 链表

> 链表是非顺序存储的线性表。

- [构造单链表](./src/link_list/README.md)

## 树

> 树是由根节点和其孩子构成的抽象数据结构。

- [构建二叉树](./src/binary_tree/README.md)
- [二叉树的遍历](./src/traverse_binary_tree/README.md)
- [以前序遍历和中序遍历构建二叉树](./src/build_binary_tree_from_pre_and_in/README.md)
- [以中序遍历和后序遍历构造二叉树]

## 搜索

> 搜索算法是利用计算机高性能来穷举一个问题的所有解。

- [全排列](./src/permutation/README.md)
- [数组的枚举](./src/array_enumeration/README.md)
- [有向图无环的路径](./src/graph_path/README.md)

## 几何

> 几何，研究形状、大小、图形的相对位置等空间与区关系以及空间形式的度量。

- [可否为三角形](./src/is_triangle/README.md)
- [可否为正方形](./src/is_square/README.md)
- [判断二维平面上的点是否在同一条直线上](./src/check_straight_line/README.md)

## reference

- [Introduction to Algorithms, 3rd Edition](https://web.ist.utl.pt/~fabio.ferreira/material/asa/clrs.pdf)
- [Algorithms 4th](https://algs4.cs.princeton.edu/home/)
