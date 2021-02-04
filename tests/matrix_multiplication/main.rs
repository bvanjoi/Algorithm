use algorithm::matrix_multiplication;

fn matrix_multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>, c: Vec<Vec<i32>>) {
    assert_eq!(matrix_multiplication::matrix_multiply(a, b), c);
}

#[test]
fn test_matrix_multiply() {
    matrix_multiply(
        vec![vec![1, 1], vec![2, 0]],
        vec![vec![0, 2, 3], vec![1, 1, 2]],
        vec![vec![1, 3, 5], vec![0, 4, 6]],
    );
    matrix_multiply(
        vec![vec![8, 8, 6]],
        vec![vec![5, 2], vec![1, 3], vec![6, 5]],
        vec![vec![84, 70]],
    );
    matrix_multiply(
        vec![vec![1, 2, 3], vec![2, 3, 4], vec![1, 2, 5]],
        vec![vec![2, 1, 3], vec![5, 0, -2], vec![2, 3, -1]],
        vec![vec![18, 10, -4], vec![27, 14, -4], vec![22, 16, -6]],
    );
    matrix_multiply(
        vec![vec![1, 2, 3], vec![4, 5, 6]],
        vec![vec![1, 2], vec![3, 4], vec![5, 6]],
        vec![vec![22, 28], vec![49, 64]],
    )
}
