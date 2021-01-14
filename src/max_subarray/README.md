# 动态规划 — 最大子序和

## 最大子数组问题简介

给定一维数组，找出一个连续的子序列，使该子序列的和最大。

### 算法

动态规划求解该问题是一个*迭代*的过程，该过程维护了一个表示*当前和*的变量 `current_sum`, 对于每个元素，`current_sum` 都会与其相加。当 `current_sum` 小于 0 时，将该变量置为 0; 否则不变。
当遍历结束后，选择 `current_sum` 的最大值作为结果。

### 示例

初始时：

- 序列 `A`: [-2, 1, -3, 4, -1, 2, 1, -5, 4].
- 当前和 `current_sum`: 0.
- 结果 `max_sum`: 0.

迭代过程如下：

| `i` | `A[i]` | `A[i] + current_sum` | `current_sum`（本轮结束后） | `max_sum` |
| -   |    -   |    -                 |    -                      |    -      |
| 0   |   -2   |    -2                |    0                      |    0      |
| 1   |    1   |     1                |    1                      |    1      |
| 2   |   -3   |    -2                |    0                      |    1      |
| 3   |    4   |     4                |    4                      |    4      |
| 4   |   -1   |     3                |    3                      |    4      |
| 5   |    2   |     5                |    5                      |    5      |
| 6   |    1   |     6                |    6                      |    6      |
| 7   |   -5   |     1                |    1                      |    6      |
| 8   |    4   |     5                |    5                      |    6      |

### 实现

```Rust
pub fn max_subarray(arr: Vec<i32>) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;
    for value in arr {
        current_sum = 0.max(current_sum + value);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}
```

### 性能

- 运行时间: $O(n)$.
- 空间复杂度: $O(1)$.

### 特性

### 扩展

最长子序列问题存在多种解法：

- [暴力法](./brute-force.md)
- [分治法](./divide-and-conquer.md)

### 练习

- [最大子序和](https://leetcode-cn.com/problems/maximum-subarray/)

## 参考

- [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming)
- [maximum subarray](https://en.wikipedia.org/wiki/Maximum_subarray_problem)
