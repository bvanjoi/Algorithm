# 集合 - 数组的交集

## 交集

对于两个集合 A, B 而言，其交集为包含所有既属于 A 又属于 B 的元素的集合，写作 $A \cap B$.

## 算法

- 输入：两个以数组形式存储的集合；
- 输出：以数组形式输出的两个集合的交集。

为了更高效地计算两个数组的交集：

1. 首先，将数组转化为集合的形式；运行时间：$O(n)$.
2. 迭代其中一个集合，并统计当前元素是否出现在另一个集合中；运行时间 $O(nlogn)$.
3. 返回交集。

## 示例

- 输入：`nums1 = [4,9,5]`, `nums2 = [9,4,8]`.

1. 转换为集合 `set1 = {4,9,5}`, `set2 = {9,4,8}`.
2. 迭代 `set1`, 发现元素 `4`, `9` 也出现在 `set2` 中；
3. 返回 `[4,9]`.

## 实现

- [intersection 实现](./mod.rs)

## 复杂度

- 运行时间：$O(nlogn)$.
- 空间复杂度：$O(n)$.

## 练习

- [两个数组的交集](https://leetcode-cn.com/problems/intersection-of-two-arrays/)
- [两个数组的交集 II](https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/)

## 扩展

- 交集与并集的关系：A + B - intersection = union.
- Rust 中 `HashSet` 实现了 intersection 算法。

## 参考

- [intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory))
