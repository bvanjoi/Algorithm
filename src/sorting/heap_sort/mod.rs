use std::collections::BinaryHeap;

pub fn heap_sort(vec: Vec<i32>) -> Vec<i32> {
    let mut heap = BinaryHeap::from(vec);
    let mut result = vec![];
    while !heap.is_empty() {
        let top_ele = heap.pop().unwrap();
        result.push(top_ele);
    }
    result.reverse();
    return result;
}
