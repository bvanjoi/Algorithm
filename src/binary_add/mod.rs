pub fn binary_add(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    if a.len() == 0 {
        return a;
    } else if b.len() == 0 {
        return b;
    }

    let mut i: usize = a.len() - 1;
    let mut j: usize = b.len() - 1;
    let mut carry: u8 = 0;
    let mut res: Vec<u8> = Vec::new();

    loop {
        if a[i] > 1 || b[j] > 1 {
            panic!("Error! The input had value more than 1.");
        }
        let mut t = a[i] + b[j] + carry;
        if t > 1 {
            t = t - 2;
            carry = 1;
        } else {
            carry = 0;
        }
        res.push(t);
        if i == 0 || j == 0 {
            break;
        }
        i = i - 1;
        j = j - 1;
    }

    while i != 0 {
        i = i - 1;
        let mut t = a[i] + carry;
        if t > 1 {
            t = t - 2;
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
        j = j - 1;
        let mut t = b[j] + carry;
        if t > 1 {
            t = t - 2;
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
