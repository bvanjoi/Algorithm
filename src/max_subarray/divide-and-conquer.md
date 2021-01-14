# 分治法 — 最大子序和

## 最大子数组问题简介

给定一维数组，找出一个连续的子序列，使该子序列的和最大。

### 算法

分治法求解该问题是一个*递归*的过程，如果想要求解某个数组的最大子序和，则应该将数组划分为两个长度尽可能相等的子序列，例如对于数组 A[low..high] 划分成 A[low..mid], A[mid+1...high], 则最大子序列 A[i..j] 则可能存在三种情况：

1. 完全落在 A[low..mid] 上，即 $low \le i \le j \le mid$.
2. 完全落在 A[mid+1..high] 上，即 $mid < i \le j \le high$.
3. 跨过 `mid`, 即 $low \le i \le mid < j \le hight$.

我们可以**递归**地解决这个问题，首先找到 A[low..mid] 和 A[mid+1..high]的最大子序和（直到遇到只有一个元素的边界情况），而第三种情况（即跨过 `mid` 的情况），可以由上述两种情况计算得到。

### 示例

初始时：

- 序列 `A`: [4, -3, 5, -2, -1, 2, 6, -2].

```text
            [4, -3, 5, -2, -1, 2, 6, -2]
                  /             \
          [4, -3, 5, -2]     [-1, 2, 6, -2]
             /     \             /      \
        [4, -3]   [5, -2]    [-1, 2]    [6, -2]
         /  \      /  \       /   \      /   \
      [4]  [-3]  [5]  [-2]  [-1]  [2]  [6]  [-2]
        \   /      \   /      \    /     \   /
          4          5          2          6 
            \      /             \        /
               6                    8
                   \            /
                        11
```

### 实现

```Rust
pub fn max_subarray_divide_and_conquer(arr: Vec<i32>) -> i32 {
    fn find_max_crossing_subarray(arr: &mut Vec<i32>, left: i32, mid: i32, right: i32) -> i32 {
        let mut left_max_sum = i32::min_value();
        let mut sum = arr[mid];
        for i in (left..mid).rev() {
            sum += arr[i as usize];
            if sum > left_max_sum {
                left_max_sum = sum;
            }
        }
        let mut right_max_sum = i32::min_value();
        let mut sum = arr[mid+1];
        for i in mid+2..=right {
            sum += arr[i as usize];
            if sum > right_max_sum {
                right_max_sum = sum;
            }
        } 
        return left_max_sum + right_max_sum;
    }
    fn recursive(arr: &mut Vec<i32>, left: i32, right: i32) -> i32 {
        if left == right { // 递归的基准情况
            return arr[left as usize]; 
        }
        let mid = left + (right - left) / 2;
        let max_left_sum = recursive(arr, left, mid);
        let max_right_sum = recursive(arr, mid + 1, right); 
        let max_crossing_subarray = find_max_crossing_subarray(arr, left, mid, right);
        return i32::max(max_left_sum, max_right_sum).max(max_crossing_subarray);
    }
    let len = arr.len() as i32;
    let arr = &mut arr.clone();
    recursive(arr, 0, len - 1)
}
```

### 性能

- 运行时间: $O(nlogn)$.
- 空间复杂度: $O(n)$.

## 参考

- [divide and conquer](https://en.wikipedia.org/wiki/Divide-and-conquer_algorithm)
- [maximum subarray](https://en.wikipedia.org/wiki/Maximum_subarray_problem)
