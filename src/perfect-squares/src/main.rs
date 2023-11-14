fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![i32::MAX - 1; n as usize + 1];
        let mut j = 1usize;
        dp[0] = 0;
        while j * j <= n as usize {
            (j * j..=n as usize).for_each(|i| {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
            });
            j += 1;
        }
        dp[n as usize]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_squares() {
        let case1 = Solution::num_squares(12);
        assert_eq!(3, case1);

        let case2 = Solution::num_squares(13);
        assert_eq!(2, case2);
    }
}