struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        
        for index in 1..num_rows+1 {
            let index = index as usize;
            let mut curr_vec: Vec<i32> = vec![1; index];
            for inner_index in 1..index-1 {
                let inner_index = inner_index as usize;
                curr_vec[inner_index] = result[index - 2][inner_index - 1] + result[index - 2][inner_index];
            }
            result.push(curr_vec);
        }

        result
    }
}
fn main() {
    let result = Solution::generate(3);
    println!("result: {:?}", result);
}
