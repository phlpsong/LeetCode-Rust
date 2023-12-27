struct Solution { }

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut l = 0;
        let mut h = 0;
        for ch in s.chars().into_iter() {
            if ch == '(' {
                l += 1;
                h += 1;
            } else if ch == ')' {
                if l > 0 {
                    l -= 1;
                }
                h -= 1;
            } else if ch == '*' {
                if l > 0 {
                    l -= 1;
                }
                h += 1;
            }
            if h < 0 {
                return false;
            }
        }
        l == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_valid_string() {
        let case1 = Solution::check_valid_string("()".to_string());
        assert_eq!(true, case1);

        let case2 = Solution::check_valid_string("(*)".to_string());
        assert_eq!(true, case2);

        let case3 = Solution::check_valid_string("(*))".to_string());
        assert_eq!(true, case3);
    }
}