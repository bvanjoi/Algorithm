use super::TestCase;

type Input<T> = Vec<T>;
type Expected<T> = Vec<T>;

fn create_sort_test_case<T: Ord + Clone>(input: Vec<T>) -> TestCase<Input<T>, Expected<T>> {
    let mut expected = input.clone();
    expected.sort();
    TestCase::new(input, expected)
}

pub fn cases() -> Vec<TestCase<Vec<i32>, Vec<i32>>> {
    vec![
        create_sort_test_case(vec![]),
        create_sort_test_case(vec![1]),
        create_sort_test_case(vec![1, 1]),
        create_sort_test_case(vec![1, 2, 3, 4, 5]),
        create_sort_test_case(vec![
            1, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5,
        ]),
        create_sort_test_case(vec![5, 4, 3, 2, 1, 0]),
        create_sort_test_case((0..=1000).rev().collect::<Vec<_>>()),
        create_sort_test_case(vec![2, 4, 5, 7, 1, 2, 3, 6]),
        create_sort_test_case(vec![31, 41, 59, 26, 41, 58]),
        create_sort_test_case(vec![3, 4, 2, 1, 7, 5, 8, 9, 0, 6]),
        create_sort_test_case(vec![3, 41, 52, 26, 38, 57, 9, 49]),
        create_sort_test_case(vec![
            2, 123, 4, -1, 21, 41, 5, 423, 43, 5643, 6, 5347, 45, 7, 6454, 3, 21, 14, 124, 12, 4,
            124, 2315, 32,
        ]),
    ]
}
