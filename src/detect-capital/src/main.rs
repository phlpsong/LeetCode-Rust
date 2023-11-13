fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut cnt = 0;
        let len = word.len();

        for c in word.chars() {
            if c.is_uppercase() {
                cnt += 1;
            }
        }
        let ch = word.chars().next().unwrap();
        if cnt == len || cnt == 0 || (cnt == 1 && ch.is_uppercase()) {
            return true;
        } 
        false
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_capital_use() {
        let case1 = Solution::detect_capital_use("USA".to_string());
        assert_eq!(true, case1);

        let case2 = Solution::detect_capital_use("FlaG".to_string());
        assert_eq!(false, case2);
    }
}
