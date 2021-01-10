# 排序问题 — 选择排序

## 排序问题概览

- 输入: 一个长度为 $n$ 的序列 $A$, 且 $A = <a_1, a_2, ..., a_n>$
- 输出: 序列 $a_1', a_2', ..., a_n'$ 且满足 $a_1' \le a_2' \le ... \le a_n'$.

## 选择排序简介

### 算法

选择排序是一个*迭代*的过程。每次迭代中将未排序数组中的最小值扔到已排序数组中的**最末尾**。

### 示例

初始时：

- 序列 `A`: [5, 2, 4, 6, 1, 3],

1. 已排序的数组为 [], 未排序的部分为 [5,2,4,6,1,3], 将未排序部分的 1 扔到已排序的末尾。
2. 已排序的数组为 [1], 未排序的部分为 [5,2,4,6,3], 将未排序部分的 2 扔到已排序的末尾。
3. 已排序的数组为 [1,2], 未排序的部分为 [5,4,6,3], 将未排序部分的 3 扔到已排序的末尾。
4. 已排序的数组为 [1,2,3], 未排序的部分为 [5,4,6], 将未排序部分的 4 扔到已排序的末尾。
5. 已排序的数组为 [1,2,3,4], 未排序的部分为 [5,6], 将未排序部分的 5 扔到已排序的末尾。
6. 已排序的数组为 [1,2,3,4,5], 未排序的部分为 [6], 将未排序部分的 6 扔到已排序的末尾。
7. 已排序的数组为 [1,2,3,4,5,6], 未排序的部分为 [], 结束。

### 实现

```Rust
pub fn selection_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
               min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
    return arr;
}
```

### 性能

- 运行时间: $O(n^2)$.
- 空间复杂度: $O(1)$.

### 特性

选择排序满足:

- 对于**较小**的数组，其表现更加优秀。

### 扩展

- 按照从大到小的顺序排序：

```rust
pub fn selection_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 0..arr.len() {
        let mut max_index = i;
        for j in i + 1..arr.len() {
            if arr[j] > arr[max_index] {
               max_index = j;
            }
        }
        arr.swap(i, max_index);
    }
    return arr;
}
```

### 练习

- [排序数组](https://leetcode-cn.com/problems/sort-an-array/)

## 参考

- [selection sort](https://en.wikipedia.org/wiki/Selection_sort)
