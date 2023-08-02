fn main() {
    let res = Solution::majority_element(vec![2,2,1,1,1,2,2]);
    println!("res: {}", res);
}

struct Solution { }

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut count = 1;
        for index in 1..nums.len() {
            if nums[index] == res {
                count += 1
            } else {
                count -= 1;
                if count == 0 {
                    res = nums[index + 1];
                }
            }
        }
        res
    }
}