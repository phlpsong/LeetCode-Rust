struct Solution { }

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();
        let n = haystack_chars.len();
        let m = needle_chars.len();
        if n < m {
            return -1;
        }
        for index in 0..(n-m) {
            let mut a = index;
            let mut b = 0;
            while b < m && haystack_chars[a] == needle_chars[b] {
                a += 1;
                b += 1;
            }
            if b == m {
                return index as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        let case1 = Solution::str_str("sadbutsad".to_string(), "sad".to_string());
        assert_eq!(case1, 0);

        let case2 = Solution::str_str("leetcode".to_string(), "leeto".to_string());
        assert_eq!(case2, -1);

        let case3 = Solution::str_str("aaa".to_string(), "aaaa".to_string());
        assert_eq!(case3, -1);
    }
}