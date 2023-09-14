use std::collections::HashMap;

fn main() {
    let res = Solution::check_permutation(String::from("abcb"), String::from("cba"));
    println!("res: {}", res);
}

struct Solution { }
impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let mut map = HashMap::new();
        for char in s1.chars() {
            if let Some(&value) = map.get(&char) {
                map.insert(char, value + 1);
            } else {
                map.insert(char, 1);
            }
        }

        for char in s2.chars() {
            if let Some(&value) = map.get(&char) {
                if value == 1 {
                    map.remove(&char);
                    continue;
                }
                map.insert(char, value - 1);
            } else {
                return false;
            }
        }
        if !map.is_empty() { return false; }
        true
    }
}