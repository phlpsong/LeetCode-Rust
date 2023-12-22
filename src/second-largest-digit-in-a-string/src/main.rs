use std::collections::HashSet;

struct Solution { }

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut highest = i32::MIN;
        let mut second_highest = i32::MIN;
        let mut hs = HashSet::new();
        for c in s.chars() {
            if c.is_digit(10) {
                let n = c.to_digit(10).unwrap() as i32;
                if !hs.contains(&n) {
                    hs.insert(n);
                    if n > highest {
                        second_highest = highest;
                        highest = n;
                    } else if n > second_highest {
                        second_highest = n;
                    }
                }
            }
        }
        if hs.len() < 2 {
            return -1;
        }
        second_highest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_second_highest() {
        let case1 = Solution::second_highest(String::from("dfa12321afd"));
        assert_eq!(2, case1);

        let case2 = Solution::second_highest(String::from("abc1111"));
        assert_eq!(-1, case2);
    }
}