# 栈应用 - 不可能的出栈顺序

## 栈简介

栈(Stack)是一种后进先出(LIFO, Last In First Out)的数据结构。

栈包含两个最基本的操作：

- `push(x)`: 将元素 x 入栈；
- `pop()`: 栈定元素出栈。

## 不可能的出栈顺序

假如入栈的元素按顺序为 1, 2, 3, 则可能存在的出栈顺序有：

|出栈顺序| 解释|
|-|-|
|1,2,3| push(1), pop(), push(2), pop(), push(3), pop()|
|1,3,2| push(1), pop(), push(2), push(3), pop(), pop()|
|2,1,3| push(1), push(2), pop(), pop(), push(3), pop()|
|2,3,1| push(1), push(2), pop(), push(3), pop(), pop()|
|3,2,1| push(1), push(2), push(3), pop(), pop(), pop()|

同时，不可能的出栈顺序为：

- 3,1,2, 该例中，当 3 为第一个出栈元素，表明 1,2 已经入栈，且 2 必然在 1 之上。因此 1 不可能先于 2 出栈。

本文就来实现一个算法：对于给定的入栈顺序，某种出栈顺序是否可能存在。

### Rust 中模拟栈

在 Rust 标准库中，并没有实现栈这个数据结构，因此，首先来实现一个栈。

为了简单起见，该栈只能存储 `i32` 类型，并只实现创建实例、入栈、出栈、获取顶部元素等基本操作。

代码如下：

```rust
/// # Stack
///
/// The data struct to explain unreachable 
struct Stack {
    /// A vector to store stack elements.
    r: Vec<i32>,
}

impl Stack {
    /// Create a new Stack
    pub fn new() -> Stack {
        return Stack { r: vec![] };
    }

    /// Push element to r
    pub fn push(&mut self, ele: i32) {
        self.r.push(ele);
    }

    /// Pop element from r
    pub fn pop(&mut self) {
        if self.r.is_empty() {
            panic!("stack is empty");
        }
        self.r.pop();
    }

    /// Get the top element of r
    pub fn top(&self) -> i32 {
        if self.r.is_empty() {
            panic!("stack is empty");
        }
        *self.r.last().unwrap()
    }
}
```

### 算法流程

为了判断出栈顺序是否成立，首先需要输入两个参数：

- `push_order`: 入栈顺序；
- `pop_order`: 出栈顺序；

同时，需要保证：1. 两个参数的元素个数、内容相等，仅在顺序上不一致；2. 为了方便处理，需要保证两个参数数组中无重复元素。

函数初始时，首先初始化一个栈 `stack`,

随后，迭代 `pop_order`, 在每次迭代过程中，需要将找到当前元素 `pop_order[index_pop]` 在 `push_order` 内的下标 `index_push_target`:

- 若 `index_push_target > index_push_now`, 则将 `push_order[index_push_now..index_push_tart]` 入栈，并执行 `index_push_now = index_push_target + 1`.
- 若 `index_push_target <= index_push_now`, 则检查 `stack.top()` 是否等于 `pop_order[index_pop]`, 若等于，则继续迭代；否则告知系统，该出栈顺序无法实现。

### 算法示例

以

- `push_order`: `0, 1, 2, 3, 4, 5, 6, 7, 8, 9`,
- `pop_order`: `4, 3, 2, 1, 0, 9, 8, 7, 6, 5`

为例：

初始时：

```txt
index_push_now
  |
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9
  4, 3, 2, 1, 0, 9, 8, 7, 6, 5
  |
index_pop

stack: []
```

1. `index_pop` 为 0, 对应 `pop_order[index_pop]` 为 4, 4 在 `push_order` 中的下标为 4, 大于 `index_push_now`, 则入栈相应部分元素，最终 `stack` 内的值为 [0,1,2,3].

此时：

