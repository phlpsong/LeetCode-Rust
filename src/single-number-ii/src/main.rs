use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for num in nums {
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
        }
        for (key, val) in map.iter() {
            if *val == 1 {
                return *key;
            }
        }
        -1
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_number() {
        let case1 = Solution::single_number(vec![2,2,3,2]);
        assert_eq!(3, case1);

        let case2 = Solution::single_number(vec![0,1,0,1,0,1,99]);
        assert_eq!(99, case2);
    }
}