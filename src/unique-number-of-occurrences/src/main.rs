use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for index in 0..arr.len() {
            if let Some(&value) = map.get(&arr[index]) {
                map.insert(arr[index], value + 1);
            } else {
                map.insert(arr[index], 1);
            }
        }
        let count_vec: Vec<i32> = map.into_iter()
                                        .map(|(_key, val)| val)
                                        .collect();
        let raw_count = count_vec.len();
        let count_set: HashSet<i32> = count_vec.into_iter().collect();
        return raw_count == count_set.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_occurrences() {
        let case1 = Solution::unique_occurrences(vec![1,2,2,1,1,3]);
        assert_eq!(true, case1);

        let case2 = Solution::unique_occurrences(vec![1, 2]);
        assert_eq!(false, case2);

        let case2 = Solution::unique_occurrences(vec![-3,0,1,-3,1,1,1,-3,10,0]);
        assert_eq!(true, case2);
    }
}