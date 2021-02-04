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
