use std::cmp;
fn main() {
    let res = Solution::find_max_consecutive_ones(vec![0]);
    println!("res: {}", res);
}

struct Solution { }

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut max = res;
        for index in 0..nums.len() {
            if nums[index] == 1 {
                res += 1;
                max = cmp::max(res, max);
            } else {
                res = 0;
            }
        }
        max
    }
}