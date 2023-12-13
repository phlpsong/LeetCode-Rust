struct Solution { }

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for index in 0..nums.len() {
            while nums[index] > 0 && nums[index] < nums.len() as i32 && nums[index] != nums[(nums[index] - 1) as usize] {
                let start = (nums[index] - 1) as usize;
                nums.swap(start, index);
            }
        }

        for index in 0..nums.len() {
            if nums[index] != (index + 1) as i32 {
                return (index + 1) as i32;
            }
        }
        (nums.len() + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        let case1 = Solution::first_missing_positive(vec![1, 2, 0]);
        assert_eq!(case1, 3);

        let case2 = Solution::first_missing_positive(vec![3,4,-1,1]);
        assert_eq!(case2, 2);

        let case3 = Solution::first_missing_positive(vec![7,8,9,11,12]);
        assert_eq!(case3, 1);
    }
}