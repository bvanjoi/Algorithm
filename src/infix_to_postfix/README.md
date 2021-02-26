# 中缀表达式与逆波兰表达式的转换

## 简介

- 中缀表达式，即运算符在运算数中间，例如：`a + b * (c - d) + e/f`.
- 波兰表达式，又称为前缀表达式，例如：`+a+*b-cd/ef`.
- 逆波兰表达式，又称为后缀表达式，例如：`abcd-*+ef/+`.

我们的工作是：通过**栈**来将中缀表达式转换为逆波兰表达式。

> 为什么要进行如此转换?
>
> 对于计算机而言，逆波兰表达式更方便运算。

### 算法

首先，明确运算符的优先级，从高到低依旧是：括号，乘除，加减。

其次，我们的算法思想是：从左到右遍历字符串，若遇到运算数，则记录到结果数组中，若遇到运算符，则：

1. 处理栈中元素，从栈顶开始：
    - 若当前运算符为 '(', 直接入栈。
    - 若当前运算符为 ')', 则出栈直到遇到 '(', 若栈中无 '(', 则报错。
    - 若栈顶运算符为 '('，当前运算符为非 ')' 的其他运算符，则当前元素直接入栈；
    - 若栈顶部元素为 ')'，则报错；
    - 若当前栈顶运算符的优先级**大于等于**当前运算符优先级，则出栈并记录到结果数组中，直到栈为空或不满足前述条件。
2. 当前元素入栈。

### 示例

对于中缀表达式：`1 + 2 * (4-3) + 6/2`, 其过程如下：

|index |operation sign stack|result                  | instructions     |
| -    |  -                 | -                      |  -               |
| 0    | empty              | 1                      | 读取运算数 1       |
| 1    | +                  | 1                      | 运算符 + 入栈      |
| 2    | +                  | 1 2                    | 读取运算数 2       |
| 3    | + *| 1 2                    |* 优先级大于 +, * 入栈  |
| 4    | + * (              | 1 2                    | ( 直接入栈        |
| 5    | + * (              | 1 2 4                  | 读取运算数 4       |
| 6    | + * ( -            | 1 2 4                  | 栈顶为 (, 直接入栈  |
| 7    | + * ( -            | 1 2 4 3                | 读取运算数 3       |
| 8    | + *                | 1 2 4 3 -              | 遇到 ), 适配到 (   |
| 9    | +                  | 1 2 4 3 - *+          | + 优先级小于等于*, +   |
| 10   | +                  | 1 2 4 3 - * + 6        | 读取运算数 6       |
| 11   | + /                | 1 2 4 3 - * + 6        | / 优先级大于 +, / 入栈  |
| 12   | + /                | 1 2 4 3 - * + 6 2      | 读取运算数 2       |
| 13   |                    | 1 2 4 3 - * + 6 2 / +  | 出栈              |

### 实现

```rust
pub fn in2post(s: String) -> String {
    // 记录操作符的栈
    let mut record_stack: Vec<String> = vec![];
    // 记录结果的字符串数组
    let mut result: Vec<String> = vec![];
    // 记录当前数字
    let mut temp_number = String::new();
    /// 当 temp_number 不为空时， 将 temp_number 存入到 result 中
    /// * `result` - The result of string
    /// * `temp_number` - The number of you input
    fn gen_vars(result: &mut Vec<String>, temp_number: &mut String) {
        if temp_number.len() == 0 {
            return;
        }
        result.push(temp_number.clone());
        temp_number.clear();
    }
    /// 处理运算符
    /// * `it` - operations
    fn deal_operation(
        it: char,
        record_stack: &mut Vec<String>,
        result: &mut Vec<String>,
        temp_number: &mut String,
    ) {
        /// 处理右括号
        fn deal_right_parenthesis(record_stack: &mut Vec<String>, result: &mut Vec<String>) {
            while record_stack.len() > 0 {
                let s = record_stack.pop();
                match s {
                    Some(s) => {
                        if s == "(" {
                            return;
                        } else {
                            result.push(s);
                        }
                    }
                    None => panic!("Error! Please input a valid string"),
                }
            }
        }
        // 处理加减号
        fn deal_add_and_sub(record_stack: &mut Vec<String>, result: &mut Vec<String>) {
            while record_stack.len() > 0 {
                let s = record_stack.last();
                match s {
                    Some(s) => {
                        if s == "+" || s == "-" || s == "*" || s == "/" {
                            result.push(s.clone());
                            record_stack.pop();
                        } else {
                            break;
                        }
                    }
                    None => panic!("Error! Please input a valid string"),
                }
            }
        }
        match it {
            '(' => record_stack.push(it.to_string()),
            ')' => {
                gen_vars(result, temp_number);
                deal_right_parenthesis(record_stack, result);
            }
            '*' | '/' => {
                gen_vars(result, temp_number);
                record_stack.push(it.to_string());
            }
            '+' | '-' => {
                gen_vars(result, temp_number);
                deal_add_and_sub(record_stack, result);
                record_stack.push(it.to_string());
            }
            _ => (),
        }
    }
    for it in s.chars() {
        match it {
            '(' | ')' | '*' | '/' | '+' | '-' => {
                deal_operation(it, &mut record_stack, &mut result, &mut temp_number);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => temp_number.push(it),
            ' ' => (),
            _ => panic!("Error! Please input a valid string"),
        }
    }
    if temp_number.len() > 0 {
        gen_vars(&mut result, &mut temp_number);
    }
    while let Some(s) = record_stack.pop() {
        result.push(s)
    }
    /// 检查后缀表达式的有效性并输出结果
    fn compute(result: &Vec<String>) -> i32 {
        fn is_valid(r: &Vec<i32>) -> bool {
            if r.len() < 2 {
                false
            } else {
                true
            }
        }
        let mut r: Vec<i32> = vec![];
        for it in result {
            if it == "+" || it == "-" || it == "*" || it == "/" {
                if !is_valid(&r) {
                    panic!("Error! Please input a valid string");
                }
                let second_number = r.pop().unwrap();
                let first_number = r.pop().unwrap();
                if it == "+" {
                    r.push(first_number + second_number);
                } else if it == "-" {
                    r.push(first_number - second_number);
                } else if it == "*" {
                    r.push(first_number * second_number);
                } else if it == "/" {
                    r.push(first_number / second_number);
                }
            } else {
                if it.parse::<i32>().is_err() {
                    panic!("Error! Please input a valid string");
                }
                r.push(it.parse().unwrap());
            }
        }
        if r.len() == 1 {
            return r.pop().unwrap();
        } else {
            panic!("Error! Please input a valid string");
        }
    }
    compute(&result.clone());
    result.join(" ")
}
```

### 性能

- 运行时间: $O(n)$.
- 空间复杂度： $O(n)$.

### 练习

## 参考

- [Reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
