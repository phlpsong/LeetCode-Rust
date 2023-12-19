use std::collections::HashSet;

struct Solution { }

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();

        vec![
            set1.difference(&set2).map(|&x| x).collect(),
            set2.difference(&set1).map(|&x| x).collect(),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_difference() {
        let case1 = Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]);
        assert_eq!(vec![vec![1, 3], vec![6, 4]], case1);

        let case2 = Solution::find_difference(vec![1,2,3,3], vec![1,1,2,2]);
        assert_eq!(vec![vec![3], vec![]], case2);
    }
}