pub fn array_enumeration(r: &mut Vec<i32>, index: usize, m: i32, c: i32, res: &mut Vec<Vec<i32>>) {
    if index == r.len() {
        if c == m {
            res.push(r.to_vec());
        }
        return;
    }
    r[index] = 1;
    array_enumeration(r, index + 1, m, c + 1, res);
    r[index] = 0;
    array_enumeration(r, index + 1, m, c, res);
}

#[test]
fn test_array_enumeration() {
    let mut r = vec![0, 0, 0];
    let m = 2;
    let mut res = vec![];
    array_enumeration(&mut r, 0, m, 0, &mut res);

    assert_eq!(res, vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
}
