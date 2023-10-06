fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_first = i32::MIN;
        let mut max_second = i32::MIN;

        for index in 0..nums.len() {
            if nums[index] > max_first {
                max_second = max_first;
                max_first = nums[index];
            } else if nums[index] > max_second {
                max_second = nums[index];
            }
        }

        return (max_first - 1) * (max_second - 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_product() {
        let case1 = Solution::max_product(vec![3, 4, 5, 2]);
        assert_eq!(12, case1);

        let case2 = Solution::max_product(vec![1,5,4,5]);
        assert_eq!(16, case2);

        let case3 = Solution::max_product(vec![3, 7]);
        assert_eq!(12, case3);
    }
}