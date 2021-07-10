# 查找问题 — 二分查找

## 二分查找简介

### 算法

二分查找有一个前提条件：输入的序列 $A$ 为有序序列。

二分查找每次检查当前序列中的中间元素，并将其与待查找元素的大小进行比较：

- 若中间元素等于待查找元素，则返回中间元素下标。
- 若中间元素小于待查找元素，则丢弃序列左侧元素。
- 若中间元素大于待查找元素，则丢弃序列右侧元素。

### 示例

初始时：

- 序列 `A`: [1, 2, 3, 5, 6, 7, 8], `v`: 3.

1. 检查 A[(0 + 7) / 2 = 3], 其值 5, 大于 3, 丢弃右侧元素。
2. 检查 A[(0 + 3) / 2 = 1], 其值 2, 小于 3, 丢弃左侧元素。
3. 检查 A[(1 + 3) / 2 = 2], 其值 3, 等于 3, 返回下标 2.

### 实现

```Rust
pub fn binary_search(arr: Vec<i32>, v: i32) -> i32 {
  let mut left = 0;
  let mut right = arr.len();

  while left < right {
      let mid = left + (right - left) / 2;
      if arr[mid] == v {
          return mid as i32;
      } else if arr[mid] > v {
          right = mid;
      } else {
          left = mid + 1;
      }
  }
  return -1;
}
```

### 性能

- 运行时间:
  - 平均运行时间: $O(logn)$.
  - 最佳情况: $O(1)$. 第一个中间元素即为所寻。
  - 最坏情况: $O(logn)$. 未找到所寻元素或最后一个中间元素为所寻元素。
- 空间复杂度: $O(1)$.

## 扩展

- [查找有序序列中的元素的第一个和最后一个位置](./mod.rs)

## 练习

- [旋转数组的最小数字](https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/)
- [旋转数组的最小数字 II](https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array-ii/)

## 参考

- [binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm)
