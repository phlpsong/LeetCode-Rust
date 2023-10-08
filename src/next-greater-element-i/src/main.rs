use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut vec: Vec<i32> = vec![];

        for index in 0..nums2.len() {
            while !vec.is_empty() && vec[vec.len() - 1] < nums2[index] {
                map.insert(vec[vec.len() - 1], nums2[index]);
                vec.pop();
            }
            vec.push(nums2[index]);
        }

        let mut res: Vec<i32> = vec![];
        for index in 0..nums1.len() {
            if let Some(&val) = map.get(&nums1[index]) {
                res.push(val);
            } else {
                res.push(-1);
            }
            
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_greater_element() {
        let case1 = Solution::next_greater_element(vec![4,1,2], vec![1, 3, 4, 2]);
        assert_eq!(vec![-1, 3, -1], case1);

        let case2 = Solution::next_greater_element(vec![2,4], vec![1, 2, 3, 4]);
        assert_eq!(vec![3, -1], case2);
    }
}