pub fn counting_sort(arr: Vec<i32>) -> Vec<i32> {
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let mut c: Vec<usize> = vec![0; (max - min + 1) as usize];
    arr.iter().for_each(|&v| c[(v - min) as usize] += 1);
    let mut c: Vec<usize> = c
        .iter()
        .scan(0, |acc, &v| {
            *acc += v;
            Some(*acc)
        })
        .collect();
    let mut b = vec![0; arr.len()];
    arr.iter().rev().for_each(|&v| {
        // 反向遍历 arr 中当前元素
        // index 为该元素在 c 中的位置
        let index = (v - min) as usize;
        // 有 c[index] - 1 元素小于等于 v.
        b[c[index] - 1] = v;
        c[index] -= 1;
    });
    b
}
