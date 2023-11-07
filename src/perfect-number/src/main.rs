fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        
        let mut i = 2;
        let mut sum = 1;

        while i * i <= num {
            if num % i == 0 {
                sum += i;
                sum += num / i;
            }

            i += 1;
        }

        sum == num
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_perfect_number() {
        let case1 = Solution::check_perfect_number(28);
        assert_eq!(true, case1);

        let case2 = Solution::check_perfect_number(7);
        assert_eq!(false, case2);
    }
}
