# 枚举法 — 最大子序和

## 最大子数组问题简介

给定一维数组，找出一个连续的子序列，使该子序列的和最大。

### 算法

在所有求解最大子序和的方法中，枚举法是最直观的一种。它目的在于列举出数组内所有相加的子序列，然后找到最大的一个。

### 示例

初始时：

- 序列 `A`: [4, -3, 5, 2].

通过双重循环可以列出所有子序列：[4], [4,-3], [4,-3,5], [4,-3,5,2], [-3], [-3,5], [-3,5,2], [5], [5,2], [2]. 其中最大的和为 7.

因此，结果为 7.

### 实现

```Rust
pub fn max_subarray_brute_force(arr: Vec<i32>) -> i32 {
    if arr.len() == 0 {
        panic!("The array's length must be greater 0");
    }
    let mut max_sum = arr[0];
    for i in 0..arr.len() {
        let mut now_sum =0;
        for j in i..arr.len() {
            now_sum += arr[j];
            max_sum = i32::max(max_sum, now_sum);
        }
    }
    max_sum
}
```

### 性能

- 运行时间: $O(n^2)$.
- 空间复杂度: $O(1)$.

## 参考

- [maximum subarray](https://en.wikipedia.org/wiki/Maximum_subarray_problem)
