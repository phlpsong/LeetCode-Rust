struct Solution { }

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut prev = 0;
        s.chars().for_each(|ch| {
            let curr = match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0
            };
            if curr > prev {
                res -= prev;
                res = res + curr - prev;
            } else {
                res += curr;
            }
            prev = curr;
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let case1 = Solution::roman_to_int("III".to_string());
        assert_eq!(case1, 3);

        let case2 = Solution::roman_to_int("IV".to_string());
        assert_eq!(case2, 4);

        let case3 = Solution::roman_to_int("IX".to_string());
        assert_eq!(case3, 9);

        let case4 = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(case4, 58);

        let case5 = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(case5, 1994);
    }
}