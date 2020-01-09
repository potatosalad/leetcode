/**
 * [0227] Basic Calculator II
 *
 * Implement a basic calculator to evaluate a simple expression string.
 *
 * The expression string contains only non-negative integers, +, -, *, / operators and empty spaces  . The integer division should truncate toward zero.
 *
 * Example 1:
 *
 *
 * Input: "3+2*2"
 * Output: 7
 *
 *
 * Example 2:
 *
 *
 * Input: " 3/2 "
 * Output: 1
 *
 * Example 3:
 *
 *
 * Input: " 3+5 / 2 "
 * Output: 5
 *
 *
 * Note:
 *
 *
 * 	You may assume that the given expression is always valid.
 * 	Do not use the eval built-in library function.
 *
 *
 */
pub struct Solution {}

// submission codes start here

#[derive(Clone, Debug, PartialEq)]
enum CalcToken {
    Num(i64),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug, PartialEq)]
struct CalcParser<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> Iterator for CalcParser<'a> {
    type Item = CalcToken;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            return None;
        }
        let d = &self.data[self.index..];
        if d[0] == '+' as u8 {
            self.index += 1;
            return Some(CalcToken::Add);
        } else if d[0] == '-' as u8 {
            self.index += 1;
            return Some(CalcToken::Sub);
        } else if d[0] == '*' as u8 {
            self.index += 1;
            return Some(CalcToken::Mul);
        } else if d[0] == '/' as u8 {
            self.index += 1;
            return Some(CalcToken::Div);
        } else if d[0] >= '0' as u8 && d[0] <= '9' as u8 {
            let mut i = 1;
            while i < d.len() && d[i] >= '0' as u8 && d[i] <= '9' as u8 {
                i += 1;
            }
            let x: i64 = std::str::from_utf8(&d[..i]).unwrap().parse().unwrap();
            self.index += i;
            return Some(CalcToken::Num(x));
        } else {
            self.index += 1;
            return self.next();
        }
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut parser = CalcParser {
            data: s.as_bytes(),
            index: 0,
        }
        .peekable();
        let mut a_val: i64 = parser
            .next()
            .map(|token| match token {
                CalcToken::Num(n) => n,
                _ => unreachable!(),
            })
            .unwrap();
        loop {
            match parser.next() {
                Some(CalcToken::Add) => match parser.next() {
                    Some(CalcToken::Num(mut b_val)) => {
                        loop {
                            match parser.peek() {
                                Some(CalcToken::Mul) => {
                                    parser.next();
                                    match parser.next() {
                                        Some(CalcToken::Num(c_val)) => {
                                            b_val *= c_val;
                                        }
                                        _ => {
                                            unreachable!();
                                        }
                                    }
                                }
                                Some(CalcToken::Div) => {
                                    parser.next();
                                    match parser.next() {
                                        Some(CalcToken::Num(c_val)) => {
                                            b_val /= c_val;
                                        }
                                        _ => {
                                            unreachable!();
                                        }
                                    }
                                }
                                _ => {
                                    break;
                                }
                            };
                        }
                        a_val += b_val;
                    }
                    _ => {
                        unreachable!();
                    }
                },
                Some(CalcToken::Sub) => match parser.next() {
                    Some(CalcToken::Num(mut b_val)) => {
                        loop {
                            match parser.peek() {
                                Some(CalcToken::Mul) => {
                                    parser.next();
                                    match parser.next() {
                                        Some(CalcToken::Num(c_val)) => {
                                            b_val *= c_val;
                                        }
                                        _ => {
                                            unreachable!();
                                        }
                                    }
                                }
                                Some(CalcToken::Div) => {
                                    parser.next();
                                    match parser.next() {
                                        Some(CalcToken::Num(c_val)) => {
                                            b_val /= c_val;
                                        }
                                        _ => {
                                            unreachable!();
                                        }
                                    }
                                }
                                _ => {
                                    break;
                                }
                            };
                        }
                        a_val -= b_val;
                    }
                    _ => {
                        unreachable!();
                    }
                },
                Some(CalcToken::Mul) => match parser.next() {
                    Some(CalcToken::Num(b_val)) => {
                        a_val *= b_val;
                    }
                    _ => {
                        unreachable!();
                    }
                },
                Some(CalcToken::Div) => match parser.next() {
                    Some(CalcToken::Num(b_val)) => {
                        a_val /= b_val;
                    }
                    _ => {
                        unreachable!();
                    }
                },
                None => {
                    break;
                }
                _ => {
                    unreachable!();
                }
            }
        }
        a_val as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0227() {
        assert_eq!(7, Solution::calculate("3+2*2".to_string()));
        assert_eq!(1, Solution::calculate(" 3/2 ".to_string()));
        assert_eq!(5, Solution::calculate(" 3+5 / 2 ".to_string()));
    }
}
