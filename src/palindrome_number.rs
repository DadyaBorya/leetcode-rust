pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let (mut rev, mut num) = (0, x);
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }

        rev == x
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(is_palindrome(10), false);
    }
}