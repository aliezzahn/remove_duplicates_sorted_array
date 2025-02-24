pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 1; // Pointer for placing the next unique element

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2], vec![1, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);

        let mut nums = vec![];
        assert_eq!(Solution::remove_duplicates(&mut nums), 0);
        assert_eq!(nums, vec![]);

        let mut nums = vec![1, 1, 1];
        assert_eq!(Solution::remove_duplicates(&mut nums), 1);
        assert_eq!(nums[..1], vec![1]);
    }
}