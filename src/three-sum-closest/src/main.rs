fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        nums.sort();
        let mut ans = nums[0] + nums[1] + nums[len - 1];
        for index in 0..len {
            let mut left = index + 1;
            let mut right = len - 1;
            while left < right {
                let tmp = nums[left] + nums[index] + nums[right];
                if tmp == target {
                    return target;
                } else {
                    if (tmp - target).abs() < (ans - target).abs() {
                        ans = tmp;
                    } 
                    if tmp > target {
                        right -= 1;
                    } else {
                        left += 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        let case1 = Solution::three_sum_closest(vec![-1,2,1,4], 1);
        assert_eq!(case1, 2);

        let case2 = Solution::three_sum_closest(vec![0,0,0], 1);
        assert_eq!(case2, 0);
    }
}