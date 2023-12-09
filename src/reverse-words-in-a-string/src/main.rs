struct Solution { }

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans = String::new();
        for str in s.split_whitespace().rev() {
            ans.push_str(str);
            ans.push(' ');
        }
        ans.pop();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let case1 = Solution::reverse_words("the sky is blue".to_string());
        assert_eq!(case1, "blue is sky the".to_string());

        let case2 = Solution::reverse_words("  hello world  ".to_string());
        assert_eq!(case2, "world hello".to_string());

        let case3 = Solution::reverse_words("a good   example".to_string());
        assert_eq!(case3, "example good a".to_string());
    }
}