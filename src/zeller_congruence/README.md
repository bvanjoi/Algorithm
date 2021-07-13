# 数论问题 - 蔡勒公式

## 蔡勒公式简介

蔡勒公式是一种计算某年某月某日是星期几的算法。

## 算法

- 输入：`year`, `month`, `day`
- 输出：`1 | 2 | 3 | 4 | 5 | 6 | 7`

其算法可以表示为：

```rust
fn zeller_congruence(day: i32, month: i32, year: i32) -> usize {
    let mut year = year;
    let m = if month < 3 {
        year -= 1;
        month + 12
    } else {
        month
    };
    let y = year % 100;
    let c = year / 100;

    let d = day;
    (((y + y / 4 + c / 4 - 2 * c + 26 * (m + 1) / 10 + d - 1) % 7 + 7) % 7) as usize
}
```

最终结果满足：

- `0` 代表星期日；
- `1` 代表星期一；
- `2` 代表星期二；
- `3` 代表星期三；
- `4` 代表星期四；
- `5` 代表星期五；
- `6` 代表星期六；

## 性能

- 运行时间：O(1).
- 运行空间：O(1).

## 练习

- [一周中的第几天](https://leetcode-cn.com/problems/day-of-the-week/)

## 参考

- [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
