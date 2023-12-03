struct Solution { }

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = n as usize;
        let mut res = Vec::new();
        while right < nums.len() {
            res.push(nums[left]);
            res.push(nums[right]);
            left += 1;
            right += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        let case1 = Solution::shuffle(vec![2,5,1,3,4,7], 3);
        assert_eq!(case1, [2,3,5,4,1,7]);

        let case2 = Solution::shuffle(vec![1,2,3,4,4,3,2,1], 4);
        assert_eq!(case2, [1,4,2,3,3,2,4,1]);

        let case3 = Solution::shuffle(vec![1,1,2,2], 2);
        assert_eq!(case3, [1,2,1,2]);
    }
}