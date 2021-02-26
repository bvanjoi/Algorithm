use algorithm::infix_to_postfix::in2post;
use std::panic::catch_unwind;
#[test]
fn test_in2post() {
    assert_eq!(in2post(String::from("1")), String::from("1"));
    assert_eq!(in2post(String::from("(1)")), String::from("1"));
    assert_eq!(in2post(String::from("123")), String::from("123"));
    assert_eq!(in2post(String::from("(1 + 21) ")), String::from("1 21 +"));
    assert_eq!(in2post(String::from("1 + (21) ")), String::from("1 21 +"));
    assert_eq!(in2post(String::from("1 + 21 ")), String::from("1 21 +"));
    assert_eq!(in2post(String::from("6/2 ")), String::from("6 2 /"));
    assert_eq!(
        in2post(String::from("21+3*(7-4)+831/42")),
        String::from("21 3 7 4 - * + 831 42 / +")
    );
    assert_eq!(
        in2post(String::from("1 + 21 * 3 - 2  / 41 ")),
        String::from("1 21 3 * + 2 41 / -")
    );
    assert_eq!(
        in2post(String::from("1 + 2 * (4-3) + 6/2")),
        String::from("1 2 4 3 - * + 6 2 / +")
    );
}
#[test]
fn test_in2post_failed() {
    assert!(catch_unwind(|| in2post(String::from("(1"))).is_err());
    assert!(catch_unwind(|| in2post(String::from("1 2 +"))).is_err());
    assert!(catch_unwind(|| in2post(String::from("1 2 3 +"))).is_err());
    assert!(catch_unwind(|| in2post(String::from("1 + 2 3 +"))).is_err());
}