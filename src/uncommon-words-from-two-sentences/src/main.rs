use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = HashMap::new();
        let vec_s1 = s1.split(" ");
        let vec_s2 = s2.split(" ");
        for str in vec_s1 {
            let entry = map.entry(str).or_insert((0, 0));
            entry.0 = entry.0 + 1;
        }

        for str in vec_s2 {
            let entry = map.entry(str).or_insert((0, 0));
            entry.1 = entry.1 + 1;
        }

        let mut vec = vec![];
        for (key, value) in map.iter() {
            if (value.0 == 1 && value.1 == 0) || (value.0 == 0 && value.1 == 1) {
                vec.push(key.to_string());
               }
         }

         vec
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uncommon_from_sentences() {
        let case1 = Solution::uncommon_from_sentences("this apple is sweet".to_string(), "this apple is sour".to_string());
        assert_eq!(vec!["sweet".to_string(), "sour".to_string()], case1);

        let case2 = Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string());
        assert_eq!(vec!["banana".to_string()], case2);
    }
}