struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        let mut next_index = index;
        if nums.len() == 0 {
            return 0;
        }
        while next_index < nums.len() {
            if nums[next_index] == val {
                next_index += 1;
            } else {
                let tmp = nums[next_index];
                nums[next_index] = nums[index];
                nums[index] = tmp;
                index += 1;
                next_index = index;
            }
        }
        index as i32
    }
}
fn main() {
    let mut nums = vec![0,1,2,2,3,0,4,2];
    let res = Solution::remove_element(&mut nums, 2);
    println!("result: {}", res);
}
