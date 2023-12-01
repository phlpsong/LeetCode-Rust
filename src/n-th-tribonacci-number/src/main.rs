struct Solution { }

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 {
            return 1;
        }
        let mut vec = vec![0; (n + 1) as usize];
        vec[0] = 0;
        vec[1] = 1;
        vec[2] = 1;
        for index in 3..(n + 1) {
            vec[index as usize] = vec[(index - 1) as usize] + vec[(index - 2) as usize] + vec[(index - 3) as usize];
        }

        vec[n as usize]
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