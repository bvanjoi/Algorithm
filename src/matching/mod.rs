/// 朴素匹配算法
pub fn simple(s1: String, s2: String) -> Option<usize> {
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    if s1.len() < s2.len() {
        return None;
    }
    for begin in 0..=(s1.len() - s2.len()) {
        let mut offset = 0;
        while offset < s2.len() {
            if s1[begin + offset] == s2[offset] {
                offset += 1;
            } else {
                break;
            }
        }
        if offset == s2.len() {
            return Some(begin);
        }
    }
    None
}

#[test]
fn test_matching() {
    fn test(s1: &str, s2: &str, result: Option<usize>) {
        assert_eq!(simple(s1.to_string(), s2.to_string()), result);
    }

    test("BBCABCDABABCDABCDABDE", "ABCDABD", Some(13));
    test("BBC_ABCDAB_ABCDABCDABDE", "ABCDABD", Some(15));
    test("acaabc", "acaabc", Some(0));
    test("ababababac", "abababac", Some(2));
    test("ababababac", "abababacd", None);
    test("000010001010001", "0001", Some(1));
    test("", "", Some(0));
    test("", "a", None);
}
