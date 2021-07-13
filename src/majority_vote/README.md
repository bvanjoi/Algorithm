# 数据流 - 多数投票(Boyer-Moore majority vote algorithm)

## 问题描述

- 输入：给定一个由数字组成的数组 `nums`，判断其中是否存在占比找过一半的元素。
- 输出：若该元素存在，则将其输出；否则为 `None`.

## 算法

> 该算法由 Robert S.Boyer 和 J Strother Moore 于 1981 年发表。

1. 初始化元素 `m`, 并设置计数器 `c = 0`.
2. 迭代数组，对于每一个元素 `it`:
   - 若 `c == 0` 成立，则另 `m = it` 以及 `c += 1`;
   - 否则，若 `m == it` 成立，则 `c += 1`
   - 否则，`c -= 1`.
3. 上述迭代结束后，再次迭代，检查 `m` 在 `nums` 中的个数 `count`, 若 `count > nums.len() / 2` 是否成立，若成立，则 `m` 为占比超过一半的元素。

## 示例

数组 `nums = [2,1,3,1,1]`.

1. 初始时，`c = 0`.
2. 下标 1 处，`c = 1`, `m = 2`.
3. 下标 2 处，`c = 0`, `m = 2`.
4. 下标 3 处，`c = 1`, `m = 3`.
5. 下标 4 处，`c = 1`, `m = 1`.
6. 下标 5 处，`c = 2`, `m = 1`.
7. 再次迭代，检查出 元素 `1` 在原数组中个数为 `3`, 满足条件。

## 实现

[majority_vote](./mod.rs)

## 性能

- 运行时间：$O(n)$.
- 空间：$O(1)$.

## 练习

- [主要元素](https://leetcode-cn.com/problems/find-majority-element-lcci/)

## 参考

- [Boyer–Moore majority vote algorithm](https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm)
