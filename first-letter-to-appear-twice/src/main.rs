use std::collections::HashSet;

fn main() {
    let res = Solution::repeated_character(String::from("abcdd"));
    println!("res: {}", res);
}

struct Solution { }

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let chars: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();
        for char in chars {
            if set.contains(&char) {
                return char;
            }
            set.insert(char);
        }
        '\0'
    }
}