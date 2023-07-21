pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut shift = 0;

        for i in 1..nums.len() {
            if nums[i] == nums[i -1] {
                shift += 1;
            }

            nums[i - shift] = nums[i];
        }

        (nums.len() - shift) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1,1,2]), 2)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5)
    }
}