struct Solution { }

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut roman = String::new();
        while num > 0 {
            match num {
                1000.. => {
                    num -= 1000;
                    roman.push_str("M");
                }
                900.. => {
                    num -= 900;
                    roman.push_str("CM");
                }
                500.. => {
                    num -= 500;
                    roman.push_str("D");
                }
                400.. => {
                    num -= 400;
                    roman.push_str("CD");
                }
                100.. => {
                    num -= 100;
                    roman.push_str("C");
                }
                90.. => {
                    num -= 90;
                    roman.push_str("XC");
                }
                50.. => {
                    num -= 50;
                    roman.push_str("L");
                }
                40.. => {
                    num -= 40;
                    roman.push_str("XL");
                }
                10.. => {
                    num -= 10;
                    roman.push_str("X");
                }
                9.. => {
                    num -= 9;
                    roman.push_str("IX");
                }
                5.. => {
                    num -= 5;
                    roman.push_str("V");
                }
                4.. => {
                    num -= 4;
                    roman.push_str("IV");
                }
                1.. => {
                    num -= 1;
                    roman.push_str("I")
                }
                _ => {}

            }
        }
        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        let case1 = Solution::int_to_roman(3);
        assert_eq!(case1, "III".to_string());

        let case2 = Solution::int_to_roman(4);
        assert_eq!(case2, "IV".to_string());

        let case3 = Solution::int_to_roman(9);
        assert_eq!(case3, "IX".to_string());

        let case4 = Solution::int_to_roman(58);
        assert_eq!(case4, "LVIII".to_string());

        let case5 = Solution::int_to_roman(1994);
        assert_eq!(case5, "MCMXCIV".to_string());
    }
}