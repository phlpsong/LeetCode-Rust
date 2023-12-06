use std::collections::HashMap;

struct Solution { }

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();
        s.chars().for_each(|ch| *map.entry(ch).or_insert(0) += 1);

        for (i, c) in s.chars().enumerate() {
            if map[&c] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_uniq_char() {
        let case1 = Solution::first_uniq_char("leetcode".to_string());
        assert_eq!(case1, 0);

        let case2 = Solution::first_uniq_char("loveleetcode".to_string());
        assert_eq!(case2, 2);

        let case3 = Solution::first_uniq_char("aabb".to_string());
        assert_eq!(case3, -1);
    }
}