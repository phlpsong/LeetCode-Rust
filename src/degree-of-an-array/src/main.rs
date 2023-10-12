use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for index in 0..nums.len() {
            if let Some(val) = map.get(&nums[index]) {
                map.insert(nums[index], vec![val[0] + 1, val[1], index as i32]);
            } else {
                map.insert(nums[index], vec![1, index as i32, index as i32]);
            }
        }
        let mut degree = 1;
        let mut count = i32::MAX;
        for (&_key, &ref val) in map.iter() {
            if val[0] > degree {
                degree = val[0];
                count = val[2] - val[1];
            } else if val[0] == degree {
                count = count.min(val[2] - val[1]);
            }
        }
        return count + 1;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_shortest_sub_array() {
        let case1 = Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]);
        assert_eq!(2, case1);

        let case2 = Solution::find_shortest_sub_array(vec![1,2,2,3,1,4,2]);
        assert_eq!(6, case2);
    }
}