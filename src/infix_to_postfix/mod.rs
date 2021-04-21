/// 中缀表达式转后缀表达式
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
        result.push(s);
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
