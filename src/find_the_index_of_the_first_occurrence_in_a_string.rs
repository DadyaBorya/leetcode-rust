pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }

        if haystack.len() < needle.len() {
            return -1;
        }

        for i in 0..=(haystack.len() - needle.len()) {
            if haystack[i..(i + needle.len())] == needle {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::str_str("sabutsad".to_string(), "sad".to_string()), 5)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1)
    }
}
