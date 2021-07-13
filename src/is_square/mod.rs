pub fn is_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    /// 判断两个向量是否垂直且相等，且向量长度大于 0.
    fn is_vertical_equal(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
        if v1[0] * v2[0] + v1[1] * v2[1] == 0 {
            (v1[0] * v1[0] + v1[1] * v1[1]) > 0
                && (v1[0] * v1[0] + v1[1] * v1[1]) == (v2[0] * v2[0] + v2[1] * v2[1])
        } else {
            false
        }
    }

    fn check(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>, p4: &Vec<i32>) -> bool {
        let v1 = vec![p2[0] - p1[0], p2[1] - p1[1]];
        let v2 = vec![p3[0] - p2[0], p3[1] - p2[1]];
        let v3 = vec![p4[0] - p3[0], p4[1] - p3[1]];
        let v4 = vec![p1[0] - p4[0], p1[1] - p4[1]];
        is_vertical_equal(&v1, &v2)
            && is_vertical_equal(&v2, &v3)
            && is_vertical_equal(&v3, &v4)
            && is_vertical_equal(&v4, &v1)
    }

    // 三种连线情况，只要有一种满足即可。
    check(&p1, &p2, &p3, &p4) || check(&p1, &p3, &p2, &p4) || check(&p1, &p2, &p4, &p3)
}

#[test]
fn test_is_square() {
    assert_eq!(
        is_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
        true
    );
    assert_eq!(
        is_square(vec![0, 0], vec![-1, 0], vec![1, 0], vec![0, 1]),
        false
    );
    assert_eq!(
        is_square(vec![0, 0], vec![0, 0], vec![0, 0], vec![0, 0]),
        false
    );
    assert_eq!(
        is_square(vec![0, 1], vec![1, 2], vec![0, 2], vec![0, 0]),
        false
    );
    assert_eq!(
        is_square(vec![0, 0], vec![0, 2], vec![2, 2], vec![1, -1]),
        false
    );
}
