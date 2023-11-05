use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();
        for char in magazine.chars() {
            let entry = map.entry(char).or_insert(0);
            *entry += 1;
        }
        for char in ransom_note.chars() {
            println!("char: {}, map: {:?}", char, map);
            if let Some(&val) = map.get(&char) {
                if val == 1 {
                    map.remove(&char);
                } else {
                    map.insert(char, val - 1);
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_construct() {
        let case1 = Solution::can_construct("a".to_string(), "b".to_string());
        assert_eq!(false, case1);

        let case2 = Solution::can_construct("aa".to_string(), "ab".to_string());
        assert_eq!(false, case2);

        let case3 = Solution::can_construct("aa".to_string(), "aab".to_string());
        assert_eq!(true, case3);
    }
}