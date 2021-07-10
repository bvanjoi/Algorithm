# 树 - 从前序遍历和中序遍历序列生成二叉树

## 二叉树特性

- 中序遍历 + 前序遍历
- 中序遍历 + 后序遍历

上述二者均可以生成唯一的二叉树。本文关注 中序遍历 + 前序遍历 生成二叉树。

## 算法

给定两个数组 `pre_order` 和 `in_order`. 遍历 `pre_order`, 每次遍历过程中，以 `pre_order[pre_index]` 为基准，找到 `in_order` 中等于该值的下标，下标左侧为左孩子，下标右侧为右孩子。

## 实现

[从前序遍历和中序遍历序列生成二叉树](./mod.rs)

## 练习

- [重建二叉树](https://leetcode-cn.com/problems/zhong-jian-er-cha-shu-lcof/)
