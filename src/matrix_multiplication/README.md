# 枚举法 — 矩阵乘法

## 矩阵乘法简介

### 算法

枚举法是按照矩阵乘法定义执行的算法。

### 示例

初始时：

- 矩阵 `A`: [[ 1,2,3], [ 4,5,6]].
- 矩阵 `B`: [[ 1,2], [ 3,4], [ 5,6]].
则 $A * B$ = [[ 22,28], [ 49, 64]].

### 实现

```Rust
pub fn matrix_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if a.len() == 0 || b.len() == 0 {
        panic!("Error! The input can't be empty");
    } else if b.len() != a[0].len() {
        panic!("Error! The number of rows in B has to be equal to the number of columns of A");
    }
    let n = a.len();
    let m = b[0].len();
    let p = a[0].len();
    let mut c: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            for k in 0..p {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}
```

### 性能

- 运行时间: $O(n^3)$.

## 参考

- [matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication)
