# 栈应用 - 有效的出栈顺序

## 有效的出栈顺序

假如入栈的元素按顺序为 1, 2, 3, 则可能存在的出栈顺序有：

|出栈顺序| 解释|
|-|-|
|1,2,3| push(1), pop(), push(2), pop(), push(3), pop()|
|1,3,2| push(1), pop(), push(2), push(3), pop(), pop()|
|2,1,3| push(1), push(2), pop(), pop(), push(3), pop()|
|2,3,1| push(1), push(2), pop(), push(3), pop(), pop()|
|3,2,1| push(1), push(2), push(3), pop(), pop(), pop()|

同时，不可能的出栈顺序为：

- `[3,1,2]`, 该例中，当 3 为第一个出栈元素，表明 1,2 已经入栈，且 2 必然在 1 之上。因此 1 不可能先于 2 出栈。

本文就来实现一个算法：对于给定的入栈顺序与出栈顺序，检查该出站顺序是否可以实现，若是，则有效。

### Rust 中模拟栈

在 Rust 标准库中，并没有实现栈这个数据结构，因此，使用 `Vec` 来模拟栈。

### 算法流程

为了判断出栈顺序是否有效，首先需要输入两个参数：

- `push_order`: 入栈顺序；
- `pop_order`: 出栈顺序；

同时，需要保证：1. 两个参数的元素个数、内容相等，仅在顺序上不一致；2. 为了方便处理，需要保证两个参数数组中**无重复元素**（在实际问题中，可以使用下标当作 key）。

函数初始时，首先初始化一个栈 `stack = vec![];`, 和 `index_pop = 0;`

随后，迭代 `push_order`, 在每次迭代过程中，将入栈顺序依次入栈到 `stack` 中。由于元素的唯一性，若 `stack` 的栈顶元素等于 `pop_order[index_pop]` 时，`index_pop += 1`, `stack` 出栈。直到迭代结束。

最终，检查 `stack` 是否为空，若为空，则表明出栈顺序有效。

### 算法示例

以

- `push_order`: `1, 2, 3, 4, 5`,
- `pop_order`: `4, 5, 3, 2, 1`

为例：

初始时：

```txt
index_push
  |
  1, 2, 3, 4, 5
  4, 5, 3, 2, 1
  |
index_pop

stack: []
```

1. `index_push` 为 0, 对应 `push_order[index_push]` 为 1, stack 将 1 入栈，而由于 stack.top 为 1 不等于 `pop_order[index_pop]` （即为 4），因此继续迭代。

最终迭代到：

```txt
      index_push
           |
  1, 2, 3, 4, 5
  4, 5, 3, 2, 1
  |
index_pop

stack: [1,2,3,4]
```

2. 此时， `stack.top` 等于 `pop_order[index_pop]`, 随后出栈，同时 `index_pop` 向右移动。

此时为：

```txt
      index_push
           |
  1, 2, 3, 4, 5
  4, 5, 3, 2, 1
     |
 index_pop

stack: [1,2,3]
```

3. 当 `stack.top` 不等于 `pop_order[index_pop]` 时，继续迭代 `push_order`:

```txt
         index_push
              | 
  1, 2, 3, 4, 5
  4, 5, 3, 2, 1
     |
 index_pop

stack: [1,2,3,5]
```

4. 此时，`stack.top` 再次等于 `pop_order[index_pop]`, 最终有：

```txt
            index_push
                | 
  1, 2, 3, 4, 5
  4, 5, 3, 2, 1
                |
           index_pop

stack: []
```

5. 迭代完成， `stack.is_empty()` 成立，表明此出栈顺序有效。

### 算法实现

可见： [validate_stack_pop_order](./mod.rs/).

## 扩展

- 卡特兰数：对于某个给定的 `push_order`, 其所有出栈顺序的个数为 `catalan(push_order.len());`

## 参考

- [stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type))
