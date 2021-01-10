# 排序问题 — 插入排序

## 排序问题概览

- 输入: 一个长度为 $n$ 的序列 $A$, 且 $A = <a_1, a_2, ..., a_n>$
- 输出: 序列 $a_1', a_2', ..., a_n'$ 且满足 $a_1' \le a_2' \le ... \le a_n'$.

## 插入排序简介

### 算法

插入排序是一个*迭代*的过程。每次迭代中将一个待排序的元素插入到已排序的序列中的**适当位置**，形成一个新的已排序数组。

### 示例

初始时：

- 序列 `A`: [5, 2, 4, 6, 1, 3],

1. 首先, 孤立地看 A[0], 它只是单个元素, 因此是它是已排序的. 之后关注 A[1].
2. A[1] = 2 小于 A[0] = 5, 因此将 2 移动到 5 之前, 移动后的序列 `A` 为 [2, 5, 4, 6, 1, 3]. 之后关注 A[2].
3. A[2] = 4 小于 A[1] = 5, 因此将 4 移动到 5 之前, 移动后的序列 `A` 为 [2, 4, 5, 6, 1, 3]. 之后关注 A[3].
4. A[3] = 6 大于等于 A[2] = 5, 因此原地不动, 保持不动的序列 `A` 为 [2, 4, 5, 6, 1, 3]. 之后关注 A[4].
5. A[4] = 1 小于 A[3] = 6, 向左观察，A[4] = 1 甚至小于 A[0] = 2, 因此将 1 移动到 2 之前, 移动后的序列 `A` 为 [1, 2, 4, 5, 6, 3]. 之后关注 A[5].
6. A[5] = 3 小于 A[2] = 4, 大于 A[1] = 2, 因此将 3 移动到 4 之前, 2 之后, 移动后的序列 `A` 为 [1, 2, 3, 4, 5, 6]. 完毕。

### 实现

```Rust
pub fn insertion_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 1..arr.len() {
        let (mut j, key) = (i, arr[i]);
        while j != 0 && key < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
    return arr;
}
```

### 性能

- 运行时间:
  - 平均运行时间: $O(n^2)$.
  - 最佳情况: $O(n)$. 当输入序列原本就有序。
  - 最坏情况: $O(n^2)$. 当输入序列完全倒序。
- 空间复杂度: $O(1)$.

### 特性

插入排序满足:

- 对于**较小**的数组，其表现更加优秀。
- 稳定性。对于相同的元素，排序后相对次序保持不变。

### 扩展

- 按照从大到小的顺序排序：

```rust
pub fn insertion_sort_reverse(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 1..arr.len() {
        let (mut j, key) = (i, arr[i]);
        while j != 0 && key > arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
    return arr;
}
```

- 递归版本的插入排序：

```Rust
pub fn insertion_sort_recursive(arr: Vec<i32>) -> Vec<i32> {
    fn insert(arr: &mut Vec<i32>, n: usize) {
        let mut i = n;
        let key = arr[n];
        while key < arr[i - 1] {
            arr[i] = arr[i - 1];
            i -= 1;
            if i == 0 {
                break;
            }
        }
        arr[i] = key;
    }
    fn insert_sort(arr: &mut Vec<i32>, n: usize) {
        if n > 0 {
            insert_sort(arr, n - 1);
            insert(arr, n);
        }
    }
    let mut arr = arr.clone();
    let len = arr.len();
    insert_sort(&mut arr, len - 1);
    arr
}
```

### 练习

- [排序数组](https://leetcode-cn.com/problems/sort-an-array/)

## 参考

- [insertion sort](https://en.wikipedia.org/wiki/Insertion_sort)
