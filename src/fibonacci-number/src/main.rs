fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        let mut cur: usize = 3;
        let mut res = vec![0, 1, 1];
        while cur <= (n as usize) {
            res.push(res[cur - 1] + res[cur -2]);
            cur += 1;
        } 
        return res[n as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let case1 = Solution::fib(30);
        assert_eq!(case1, 832040);

        let case2 = Solution::fib(29);
        assert_eq!(case2, 514229);

        let case2 = Solution::fib(5);
        assert_eq!(case2, 5);
    }
}
