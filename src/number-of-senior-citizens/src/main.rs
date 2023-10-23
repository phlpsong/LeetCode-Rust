fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut ans = 0;

        for s in details.iter() {
            if let Ok(age) = s[11..13].parse::<i32>() {
                if age > 60 {
                    ans += 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_seniors() {
        let case1 = Solution::count_seniors(vec!["7868190130M7522".to_string(),"5303914400F9211".to_string(),"9273338290F4010".to_string()]);
        assert_eq!(2, case1);

        let case2 = Solution::count_seniors(vec!["1313579440F2036".to_string(),"2921522980M5644".to_string()]);
        assert_eq!(0, case2);
    }
}