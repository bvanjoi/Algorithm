/// # unreachable_stack_order
///
/// * `push_order` The order of push for stack.
/// * `pop_order` The order of pop for stack
///
/// ## Examples
///
/// ```
/// use algorithm::validate_stack_pop_order::validate_stack_pop_order;
/// let result = validate_stack_pop_order(vec![1,2,3], vec![3,1,2]);
/// assert_eq!(result, false);
/// ```
pub fn validate_stack_pop_order(push_order: Vec<i32>, pop_order: Vec<i32>) -> bool {
    /// check weather arr has repeat element
    fn has_repeat(arr: Vec<i32>) -> bool {
        let arr_len = arr.len();
        let set: std::collections::HashSet<i32> = arr.into_iter().collect();
        !(set.len() == arr_len)
    }

    if push_order.len() != pop_order.len() {
        panic!("The element of push_order must be equal pop_order");
    }

    if has_repeat(push_order.clone()) && has_repeat(pop_order.clone()) {
        panic!("The element of push_order or pop_order must unique");
    }
    // 使用 vec 模拟的 栈
    let mut stack = vec![];
    // index of pop_order
    let mut index_pop = 0;

    for it in push_order {
        stack.push(it);

        while !stack.is_empty() {
            let top = *stack.last().unwrap();
            if index_pop == pop_order.len() {
                break;
            }
            if top == pop_order[index_pop] {
                stack.pop();
                index_pop += 1;
            } else {
                break;
            }
        }
    }
    // 如果 stack 为空，则表明 pop_order 有效，返回 true.
    return stack.is_empty();
}

#[test]
fn test_validate_stack_pop_order() {
    assert_eq!(
        validate_stack_pop_order(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]),
        false
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 3, 2, 1, 0, 9, 8, 7, 6, 5]
        ),
        true
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 6, 8, 7, 5, 3, 2, 9, 0, 1]
        ),
        false
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![2, 5, 6, 7, 4, 8, 9, 3, 1, 0]
        ),
        true
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 3, 2, 1, 0, 5, 6, 7, 8, 9]
        ),
        true
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 9, 8, 7, 0]
        ),
        true
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![0, 4, 6, 5, 3, 8, 1, 7, 2, 9]
        ),
        false
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 4, 7, 9, 8, 6, 5, 3, 0, 2]
        ),
        false
    );
    assert_eq!(
        validate_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![2, 1, 4, 3, 6, 5, 8, 7, 9, 0]
        ),
        true
    );
}

#[test]
#[should_panic(expected = "The element of push_order must be equal pop_order")]
fn test_validate_stack_pop_order_failed_unequal() {
    validate_stack_pop_order(
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![2, 1, 4, 3, 6, 5, 8, 7, 9],
    );
}

#[test]
#[should_panic(expected = "The element of push_order or pop_order must unique")]
fn test_validate_stack_pop_order_failed_repeat() {
    validate_stack_pop_order(
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
        vec![0, 2, 1, 4, 3, 6, 5, 8, 7, 9, 1],
    );
}
