use std::collections::HashMap;

struct Solution { }

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for index in 0..arr.len() {
            let entry = map.entry(arr[index]).or_insert(0);
            *entry += 1;
        }
        
        let mut res = -1;
        for (key, val) in map.iter() {
            if key == val && *key > res {
                res = *key;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lucky() {
        let case1 = Solution::find_lucky(vec![2,2,3,4]);
        assert_eq!(case1, 2);

        let case2 = Solution::find_lucky(vec![1,2,2,3,3,3]);
        assert_eq!(case2, 3);

        let case3 = Solution::find_lucky(vec![2,2,2,3,3]);
        assert_eq!(case3, -1);

        let case4 = Solution::find_lucky(vec![5]);
        assert_eq!(case4, -1);
    }
}