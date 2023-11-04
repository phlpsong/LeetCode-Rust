use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut n = n;
        let mut new_n: i32;
        while !set.contains(&n) {
            set.insert(n);
            new_n = 0;
            while n > 0 {
                new_n += (n % 10).pow(2);
                n /= 10;
            }
            n = new_n;
        }
        n == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_happy() {
        let case1 = Solution::is_happy(19);
        assert_eq!(true, case1);

        let case2 = Solution::is_happy(2);
        assert_eq!(false, case2);
    }
}