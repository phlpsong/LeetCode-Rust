use std::collections::HashSet;

fn main() {
    let res = Solution::is_unique(String::from("abcd"));
    println!("res: {}", res);
}

struct Solution { }

impl Solution {
    pub fn is_unique(astr: String) -> bool {
        let arr: Vec<char> = astr.chars().collect();
        let mut set = HashSet::new();
        for char in arr {
            if set.contains(&char) {
                return false;
            }
            set.insert(char);
        }
        true
    }
}