```txt
           index_push_now
                 |
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9
  4, 3, 2, 1, 0, 9, 8, 7, 6, 5
     |
 index_pop

stack: [0,1,2,3]
```

2. `pop_order[index_pop]` 的值为 3, 3 在 `push_order` 中的位置小于 `index_push_now`, 且 `stack.top()` 元素为 3, 表明可以正确出栈，继续迭代。

此时为：

```txt
           index_push_now
                 |
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9
  4, 3, 2, 1, 0, 9, 8, 7, 6, 5
        |
    index_pop

stack: [0,1,2]
```

3. 同理至 `index_pop` 等于 5 处，此时为：

```txt
           index_push_now
                 |
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9
  4, 3, 2, 1, 0, 9, 8, 7, 6, 5
                 |
            index_pop

stack: []
```

4. 由于元素 9 在 `push_order` 中的顺序大于 `index_push_now`, 则执行入栈操作，并迭代：

```txt
                      index_push_now
                               |
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9
  4, 3, 2, 1, 0, 9, 8, 7, 6, 5
                    |
               index_pop

stack: [5,6,7,8]
```

5. 重复上述过程，当 `index_pop` 迭代结束时表示该出栈顺序正确。

### 算法实现

```rust
struct Stack {
    /// A vector to store stack elements.
    r: Vec<i32>,
}

impl Stack {
    /// Create a new Stack
    pub fn new() -> Stack {
        return Stack { r: vec![] };
    }

    /// Push element to r
    pub fn push(&mut self, ele: i32) {
        self.r.push(ele);
    }

    /// Pop element from r
    pub fn pop(&mut self) {
        if self.r.is_empty() {
            panic!("stack is empty");
        }
        self.r.pop();
    }

    /// Get the top element of r
    pub fn top(&self) -> i32 {
        if self.r.is_empty() {
            panic!("stack is empty");
        }
        *self.r.last().unwrap()
    }
}

/// check weather arr has repeat element
fn has_repeat(arr: Vec<i32>) -> bool {
    let arr_len = arr.len();
    let set: std::collections::HashSet<i32> = arr.into_iter().collect();
    set.len() == arr_len
}

/// # unreachable_stack_pop_order
///
/// * `push_order` The order of push for stack.
/// * `pop_order` The order of pop for stack
/// 
/// ## Examples
///
/// ```
/// let result = unreachable_stack_pop_order(vec![1,2,3], vec![3,1,2]);
/// assert_eq!(result, false);
/// ```
pub fn unreachable_stack_pop_order(push_order: Vec<i32>, pop_order: Vec<i32>) -> bool {
    if push_order.len() != pop_order.len() {
        panic!("The element of push_order must be equal pop_order");
    }
    if !(has_repeat(push_order.clone()) && has_repeat(pop_order.clone())) {
        panic!("The element of push_order or pop_order must unique");
    }

    let mut stack = Stack::new();
    // index of pop_order
    let mut index_pop = 0;
    // now index of push_order
    let mut index_push_now = 0;

    while index_pop < pop_order.len() {
        let index_push_target_wrap = push_order.iter().find(|&n| *n == pop_order[index_pop]);
        if index_push_target_wrap == None {
            panic!("The element of push_order must be equal pop_order");
        }
        let index_push_target = *index_push_target_wrap.unwrap() as usize;

        if index_push_now > index_push_target {
            if stack.top() != pop_order[index_pop] {
                // it means the order is wrong.
                // println!("Error in {}", pop_order[index_pop]);
                return true;
            } else {
                stack.pop();
            }
        } else {
            // iter
            while index_push_now < index_push_target {
                stack.push(push_order[index_push_now]);
                index_push_now += 1;
            }
            index_push_now += 1;
        }

        // go to next
        index_pop += 1;
    }
    return false;
}
```

## 扩展

- 卡特兰数：对于某个给定的 `push_order`, 其所有出栈顺序的个数为 `catalan(push_order.len());`

## 参考

-
