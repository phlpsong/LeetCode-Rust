use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for num in nums.iter() {
            set.insert(num);
        }
        let mut count = 0;
        for num in nums.iter() {
            let mut tmp = 0;
            let mut curr_num = *num;
            // 排除序列非首个数字cd
            if !set.contains(&(curr_num - 1)) {
                while set.contains(&curr_num) {
                    tmp += 1;
                    curr_num += 1;
                }
                if tmp > count {
                    count = tmp;
                }
            }
           
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        let case1 = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(case1, 4);

        let case1 = Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]);
        assert_eq!(case1, 9);
    }
}

