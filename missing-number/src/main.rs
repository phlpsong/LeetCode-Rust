use std::cmp;
fn main() {
    let res = Solution::missing_number(vec![0,1]);
    println!("res: {}", res);
}

struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut total = 0;
        let n = nums.len() as i32;
        for num in nums {
            total += num;
        }

        let mut total2 = 0;
        let target: Vec<i32> = (0..=n).collect();
        for num in target {
            total2 += num;
        }
        total2 - total
    }
}