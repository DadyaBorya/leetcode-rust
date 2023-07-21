pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}