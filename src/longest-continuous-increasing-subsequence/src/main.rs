fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut start = 0;
        for index in 0..nums.len() {
            if index > 0 && nums[index] <= nums[index - 1] {
                start = index;
            }
            max_len = max_len.max(index - start + 1);
        }

        return max_len as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_length_of_lcis() {
        let case1 = Solution::find_length_of_lcis(vec![1,3,5,4,7]);
        assert_eq!(3, case1);

        let case2 = Solution::find_length_of_lcis(vec![2,2,2,2,2]);
        assert_eq!(1, case2);
    }
}