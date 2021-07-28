# 集合 - 数组的并集

## 并集

对于两个集合 A, B 而言，其并集为 A, B 所有元素组成的集合，写作 $A \cup B$.

## 算法

- 输入：两个以数组形式存储的集合；
- 输出：以数组形式输出的两个集合的并集。

只需要将两个数组合并，然后对合并后的去重。

## 实例

- 输入：`nums1 = [4,9,5]`, `nums2 = [9,4,8]`.

1. 合并后的数组为 `[4,9,5,9,4,8]`.
2. 去重后的数组：`[4,5,8,9]`.

## 实现

- [union 实现](./mod.rs)

## 复杂度

- 运行时间： $O(n)$.
- 空间复杂度：$O(n)$.

## 练习

- [稀疏相似度](https://leetcode-cn.com/problems/sparse-similarity-lcci/)

## 扩展

- 交集与并集的关系：A + B - intersection = union.
- Rust 中 `HashSet` 实现了 union 算法

## 参考

- [union](https://en.wikipedia.org/wiki/Union_(set_theory))
