struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result = result ^ num;
        }

        result
    }
}
fn main() {
    let result = Solution::single_number(vec![4, 1, 2, 1, 2]);
    println!("res: {}", result);
}
