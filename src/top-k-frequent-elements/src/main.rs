use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashMap = HashMap::new();
        for index in 0..nums.len() {
            let entry = hashMap.entry(nums[index]).or_insert(0);
            *entry += 1;
        }

        let mut hash_vec: Vec<(&i32, &i32)> = hashMap.iter().collect();
        
        hash_vec.sort_by(|a, b| b.1.cmp(a.1));
        println!("{:?}", hash_vec);
        let mut res = vec![];
        for index in 0..k {
            res.push(*hash_vec[index as usize].0);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let case1 = Solution::top_k_frequent(vec![1,1,1,2,2,3], 2);
        assert_eq!(vec![1, 2], case1);

        let case2 = Solution::top_k_frequent(vec![1], 1);
        assert_eq!(vec![1], case2);
    }
}