# 数据流问题 - Fisher–Yates 洗牌算法

## 问题

- 输入：给定一个数组 `nums`.
- 输出：乱序的数组。

## 算法

假设 `nums` 的长度为 n.

1. 从 1-n 中任选一个数字 j, 交换 `num[j]` 与 `num[n]`.
2. 令 `n -= 1`, 并重复步骤 1.
3. 当 `n == 1` 成立时停止。

## 实现

[洗牌算法](./mod.rs)

## 性能

- 运行时间：O(n)

## 参考

- [Fisher–Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)
