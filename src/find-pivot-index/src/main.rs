fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let total_sum: i32 = nums.iter().sum();

        for index in 0..nums.len() {
            if left_sum * 2 == total_sum - nums[index] {
                return index as i32;
            }
            left_sum += nums[index];
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pivot_index() {
        let case1 = Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]);
        assert_eq!(3, case1);

        let case2 = Solution::pivot_index(vec![1, 2, 3]);
        assert_eq!(-1, case2);

        let case3 = Solution::pivot_index(vec![2, 1, -1]);
        assert_eq!(0, case3);
    }
}