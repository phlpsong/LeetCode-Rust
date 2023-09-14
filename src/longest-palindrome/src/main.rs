use std::collections::HashMap;

fn main() {
    let res = Solution::longest_palindrome(String::from("bb"));
    println!("res: {}", res);
}

struct Solution { }
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut map = HashMap::new();
        for char in chars {
            let count = map.entry(char).or_insert(0);
            *count += 1;
        }

        let mut res = 0;
        let mut odd = 0;
        for (key, value) in &map {
            let count = value / 2;
            let remain = value % 2;
            println!("key: {}, count: {}", key, value);
            res += count - remain;
            if remain == 1 { 
                odd = 1;
            }
        }
        res + odd
    }
}