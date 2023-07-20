pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut longest_prefix = strs[0].clone();

        for s in strs.iter().skip(1) {
            let mut common_prefix_len = 0;
            for (c1, c2) in longest_prefix.chars().zip(s.chars()) {
                if c1 == c2 {
                    common_prefix_len += 1;
                } else {
                    break;
                }
            }

            longest_prefix.truncate(common_prefix_len);
        }

        longest_prefix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]), "fl".to_string());
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ]), "".to_string());
    }
}