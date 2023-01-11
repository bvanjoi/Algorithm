/// C(n, k)
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(combination(1, 1), 1);
/// assert_eq!(combination(5, 2), 10);
/// assert_eq!(combination(6, 3), 20);
/// assert_eq!(combination(52, 5), 2598960);
/// ```
///
/// ```should_panic
/// use algorithm::number::*;
/// combination(1, 2);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(n)** | **O(1)** |
pub fn combination(n: i32, k: i32) -> i32 {
    if n < k {
        panic!("Error! n must greater or equal than k");
    }
    let mut res = 1;
    for i in 1..(k + 1) {
        res = res * (n - k + i) / i;
    }
    return res;
}

/// Great Common Divisor Using Euclidean Algorithm
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(gcd(4, 3), 1);
/// assert_eq!(gcd(6, 9), 3);
/// assert_eq!(gcd(10, 15), 5);
/// assert_eq!(gcd(252, 105), 21);
/// assert_eq!(gcd(462, 1071), 21);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(log(min(a,b)))** | **O(1)** |
pub fn gcd(a: i32, b: i32) -> i32 {
    if a < b {
        return gcd(b, a);
    }
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

/// Least Common Multiple
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(lsm(4, 3), 12);
/// assert_eq!(lsm(6, 9), 18);
/// assert_eq!(lsm(10, 15), 30);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(log(min(a,b)))** | **O(1)** |
pub fn lsm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

/// zellers formula: calculate the day of the week
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(zeller_formula(31, 8, 2019), 6);
/// assert_eq!(zeller_formula(18, 7, 1999), 7);
/// assert_eq!(zeller_formula(15, 8, 1993), 7);
/// assert_eq!(zeller_formula(29, 2, 2016), 1);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(1)** | **O(1)** |
///
pub fn zeller_formula(day: usize, month: usize, year: usize) -> usize {
    let mut year = year;
    let m = if month < 3 {
        year -= 1;
        month + 12
    } else {
        month
    };
    let y = year % 100;
    let c = year / 100;
    let d = day;
    let result = ((y + y / 4 + c / 4 + 26 * (m + 1) / 10 + d - 2 * c - 1) % 7 + 7) % 7;
    match result {
        0 => 7,
        _ => result,
    }
}

/// Josephus problem
///
///
/// * `n`: the number of people standing in the circle.
/// * `m`: remove one for each **m** individuals.
///
/// return the safe position who will survive the execution.
///
/// reference: <https://en.wikipedia.org/wiki/Josephus_problem>
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(josephus_permutation(5, 2), 2);
/// assert_eq!(josephus_permutation(5, 3), 3);
/// assert_eq!(josephus_permutation(6, 5), 0);
/// assert_eq!(josephus_permutation(10, 17), 2);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(n)** | **O(1)** |
pub fn josephus_permutation(n: usize, m: usize) -> usize {
    match n {
        1 => 0,
        _ => (josephus_permutation(n - 1, m) + m) % n,
    }
}

/// return the value of polynomial multiplication by horner method
///
/// # Arguments
///
/// * `a` - the coefficients of polynomial, the index correspnd to powers
/// * `x` - the value of variable
///
/// # Example
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(horner_method(vec![1,2,3,4], 2), 49); // x + x^2 + x^3 + x^4
/// assert_eq!(horner_method(vec![1, 2, 3, 4], 2), 49);
/// assert_eq!(horner_method(vec![-1, 2, -6, 2], 3), 5)
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(n)** | **O(1)** |
pub fn horner_method(a: Vec<i32>, x: i32) -> i128 {
    let mut res: i128 = 0;
    for i in (0..a.len()).rev() {
        res = res * x as i128 + a[i] as i128;
    }
    return res;
}

/// Get all prime number in range [0, n].
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(euler_sieve(0), vec![(0, false)]);
/// assert_eq!(euler_sieve(1), vec![(0, false), (1, false)]);
/// assert_eq!(euler_sieve(2), vec![(0, false), (1, false), (2, true)]);
/// assert_eq!(
///     euler_sieve(10),
///     vec![
///       (0, false), (1, false), (2, true), (3, true), (4, false), (5, true),
///       (6, false), (7, true), (8, false), (9, false), (10, false)
///     ]
/// );
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(n(logn)(loglogn))** | **O(n)** |
pub fn euler_sieve(n: usize) -> Vec<(usize, bool)> {
    if n == 0 {
        return vec![(0, false)];
    } else if n == 1 {
        return vec![(0, false), (1, false)];
    }
    let mut r = vec![true; n + 1];
    r[0] = false;
    r[1] = false;
    for i in 2..=n {
        if r[i] == false {
            continue;
        }
        let mut j = 2;
        while i * j < n + 1 {
            r[(i * j)] = false;
            j += 1;
        }
    }

    r.into_iter().enumerate().collect::<Vec<_>>()
}

/// Generate lexicographical numbers from lower to upper.
///
/// ```
/// use algorithm::number::*;
/// assert_eq!(
///     lexical_order(1, 34),
///     vec![
///         1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 2, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
///         3, 30, 31, 32, 33, 34, 4, 5, 6, 7, 8, 9
///     ]
/// );
/// assert_eq!(
///     lexical_order(1, 13),
///     vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
/// );
/// assert_eq!(lexical_order(0, 2), vec![0, 1, 2]);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(n)** | **O(n)** |
pub fn lexical_order(lower: usize, upper: usize) -> Vec<usize> {
    if lower > upper {
        panic!("lower must be less than upper");
    }
    let mut ans = if lower == 0 { vec![0] } else { vec![] };
    let mut iter = if lower == 0 { 1 } else { lower };
    let upper = if lower == 0 { 1 + upper } else { upper };
    while ans.len() < upper {
        if iter == 0 {
            unreachable!();
        } else if iter <= upper {
            ans.push(iter);
            iter *= 10;
        } else {
            iter += 1;
            while iter % 10 == 0 {
                iter /= 10;
            }
        }
    }
    ans
}
