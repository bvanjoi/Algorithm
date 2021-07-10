# 数论问题 — 组合数

## 组合数简介

从 $n$ 个元素中提取出 $k$ 个元素，则其组合数为：

$C_n^k = \frac{n!}{k!(n-k)!}$

### 算法

$C_n^k = \frac{n * (n-1) * ... * (n-k+1)}{k * (k-1) * ... * 1} = \frac{n}{k}\frac{n-1}{k-1}...\frac{n-k+1}{1}$

### 示例

$C_5^2 = \frac{5}{3} * \frac{4}{2} * \frac{3}{1}$.

### 实现

```Rust
pub fn combination(n: i32, k: i32) -> i32 {
    if n < k {
        panic!("Error! n must greater or equal than k");
    }
    let mut res = 1;
    for i in 1..(k + 1) {
        res = res * (n - k + i) / i;
    }
    return res;
}
```

### 性能

- 运行时间: $O(n)$.
- 空间复杂度: $O(1)$.

### 特性

### 练习

## 参考

- [combination](https://en.wikipedia.org/wiki/Combination)
- [number theory](https://en.wikipedia.org/wiki/Number_theory)
