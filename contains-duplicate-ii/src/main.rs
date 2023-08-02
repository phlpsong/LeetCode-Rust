use std::collections::HashMap;

fn main() {
    let result = Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2);
    println!("res: {}", result);
}

struct Solution {}
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {

        let mut hashmap = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            if let Some(pre) = hashmap.insert(n, i) {
                if i - pre <= k as usize {
                    return true;
                }
            }
        }
        false
    }
}