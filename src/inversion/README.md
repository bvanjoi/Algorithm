# 数论问题 — 逆序数

## 逆序数简介

假设从小到大为标准次序。

- 逆序：对于 n 个不同元素，在这 n 个元素的任一排列中，当某两个元素的先后次序与标准次序不同时，就说该两个元素逆序。
- 逆序数：一个排列中逆序总数为逆序数。

例如：对于数列 [3,5,4,8,2,6,9], 其中 (3,2), (5,4), (5,2), (4,2), (8,2), (8,6) 为其逆序对，该数列的逆序数为 6.

### 算法

高效地求解一个序列的逆序数是一个数学问题。

首先需要考虑逆序数的一个特性：**一个序列的逆序等于将该数组进行插入排序时移动元素的次数**。

因此，我们可以统计在排序过程中所有元素移动的次数。

又因为归并排序的时间复杂度优秀于插入排序，因此这里使用归并排序来实现逆序数统计。

### 示例

在归并排序中，以 [1,4,6,7,9] 与 [2,3,5,10,13] 为例，

假设，第一个数组中 i = 1, 即指向元素 4. 第二个数组中 j = 0, 即指向元素 2.

此时进行两个数组进行合并。

由于 4 > 2, 则存在逆序对 (4,2), (6,2), (7,2), (9,2), 共计 last_index - i + 1 = 4 个逆序对。

因此，*本次*合并使得逆序数增加 4.

依次类推即可得到原本数组的逆序数。

### 实现

```Rust
pub fn inversion_number_merge_sort(arr: Vec<i32>) -> i32 {
    let mut count = 0;

    fn merge_sort_recursive(arr: &mut Vec<i32>, left: usize, right: usize, count: &mut i32) {
        if left + 1 < right {
            let mid = left + (right - left) / 2;
            merge_sort_recursive(arr, left, mid, count);
            merge_sort_recursive(arr, mid, right, count);
            *count += merge(arr, left, mid, right);
        }
    }

    fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> i32 {
        let mut t: Vec<i32> = Vec::new();
        let mut i = left;
        let mut j = mid;
        let mut c = 0;
        while i < mid && j < right {
            if arr[i] < arr[j] {
                t.push(arr[i]);
                i = i + 1;
            } else {
                t.push(arr[j]);
                j = j + 1;
                c += (mid - i) as i32;
            }
        }
        while i < mid {
            t.push(arr[i]);
            i = i + 1;
        }
        while j < right {
            t.push(arr[j]);
            j = j + 1;
        }
        for k in 0..t.len() {
            arr[k + left] = t[k];
        }
        return c;
    }

    let mut arr: Vec<i32> = arr.clone();
    let left = 0;
    let right = arr.len();
    merge_sort_recursive(&mut arr, left, right, &mut count);
    return count;
}
```

### 性能

- 运行时间: $O(nlog(n))$.
- 空间复杂度: $O(n)$.

### 练习

- [数组中的逆序对](https://leetcode-cn.com/problems/shu-zu-zhong-de-ni-xu-dui-lcof/)

## 参考

- [inversion](https://en.wikipedia.org/wiki/Inversion_(discrete_mathematics))
