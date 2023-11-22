fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_index = 0;
        let mut t_index = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        while s_index < s_chars.len() && t_index < t_chars.len() {
            if t_chars[t_index as usize] != s_chars[s_index as usize] {
                t_index += 1;
                continue;
            }
            s_index += 1;
            t_index += 1;
        }
        s_index >= s_chars.len()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        // let case1 = Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());
        // assert_eq!(true, case1);

        let case2 = Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string());
        assert_eq!(false, case2);
    }
}