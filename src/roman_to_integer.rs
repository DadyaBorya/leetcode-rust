pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev_value = 0;

        let roman_chars: Vec<char> = s.chars().collect();

        for &ch in roman_chars.iter().rev() {
            let value = match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if value < prev_value {
                result -= value;
            } else {
                result += value;
            }

            prev_value = value;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}

