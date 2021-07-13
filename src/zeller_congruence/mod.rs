pub fn zeller_congruence(day: i32, month: i32, year: i32) -> usize {
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
    (((y + y / 4 + c / 4 - 2 * c + 26 * (m + 1) / 10 + d - 1) % 7 + 7) % 7) as usize
}

#[test]
fn test_zeller_congruence() {
    assert_eq!(zeller_congruence(31, 8, 2019), 6);
    assert_eq!(zeller_congruence(18, 7, 1999), 0);
    assert_eq!(zeller_congruence(15, 8, 1993), 0);
    assert_eq!(zeller_congruence(29, 2, 2016), 1);
}
