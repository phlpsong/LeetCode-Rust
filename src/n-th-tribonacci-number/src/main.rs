struct Solution { }

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 {
            return 1;
        }
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        let mut res = 0;
        for index in 3..(n + 1) {
            res = a + b + c;
            a = b;
            b = c;
            c = res;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribonacci() {
        let case1 = Solution::tribonacci(4);
        assert_eq!(case1, 4);

        let case1 = Solution::tribonacci(25);
        assert_eq!(case1, 1389537);
    }
}