struct Solution { }

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let chars1: Vec<char> = word1.iter().map(|word| word.chars()).flat_map(|ch| ch).collect();
        let chars2: Vec<char> = word2.iter().map(|word| word.chars()).flat_map(|ch| ch).collect();
        if chars1.len() != chars2.len() {
            return false;
        }
        for index in 0..chars1.len() {
            if chars1[index] != chars2[index] {
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
    fn test_array_strings_are_equal() {
        let case1 = Solution::array_strings_are_equal(vec!["ab".to_string(), "c".to_string()], vec!["a".to_string(), "bc".to_string()]);
        assert_eq!(case1, true);

        let case2 = Solution::array_strings_are_equal(vec!["a".to_string(), "cb".to_string()], vec!["ab".to_string(), "c".to_string()]);
        assert_eq!(case2, false);

        let case3 = Solution::array_strings_are_equal(vec!["abc".to_string(), "d".to_string(), "defg".to_string()], vec!["abcddefg".to_string()]);
        assert_eq!(case3, true);
    }
}