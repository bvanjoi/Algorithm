/// Inversion of a number array
///
/// Let `A[0..n]` be an array of `n` distinct` numbers,
/// If `i < j` and `A[i] > A[j]`, then the pair `(i, j)`
/// is called an *inversion` of `A`.
///  
/// ```
/// use algorithm::inversion;
///
/// assert_eq!(inversion(&vec![2, 3, 8, 6, 1]), 5);
// ```
pub fn inversion<T: Ord + Clone>(arr: &[T]) -> usize {
    let mut cloned = arr.to_owned();
    let len = cloned.len();
    super::merge_sort::merge_sort_inner(&mut cloned, 0, len)
}

#[test]

fn inversion_test() {
    assert_eq!(inversion(&vec![2, 1]), 1);
    assert_eq!(inversion(&Vec::<i32>::new()), 0);
    assert_eq!(inversion(&vec![0]), 0);
    assert_eq!(inversion(&vec![1, 2]), 0);
    assert_eq!(inversion(&vec![-1, 0, 1]), 0);
    assert_eq!(inversion(&vec![3, 2, 1]), 3);
    assert_eq!(inversion(&vec![3, 5, 4, 8, 2, 6, 9]), 6);
    assert_eq!(inversion(&vec![1, 3, 2, 3, 1]), 4);
    assert_eq!(inversion(&vec![1, 3, 2, 3, 1]), 4);
}
