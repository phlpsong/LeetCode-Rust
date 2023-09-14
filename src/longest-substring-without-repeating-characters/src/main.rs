use std::collections::{HashSet, HashMap};
use std::cmp;

fn main() {
    let res = Solution::length_of_longest_substring(String::from("pwwkew"));
    println!("res: {}", res);
}

struct Solution { }

impl Solution {
    // pub fn length_of_longest_substring(s: String) -> i32 {
    //     let chars: Vec<char> = s.chars().collect();
    //     let mut max = 0;
    //     let mut set: HashSet<char> = HashSet::new();
    //     let mut start = 0;
    //     let mut end = start;
    //     while start < chars.len() {
    //         println!("statrt: {}, end: {}, set: {:?}", start, end, set);
    //         if end < chars.len() && !set.contains(&chars[end]) {
    //             set.insert(chars[end]);
    //             end += 1;
    //         } else {
    //             max = cmp::max(max, end - start);
    //             set.clear();
    //             start += 1;
    //             end = start;
    //         }
    //     }

        
    //     max as i32
    // }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut res = 0;
        for (index, char) in chars.iter().enumerate() {
            if let Some(val) = map.get(&char) {
                left = cmp::max(left, *val + 1);
            }
            map.insert(char, index);
            res = cmp::max(res, index - left + 1);
        }

        res as i32
    }
    
}