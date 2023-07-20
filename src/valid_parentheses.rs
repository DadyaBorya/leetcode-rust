pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    if let Some(char) = stack.pop() {
                        if char != c {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
}

