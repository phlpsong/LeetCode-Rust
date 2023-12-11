use std::collections::HashMap;

struct Solution { }

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums.into_iter() {
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
        }

        let mut res: Vec<i32> = vec![];
        map.into_iter().for_each(|(key, val)| {
            if val == 1 {
                res.push(key);
            }
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        let case1 = Solution::single_number(vec![1,2,1,3,2,5]);
        assert_eq!(case1, [3, 5]);

        let case1 = Solution::single_number(vec![-1, 0]);
        assert_eq!(case1, [-1, 0]);
    }
}