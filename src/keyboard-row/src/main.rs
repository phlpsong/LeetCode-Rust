use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let map: HashMap<char, i32> = HashMap::from([
            ('q', 0), ('w', 0), ('e', 0), ('r', 0), ('t', 0), ('y', 0), ('u', 0), ('i', 0), ('o', 0), ('p', 0),
            ('a', 1), ('s', 1), ('d', 1), ('f', 1), ('g', 1), ('h', 1), ('j', 1), ('k', 1), ('l', 1),
            ('z', 2), ('x', 2), ('c', 2), ('v', 2), ('b', 2), ('n', 2), ('m', 2),
        ]);
        
        let mut res: Vec<String> = vec![];
        for index in 0..words.len() {
            let word = &words[index];
            let chars: Vec<char> = word.to_lowercase().chars().collect();
            
            let mut is_same_line = true;

            let mut out_line = 0;
            if let Some(&val) = map.get(&chars[0]) {
                out_line = val;
            }
            for char_index in 1..chars.len() {
                if let Some(&val) = map.get(&chars[char_index]) {
                    if out_line == val {
                        continue;
                    } else {
                        is_same_line = false;
                        break;
                    }
                }
            }
            if is_same_line {
                res.push(words[index].clone());
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_words() {
        let case1 = Solution::find_words(vec!["Hello".to_string(), "Alaska".to_string(),"Dad".to_string(),"Peace".to_string()]);
        assert_eq!(vec!["Alaska".to_string(), "Dad".to_string()], case1);

        let case2 = Solution::find_words(vec!["omk".to_string()]);
        assert_eq!(vec![] as Vec<String>, case2);

        let case2 = Solution::find_words(vec!["adsdf".to_string(), "sfd".to_string()]);
        assert_eq!(vec!["adsdf".to_string(), "sfd".to_string()], case2);
    }
}