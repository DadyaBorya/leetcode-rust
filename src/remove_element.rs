pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut first_pointer = 0;

        for second_pointer in first_pointer..nums.len() {
            if nums[second_pointer] != val {
                nums[first_pointer] = nums[second_pointer];
                first_pointer += 1;
            }
        }

        first_pointer as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5)
    }
}