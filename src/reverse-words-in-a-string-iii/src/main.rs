fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res: Vec<String> = Vec::new();
        let str_vec: Vec<&str> = s.split(" ").collect();

        for item in str_vec {
            let reverse = item.chars().rev().collect();
            res.push(reverse);
        }
        res.join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let case1 = Solution::reverse_words("Let's take LeetCode contest".to_string());
        assert_eq!("s'teL ekat edoCteeL tsetnoc".to_string(), case1);

        let case2 = Solution::reverse_words("God Ding".to_string());
        assert_eq!("doG gniD".to_string(), case2);
    }
}