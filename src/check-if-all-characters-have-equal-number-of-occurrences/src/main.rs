use std::collections::HashMap;

struct Solution { }

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut map = HashMap::new();
        for char in chars {
            let entry = map.entry(char).or_insert(0);
            *entry += 1;
        }
        let values: Vec<i32> = map.values().cloned().collect();
        let res = values[0];
        for index in 1..values.len() {
            if values[index] != res {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_occurrences_equal() {
        let case1 = Solution::are_occurrences_equal("abacbc".to_string());
        assert_eq!(case1, true);

        let case2 = Solution::are_occurrences_equal("aaabb".to_string());
        assert_eq!(case2, false);
    }
}