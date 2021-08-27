# 数论问题 - 埃氏筛法

## 问题描述

- 输入：数字 `n`.
- 输出：[1,n] 内所有的素数

## 算法

遍历 [2,n], 将素数的倍数标记为合数，当遍历完成后，剩下违背标记的则为素数。

## 实现

[埃氏筛法](./mod.rs)

## 性能

- 运行时间：O(n(logn)(loglogn))

## 练习

- [计数质数](https://leetcode-cn.com/problems/count-primes/)

## 参考

- [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
