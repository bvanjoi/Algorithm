pub fn matrix_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if a.len() == 0 || b.len() == 0 {
        panic!("Error! The input can't be empty");
    } else if b.len() != a[0].len() {
        panic!("Error! The number of rows in B has to be equal to the number of columns of A");
    }
    let n = a.len();
    let m = b[0].len();
    let p = a[0].len();
    let mut c: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            for k in 0..p {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

#[test]
fn test_matrix_multiply() {
    assert_eq!(
        matrix_multiply(
            vec![vec![1, 1], vec![2, 0]],
            vec![vec![0, 2, 3], vec![1, 1, 2]]
        ),
        vec![vec![1, 3, 5], vec![0, 4, 6]],
    );

    assert_eq!(
        matrix_multiply(
            vec![vec![8, 8, 6]],
            vec![vec![5, 2], vec![1, 3], vec![6, 5]]
        ),
        vec![vec![84, 70]],
    );

    assert_eq!(
        matrix_multiply(
            vec![vec![1, 2, 3], vec![2, 3, 4], vec![1, 2, 5]],
            vec![vec![2, 1, 3], vec![5, 0, -2], vec![2, 3, -1]]
        ),
        vec![vec![18, 10, -4], vec![27, 14, -4], vec![22, 16, -6]],
    );

    assert_eq!(
        matrix_multiply(
            vec![vec![1, 2, 3], vec![4, 5, 6]],
            vec![vec![1, 2], vec![3, 4], vec![5, 6]]
        ),
        vec![vec![22, 28], vec![49, 64]],
    )
}
