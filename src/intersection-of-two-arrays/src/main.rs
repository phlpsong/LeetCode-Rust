use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set: HashSet<i32> = nums1.into_iter().collect();
        let mut res: HashSet<i32> = HashSet::new();
        for index in 0..nums2.len() {
            if set.contains(&nums2[index]) {
                res.insert(nums2[index]);
            }
        }
        let res = res.into_iter().collect();
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersection() {
        let case1 = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        assert_eq!(vec![2], case1);

        let case2 = Solution::intersection(vec![4, 9, 5], vec![9,4,9,8,4]);
        assert_eq!(vec![4, 9], case2);
    }
}