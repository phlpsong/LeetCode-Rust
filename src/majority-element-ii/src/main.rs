use std::collections::HashMap;

struct Solution { }

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let target = (nums.len() / 3) as i32;
        for num in nums {
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
        }
        let mut res = Vec::new();
        for (key, val) in map.iter() {
            if *val > target {
                res.push(*key);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_majority_element() {
        let case1 = Solution::majority_element(vec![3,2,3]);
        assert_eq!(vec![3], case1);

        let case2 = Solution::majority_element(vec![1,1]);
        assert_eq!(vec![1], case2);

        let case3 = Solution::majority_element(vec![1, 2]);
        assert_eq!(vec![2, 1], case3);
    }
}