# 位运算 - 判断整数是否为 2 的幂

## 问题描述

给定整数 num, 判断它是否是 2 的幂。

## 算法

> 固然，可以使用 O(logn) 量级的除法来统计二进制形式内 1 的个数，但是这种做法并不优雅。

对于任何 2 的幂而言，其二进制形式都只有一个 1, 据此，原问题就转化为如何判断二进制行驶中是否只有一个 1, 其方法为 `n & (n - 1) == 0`.

## 实现

[判断整数是否为 2 的幂](./mod.rs)

## 性能

- 运行时间 $O(1)$.

## 练习

- [2 的幂](https://leetcode-cn.com/problems/power-of-two/)
- [位 1 的个数](https://leetcode-cn.com/problems/number-of-1-bits/)
- [4 的幂](https://leetcode-cn.com/problems/power-of-four/)

## 参考

- [位运算](https://en.wikipedia.org/wiki/Bitwise_operation)
