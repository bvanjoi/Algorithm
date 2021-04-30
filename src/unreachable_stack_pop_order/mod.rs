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

/// check weather arr has repeat element
fn has_repeat(arr: Vec<i32>) -> bool {
    let arr_len = arr.len();
    let set: std::collections::HashSet<i32> = arr.into_iter().collect();
    set.len() == arr_len
}

/// # unreachable_stack_order
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
#[cfg(test)]
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

#[test]
fn test_unreachable_stack_pop_order() {
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 3, 2, 1, 0, 9, 8, 7, 6, 5]
        ),
        false
    );
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 6, 8, 7, 5, 3, 2, 9, 0, 1]
        ),
        true
    );
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![2, 5, 6, 7, 4, 8, 9, 3, 1, 0]
        ),
        false
    );
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 3, 2, 1, 0, 5, 6, 7, 8, 9]
        ),
        false
    );
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 9, 8, 7, 0]
        ),
        false
    );
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![0, 4, 6, 5, 3, 8, 1, 7, 2, 9]
        ),
        true
    );
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 4, 7, 9, 8, 6, 5, 3, 0, 2]
        ),
        true
    );
    assert_eq!(
        unreachable_stack_pop_order(
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![2, 1, 4, 3, 6, 5, 8, 7, 9, 0]
        ),
        false
    );
}

#[test]
#[should_panic(expected = "The element of push_order must be equal pop_order")]
fn test_unreachable_stack_pop_order_failed_unequal() {
    unreachable_stack_pop_order(
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![2, 1, 4, 3, 6, 5, 8, 7, 9],
    );
}

#[test]
#[should_panic(expected = "The element of push_order or pop_order must unique")]
fn test_unreachable_stack_pop_order_failed_repeat() {
    unreachable_stack_pop_order(
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1],
        vec![0, 2, 1, 4, 3, 6, 5, 8, 7, 9, 1],
    );
}
