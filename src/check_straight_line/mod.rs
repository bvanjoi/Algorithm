pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    if coordinates.len() == 1 {
        panic!("make sure points number >= 2.")
    } else if coordinates.len() == 2 {
    } else {
        let x_offset = coordinates[0][0] - coordinates[1][0]; // x_0 - x_1;
        let y_offset = coordinates[0][1] - coordinates[1][1]; // y_0 - y_1;
        for i in 2..coordinates.len() {
            // is (y - y_0) * (x_0 - x_1) == (x - x_0) * (y_0 - y_1) right?
            if (coordinates[i][1] - coordinates[0][1]) * x_offset
                == (coordinates[i][0] - coordinates[0][0]) * y_offset
            {
            } else {
                return false;
            }
        }
    }
    return true;
}

#[test]
fn test_check_straight_line() {
    assert_eq!(
        check_straight_line(vec![vec![2, 1], vec![4, 2], vec![6, 3]]),
        true
    );
    assert_eq!(
        check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]),
        true
    );
    assert_eq!(
        check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]),
        false
    );
}
