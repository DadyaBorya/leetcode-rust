pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut overflow = 1;
        let mut vec: Vec<i32> = vec![];
        for &i in digits.iter().rev() {
            let sum = i + overflow;
            vec.push(sum % 10);
            overflow = sum / 10;
        }

        if overflow > 0 {
            vec.push(overflow);
        }

        vec.reverse();
        vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
