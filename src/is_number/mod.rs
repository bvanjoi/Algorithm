pub fn is_number(s: String) -> bool {
    #[derive(PartialEq, Copy, Clone)]
    /// 当前状态
    enum State {
        /// 初始状态
        Init,
        /// 数字的符号位置
        Sign,
        /// 整数部分
        IntegerNumber,
        /// 小数部分
        DecimalNumber,
        /// 左侧没有整数的小数点
        NoIntegerDecimalPoint,
        /// 左侧有整数的小数点
        DecimalPoint,
        /// 指数符号
        ExpChar,
        /// 指数数字的正负号
        ExpNumberSign,
        /// 指数数字
        ExpNumber,
        /// 非法
        Illegal,
    }

    struct Transfer {
        next_state: State,
        next_available_state: Vec<State>,
    }

    fn get_state(c: char, pre_state: State) -> Transfer {
        match c {
            '+' | '-' => {
                if pre_state == State::Init {
                    Transfer {
                        next_state: State::Sign,
                        next_available_state: vec![
                            State::IntegerNumber,
                            State::NoIntegerDecimalPoint,
                        ],
                    }
                } else {
                    Transfer {
                        next_state: State::ExpNumberSign,
                        next_available_state: vec![State::ExpNumber],
                    }
                }
            }
            '.' => {
                if pre_state == State::IntegerNumber {
                    Transfer {
                        next_state: State::DecimalPoint,
                        next_available_state: vec![State::DecimalNumber, State::ExpChar],
                    }
                } else {
                    Transfer {
                        next_state: State::NoIntegerDecimalPoint,
                        next_available_state: vec![State::DecimalNumber],
                    }
                }
            }
            'e' | 'E' => Transfer {
                next_state: State::ExpChar,
                next_available_state: vec![State::ExpNumberSign, State::ExpNumber],
            },
            '0'..='9' => {
                if pre_state == State::ExpNumber
                    || pre_state == State::ExpNumberSign
                    || pre_state == State::ExpChar
                {
                    Transfer {
                        next_state: State::ExpNumber,
                        next_available_state: vec![State::ExpNumber],
                    }
                } else if pre_state == State::DecimalPoint
                    || pre_state == State::NoIntegerDecimalPoint
                    || pre_state == State::DecimalNumber
                {
                    Transfer {
                        next_state: State::DecimalNumber,
                        next_available_state: vec![State::DecimalNumber, State::ExpChar],
                    }
                } else {
                    Transfer {
                        next_state: State::IntegerNumber,
                        next_available_state: vec![
                            State::IntegerNumber,
                            State::DecimalPoint,
                            State::ExpChar,
                        ],
                    }
                }
            }
            _ => Transfer {
                next_state: State::Illegal,
                next_available_state: vec![],
            },
        }
    }

    let mut now_state = State::Init;
    let mut next_available_state = vec![
        State::Sign,
        State::NoIntegerDecimalPoint,
        State::IntegerNumber,
    ];
    for c in s.chars() {
        let next = get_state(c, now_state);
        now_state = next.next_state.clone();
        if now_state == State::Illegal || next_available_state.contains(&now_state) == false {
            return false;
        }
        next_available_state = next.next_available_state.clone();
    }

    return now_state == State::DecimalNumber
        || now_state == State::IntegerNumber
        || now_state == State::ExpNumber
        || now_state == State::DecimalPoint;
}

#[test]
fn test_is_number() {
    assert_eq!(is_number(String::from("0")), true);
    assert_eq!(is_number(String::from("2")), true);
    assert_eq!(is_number(String::from("0089")), true);
    assert_eq!(is_number(String::from("-0.1")), true);
    assert_eq!(is_number(String::from("+3.14")), true);
    assert_eq!(is_number(String::from("4.")), true);
    assert_eq!(is_number(String::from(".1")), true);
    assert_eq!(is_number(String::from("-.9")), true);
    assert_eq!(is_number(String::from("2e10")), true);
    assert_eq!(is_number(String::from("-90E3")), true);
    assert_eq!(is_number(String::from("3e+7")), true);
    assert_eq!(is_number(String::from("53.5e93")), true);
    assert_eq!(is_number(String::from("-123.456e789")), true);
    assert_eq!(is_number(String::from("46.e3")), true);

    assert_eq!(is_number(String::from(".")), false);
    assert_eq!(is_number(String::from("abc")), false);
    assert_eq!(is_number(String::from("e")), false);
    assert_eq!(is_number(String::from("1a")), false);
    assert_eq!(is_number(String::from("1e")), false);
    assert_eq!(is_number(String::from("e3")), false);
    assert_eq!(is_number(String::from("99e2.5")), false);
    assert_eq!(is_number(String::from("--6")), false);
    assert_eq!(is_number(String::from("-+3")), false);
    assert_eq!(is_number(String::from("95a54e53")), false);
}
