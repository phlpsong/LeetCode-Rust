use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for index in 0..strs.len() {
            let mut chars: Vec<char> = strs[index].chars().collect();
            chars.sort();
            let key: String = chars.into_iter().collect();
            let entry = map.entry(key).or_insert(vec![]);
            entry.push(strs[index].clone());
        }

        map.into_iter().map(|(key, val)| val).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let case1 = Solution::group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]);
        assert_eq!(3, case1.len());

        let case2 = Solution::group_anagrams(vec!["".to_string()]);
        assert_eq!(vec![vec!["".to_string()]], case2);

        let case3 = Solution::group_anagrams(vec!["a".to_string()]);
        assert_eq!(vec![vec!["a".to_string()]], case3);
    }
}