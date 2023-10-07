use std::collections::HashMap;
use std::cmp;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1: HashMap<i32, i32> = HashMap::new();
        let mut map2: HashMap<i32, i32> = HashMap::new();

        for index in 0..nums1.len() {
            if let Some(&value) = map1.get(&nums1[index]) {
                map1.insert(nums1[index], value + 1);
            } else {
                map1.insert(nums1[index], 1);
            }
        }

        for index in 0..nums2.len() {
            if let Some(&value) = map2.get(&nums2[index]) {
                map2.insert(nums2[index], value + 1);
            } else {
                map2.insert(nums2[index], 1);
            }
        }

        let mut res: Vec<i32> = vec![];
        for (key, value) in map1.into_iter() {
           if let Some(&val2) = map2.get(&key) {
              for _ in 0..cmp::min(val2, value) {
                res.push(key);
              }
           }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        let case1 = Solution::intersect(vec![1,2,2,1], vec![2,2]);
        assert_eq!(vec![2, 2], case1);

        let case2 = Solution::intersect(vec![4, 9, 5], vec![9,4,9,8,4]);
        assert_eq!(vec![4, 9], case2);
    }
}