pub fn n_add(a: Vec<usize>, b: Vec<usize>, n: usize) -> Vec<usize> {
    if a.len() == 0 {
        return a;
    } else if b.len() == 0 {
        return b;
    }

    let mut i: usize = a.len() - 1;
    let mut j: usize = b.len() - 1;
    let mut carry: usize = 0;
    let mut res: Vec<usize> = Vec::new();

    loop {
        if a[i] > n - 1 || b[j] > n - 1 {
            panic!("Error! The input had value more than radix - 1.");
        }
        let mut t = a[i] + b[j] + carry;
        if t > n - 1 {
            t = t - n;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(t);
        if i == 0 || j == 0 {
            break;
        }
        i -= 1;
        j -= 1;
    }

    while i != 0 {
        i -= 1;
        let mut t = a[i] + carry;
        if t > n - 1 {
            t = t - n;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(t);
        if i == 0 {
            break;
        }
    }

    while j != 0 {
        j -= 1;
        let mut t = b[j] + carry;
        if t > n - 1 {
            t = t - n;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(t);
        if j == 0 {
            break;
        }
    }
    if carry != 0 {
        res.push(1);
    }
    res.reverse();
    return res;
}

#[test]
fn test_binary_add() {
    let radix = 2;
    assert_eq!(n_add(vec![0], vec![0], radix), vec![0]);
    assert_eq!(n_add(vec![1], vec![0], radix), vec![1]);
    assert_eq!(n_add(vec![0], vec![1], radix), vec![1]);
    assert_eq!(n_add(vec![1], vec![1], radix), vec![1, 0]);
    assert_eq!(n_add(vec![1, 0], vec![1], radix), vec![1, 1]);
    assert_eq!(n_add(vec![1, 1], vec![1], radix), vec![1, 0, 0]);
    assert_eq!(n_add(vec![0, 1], vec![0], radix), vec![0, 1]);
    assert_eq!(n_add(vec![0, 1], vec![0, 1], radix), vec![1, 0]);
    assert_eq!(n_add(vec![1, 0], vec![0, 1], radix), vec![1, 1]);
    assert_eq!(
        n_add(vec![1, 0, 1, 0], vec![1, 0, 0, 1], radix),
        vec![1, 0, 0, 1, 1]
    );
    assert_eq!(
        n_add(vec![1, 0, 1, 0], vec![1, 0, 1, 0], radix),
        vec![1, 0, 1, 0, 0]
    );
    assert_eq!(
        n_add(vec![1, 0, 1, 0], vec![1, 0, 1, 1], radix),
        vec![1, 0, 1, 0, 1]
    );
    assert_eq!(
        n_add(
            vec![
                1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1,
                0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1,
                1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1
            ],
            vec![
                1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1,
                1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0,
                0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0,
                0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1
            ],
            radix
        ),
        vec![
            1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1,
            0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0,
            1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0,
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn test_normal_add() {
    let radix = 10;
    assert_eq!(n_add(vec![1, 2, 0, 0], vec![3, 4], radix), vec![1, 2, 3, 4]);
    assert_eq!(n_add(vec![2, 7, 4], vec![1, 8, 1], radix), vec![4, 5, 5]);
    assert_eq!(n_add(vec![2, 1, 5], vec![8, 0, 6], radix), vec![1, 0, 2, 1]);
    assert_eq!(
        n_add(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], vec![1], radix),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}
