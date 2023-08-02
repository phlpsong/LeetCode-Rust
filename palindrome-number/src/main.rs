struct Solution { }

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut num = x;
        let mut reverse_num = 0;
        while num > reverse_num {
            reverse_num = reverse_num * 10 + num % 10;
            num = (num - num % 10) / 10;
        }

        return num == reverse_num || num == reverse_num / 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrom() {
        let case1 = Solution::is_palindrome(121);
        assert_eq!(case1, true);

        let case2 = Solution::is_palindrome(-121);
        assert_eq!(case2, false);

        let case3 = Solution::is_palindrome(10);
        assert_eq!(case3, false);
    }
}