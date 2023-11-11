fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n < 1 {
            return false;
        }
        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        } 
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_ugly() {
        let case1 = Solution::is_ugly(6);
        assert_eq!(true, case1);

        let case2 = Solution::is_ugly(1);
        assert_eq!(true, case2);

        let case3 = Solution::is_ugly(14);
        assert_eq!(false, case3);
    }
}