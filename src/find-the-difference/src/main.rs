use std::collections::{HashSet, HashMap};

fn main() {
    let res = Solution::find_the_difference(String::from("abcd"), String::from("abcde"));
    println!("res: {}", res);
}

struct Solution { }

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let chars1: Vec<char> = s.chars().collect();
        let chars2: Vec<char> = t.chars().collect();

        let mut map = HashMap::new();
        for char in &chars1 {
            let count = map.entry(char).or_insert(0);
            *count += 1;
        }
        let mut res: char = char::default();
        for char in &chars2 {
            if let Some(val) = map.get(char) {
                let new_val = val - 1;
                if new_val == 0 {
                    map.remove(char);
                } else {
                    map.insert(char, new_val);
                }
            } else {
                res = *char;
                break;
            }
        }
        res
    }
}