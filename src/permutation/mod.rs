/// Get the permutation of arr
///
/// - `arr`, origin array.
/// - `visited`, `visited[i]` means weather `arr[i]` had used.
/// - `r`, the record vector.
/// - `index`, used for recursion.
/// - `res`, store result.
pub fn permutation<T: Clone>(
    arr: &Vec<T>,
    visited: &mut Vec<bool>,
    r: &mut Vec<T>,
    index: usize,
    res: &mut Vec<Vec<T>>,
) {
    if r.len() != arr.len() || visited.len() != arr.len() {
        panic!("the record or flag array had wrong.");
    }
    if index == arr.len() {
        res.push(r.to_vec());
        return;
    }
    for i in 0..arr.len() {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        r[index] = arr[i].clone();
        permutation(arr, visited, r, index + 1, res);
        visited[i] = false;
    }
}

#[test]
fn test_permutation1() {
    let arr = vec![1, 2, 3];
    let mut f = vec![false; arr.len()];
    let mut r = vec![arr[0]; arr.len()];
    let mut res = vec![];
    permutation(&arr, &mut f, &mut r, 0, &mut res);
    assert_eq!(
        res,
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    );
}

#[test]
fn test_permutation2() {
    let s = String::from("abc");
    let arr = s.chars().collect::<Vec<char>>();
    let mut f = vec![false; arr.len()];
    let mut r = vec![arr[0]; arr.len()];
    let mut res = vec![];
    permutation(&arr, &mut f, &mut r, 0, &mut res);
    assert_eq!(
        res,
        vec![
            vec!['a', 'b', 'c'],
            vec!['a', 'c', 'b'],
            vec!['b', 'a', 'c'],
            vec!['b', 'c', 'a'],
            vec!['c', 'a', 'b'],
            vec!['c', 'b', 'a']
        ]
    );
}
