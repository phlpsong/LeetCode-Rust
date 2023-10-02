fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
            // .collect();
        // let len: usize = chars.len();
        // println!("len: {}", len);
        // if len < 2 { return true; }
        // let mut start: usize = 0;
        // let mut end: usize = len - 1;
        // while start <= end {
        //     if chars[start] != chars[end] {
        //         return false;
        //     }
        //     start += 1;
        //     end -= 1;
        // }
        // use iterator
        while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
            if a != b {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peak_index_mountain_array() {
        let case1 = Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"));
        assert_eq!(case1, true);

        let case2 = Solution::is_palindrome(String::from("race a car"));
        assert_eq!(case2, false);

        let case3 = Solution::is_palindrome(String::from(" "));
        assert_eq!(case3, true);
    }
}