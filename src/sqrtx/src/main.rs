fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut start = 0;
        let mut end = x;
        let mut ans = 0;
        while start <= end {
            let mid = start + (end - start) / 2;
            let product = mid as i64 * mid as i64;
            if product == x as i64 {
                return mid as i32;
            } else if product > x as i64 {
                end = mid - 1;
            } else {
                ans = mid;
                start = mid + 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        let case1 = Solution::my_sqrt(4);
        assert_eq!(case1, 2);

        let case2 = Solution::my_sqrt(0);
        assert_eq!(case2, 0);

        let case3 = Solution::my_sqrt(8);
        assert_eq!(case3, 2);

        let case4 = Solution::my_sqrt(i32::MAX);
        assert_eq!(case4, 46340);

        let case5 = Solution::my_sqrt(1);
        assert_eq!(case5, 1);
    }
}