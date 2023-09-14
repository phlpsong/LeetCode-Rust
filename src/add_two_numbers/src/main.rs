use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let len = candy_type.len();
        let set: HashSet<i32> = candy_type.into_iter().collect();
        std::cmp::min(len / 2, set.len()) as i32
    }
}

fn main() {
    // println!("Hello, world!");
    let candy_type = vec![1, 1, 1, 1,];
    let solution = Solution::distribute_candies(candy_type);
    println!("solution: {}", solution);
}
