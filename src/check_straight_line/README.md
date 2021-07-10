# 几何 - 判断二维平面上的点是否在同一条直线上

## 判断二维平面上的点是否在同一条直线上

### 算法

依照[两点式](https://baike.baidu.com/item/%E4%B8%A4%E7%82%B9%E5%BC%8F)来判断多条点是否为同一直线。

### 实现

```Rust
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
  if coordinates.len() == 1 {
    panic!("make sure points number >= 2.")
  } else if coordinates.len() == 2 {
  } else {
    let x_offset = coordinates[0][0] - coordinates[1][0]; // x_0 - x_1;
    let y_offset = coordinates[0][1] - coordinates[1][1]; // y_0 - y_1;
    for i in 2..coordinates.len() {
      // is (y - y_0) * (x_0 - x_1) == (x - x_0) * (y_0 - y_1) right?
      if (coordinates[i][1] - coordinates[0][1]) * x_offset
        == (coordinates[i][0] - coordinates[0][0]) * y_offset
      {
      } else {
        return false;
      }
    }
  }
  return true;
}
```

### 性能

- 平均运行时间: $O(n)$.
- 空间复杂度: $O(1)$.

## 练习

- [缀点成线](https://leetcode-cn.com/problems/check-if-it-is-a-straight-line/)
- [直线上最多的点数](https://leetcode-cn.com/problems/max-points-on-a-line/)

## 参考

- [slope](https://en.wikipedia.org/wiki/Slope)
