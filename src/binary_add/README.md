# 位运算 — 二进制加法

## 位运算概览

处理二进制的各种运算。

## 二进制加法简介

- 输入: 两个二进制序列 $A$ 和 $B$.
- 输出: 返回一个序列 $C$, 其每一位上的值为 $A$ 与 $B$ 对应位上值的和。

### 算法

对两个序列中的对应位相加即可。

### 示例

初始时：

- 序列 `A`: [1, 0, 1], `i` 为 2.
- 序列 `B`: [1, 1], `j` 为 1.
- 进位标志位 `carry`: 0.
- 结果：`C`: []

1. A[2] + B[1] + carry = 0, C = [0]; 后 i = 1, j = 0, carry = 1
2. A[1] + B[0] + carry = 0, C = [0,0]; 后 i = 0, carry = 1.
3. A[0] + carry = 0, C = [0,0,0]; 后 carry = 1.
4. carry = 1, C = [1,0,0,0]; 后 carry = 0.

### 实现

```Rust
pub fn binary_add(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    if a.len() == 0 {
        return a;
    } else if b.len() == 0 {
        return b;
    }

    let mut i: usize = a.len() - 1;
    let mut j: usize = b.len() - 1;
    let mut carry: u8 = 0;
    let mut res: Vec<u8> = Vec::new();

    loop {
        if a[i] > 1 || b[j] > 1 {
            panic!("Error! The input had value more than 1.");
        }
        let mut t = a[i] + b[j] + carry;
        if t > 1 {
            t = t - 2;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(t);
        if i == 0 || j == 0 {
            break;
        }
        i = i - 1;
        j = j - 1;
    }

    while i != 0 {
        i = i - 1;
        let mut t = a[i] + carry;
        if t > 1 {
            t = t - 2;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(t);
        if i == 0 {
            break;
        }
    }

    while j != 0 {
        j = j - 1;
        let mut t = b[j] + carry;
        if t > 1 {
            t = t - 2;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(t);
        if j == 0 {
            break;
        }
    }
    if carry != 0 {
        res.push(1);
    }
    res.reverse();
    return res;
}
```

### 性能

- 运行时间: $O(n)$.
- 空间复杂度: $O(n)$. 如果可以修改原数组，则空间复杂度可简化为 $O(1)$.

### 特性

### 练习

- [二进制求和](https://leetcode-cn.com/problems/add-binary/)
