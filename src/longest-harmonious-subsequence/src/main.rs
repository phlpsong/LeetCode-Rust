use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        
        let mut res = 0;
        for (&key, &val) in map.iter() {
            if let Some(next_val) = map.get(&(key - 1)) {
                res = res.max(val + next_val);
            }
            if let Some(next_val) = map.get(&(key + 1)) {
                res = res.max(val + next_val);
            }
        }

        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_lhs() {
        let case1 = Solution::find_lhs(vec![1,3,2,2,5,2,3,7]);
        assert_eq!(5, case1);

        let case2 = Solution::find_lhs(vec![1,2,3,4]);
        assert_eq!(2, case2);

        let case3 = Solution::find_lhs(vec![1,1,1,1]);
        assert_eq!(0, case3);
    }
}