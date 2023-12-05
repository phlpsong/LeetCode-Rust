struct Solution { }

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_win_nim() {
        let case1 = Solution::can_win_nim(4);
        assert_eq!(case1, false);

        let case2 = Solution::can_win_nim(1);
        assert_eq!(case2, true);

        let case3 = Solution::can_win_nim(2);
        assert_eq!(case3, true);
    }
}