# 字符串问题 - 是否是数字

## 问题描述

- 输入：给定一个字符串 `s`.
- 输出：若在字符串可以表示为一个数字，则返回该 `true`；否则返回 `false`.

## 算法

依据数字的定义（整数、小数、科学技术法）构建有效状态机：

```txt
([+-]?((\\d+\\.)|(\\d+\\.\\d+)|(\\.\\d+)|(\\d+)))([eE][+-]?\\d+)?
```

![有效状态机](https://assets.leetcode-cn.com/solution-static/65/1.png)

## 实现

[是否为数字](./mod.rs)

## 练习

- [有效数字](https://leetcode-cn.com/problems/valid-number/)

## 性能

- 运行时间：O(n);
