use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}


struct Solution { }

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut v = nums.clone();
        v.sort();
        let mut h = HashMap::new();
        for i in 0..v.len() {
            h.entry(v[i]).or_insert(i as i32);
        }
        nums.into_iter().map(|x| *h.get(&x).unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smaller_numbers_than_current() {
        let case1 = Solution::smaller_numbers_than_current(vec![8,1,2,2,3]);
        assert_eq!(vec![4,0,1,1,3], case1);

        let case2 = Solution::smaller_numbers_than_current(vec![6,5,4,8]);
        assert_eq!(vec![2,1,0,3], case2);

        let case3 = Solution::smaller_numbers_than_current(vec![7,7,7,7]);
        assert_eq!(vec![0,0,0,0], case3);
    }
}