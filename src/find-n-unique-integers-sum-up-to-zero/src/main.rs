fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        
        for i in 1..=n/2 {
            ans.push(i);
            ans.push(-i);
        }

        if n % 2 == 0 {
            return ans
        }

        ans.push(0);

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_zero() {
        let case1 = Solution::sum_zero(5);
        assert_eq!(vec![1, -1, 2, -2, 0], case1);

        let case2 = Solution::sum_zero(3);
        assert_eq!(vec![1, -1, 0], case2);

        let case3 = Solution::sum_zero(1);
        assert_eq!(vec![0], case3);
    }
}