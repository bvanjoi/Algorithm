pub fn manacher(s: String) -> String {
    if s.len() == 0 {
        return String::new();
    }

    // 插入字符
    let mut chars = vec!['^'];
    for c in s.chars() {
        chars.push('#');
        chars.push(c);
    }
    chars.push('#');
    chars.push('$');

    let mut c = 0;
    let mut r = 0;
    let mut p = vec![0; chars.len()];
    for i in 1..chars.len() - 1 {
        let mut i_mirror = 0;
        if 2 * c >= i {
            i_mirror = 2 * c - i;
        }
        if r > i {
            // 防止超出 R
            p[i] = usize::min(r - i, p[i_mirror]);
        } else {
            // i == r 的情况
            p[i] = 0;
        }

        // 中心扩展法
        while chars[i + 1 + p[i]] == chars[i - 1 - p[i]] {
            p[i] += 1;
        }
        // 更新 r
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }
    // 找到 p 中的最大值
    let mut max_len = 0;
    let mut center_index = 0;
    for i in 1..chars.len() - 1 {
        if p[i] > max_len {
            max_len = p[i];
            center_index = i;
        }
    }
    // 原字符串的下标
    let start = (center_index - max_len) / 2;
    // 最长回文子串
    (s.chars().collect::<Vec<_>>())[start..start + max_len]
        .into_iter()
        .collect()
}

#[test]
fn test_manacher() {
    assert_eq!(manacher(String::from("babad")), String::from("bab"));
    assert_eq!(manacher(String::from("cbbd")), String::from("bb"));
    assert_eq!(manacher(String::from("a")), String::from("a"));
    assert_eq!(manacher(String::from("ac")), String::from("a"));
}
