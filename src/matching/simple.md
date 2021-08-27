# 字符串匹配问题 - 朴素枚举

## 算法

对于文字串 `s1` 以及模式串 `s2`, 可以列举出所有的情况。

枚举 [0, s1.len() - s2.len()], 对于当前下标 i, 判断 s1[i..i+s2.len()] == s2[0..s2.len()] 是否均相等。

## 示例

假设 `s1 = acaabc`, `s2 = aab`, 则匹配过程为：

1. 对于 i = 0, offset = 0 时，`s1[i + offset] == s2[offset]` 成立，则执行 `offset += 1`.

   ```txt
   acaabc
   aab
   ```

2. 对于 i = 0, offset = 1 时，`s1[i + offset] == s2[offset]` 失败，则执行 `i += 1; offset += 0`.

   ```txt
   acaabc
    aab
   ```

3. 直到 i = 2, offset = 3 时，才得到模式串在文本串中*首次*出现的下标 `2`.

   ```txt
   acaabc
     aab
   ```

## 实现

[simple](./mod.rs)

## 性能

- 运行时间：O(n * m)

## 特点

- 枚举法中存在大量的重复计算。
