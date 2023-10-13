use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut degree = 0;
        for index in 0..nums.len() {
            let cell = map.entry(nums[index]).or_insert((0, index as i32, 1));
            cell.0 = cell.0 + 1;
            cell.2 = (index as i32) - cell.1 + 1;
            degree = degree.max(cell.0);
        }
        
        return map.values()
            .filter(|cell| cell.0 == degree)
            .map(|cell| cell.2)
            .min()
            .unwrap() as i32;
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