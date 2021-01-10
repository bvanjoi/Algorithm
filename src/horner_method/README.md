# 数论问题 — 霍纳算法/秦九韶算法

## 数论问题概览

数论是研究整数性质的数学分支。

## 霍纳算法简介

### 算法

对于多项式 $f(x) = a_0 + a_1x + a_2^2x + ... +a_nx^n$.

霍纳算法的表示为：$f(x) = a_0 + x(a_1 + x(a_2 + ... + x(a_{n-1} + x(a_n)...)))$

递归的表示为：

- $b_n = a_n$
- $b_{n-1} = a_{n-1} + b_nx$
- ....
- $b_1 := a_1 + b_2x$
- $b_0 := a_0 + b_1x$

### 示例

对于: $f(x) = 2x^3 - 6x^2 + 2x - 1$, 当 $x=3$ 时：

$f(x)$ 可以转化为 $-1 + x( 2 + x( -6 + 2x))$. 其值为$5$.

### 实现

```Rust
pub fn horner_method(a: Vec<i32>, x: i32) -> i128 {
    let mut res: i128 = 0;
    for i in (0..a.len()).rev() {
        res = res * x as i128 + a[i] as i128;
    }
    return res;
}
```

### 性能

- 运行时间: $O(n)$.
- 空间复杂度: $O(1)$.

### 练习

- [大数取模](http://acm.hdu.edu.cn/showproblem.php?pid=1212)

## 参考

- [horner's method](https://en.wikipedia.org/wiki/Horner%27s_method)