use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
}

struct Solution { }
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut start = 0;
        let mut end = 0;
        let mut count = 0;
        
        let chars1: Vec<char> = s1.chars().collect();
        let size = chars1.len();

        let mut map1: HashMap<char, i32> = HashMap::new();
        let mut map2: HashMap<char, i32> = HashMap::new();

        for char in chars1 {
            let value = map1.entry(char).or_insert(0);
            *value += 1;
        }


        let chars2: Vec<char> = s2.chars().collect();
        while end < chars2.len() {
            let char = chars2[end];
            let value = map2.entry(char).or_insert(0);
            *value += 1;
            
            count += 1;

            while count == size {
                println!("map1: {:?}, map2: {:?}", map1, map2);
                if map1 == map2 {
                    return true;
                }
                let start_char = chars2[start];

                let start_val = map2.entry(start_char).or_insert(0);
                *start_val -= 1;
                if *start_val == 0 {
                    map2.remove(&start_char);
                }
                count -= 1;
                
                start += 1;
            }
            end += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include() {
        let case1 = Solution::check_inclusion(String::from("ab"), String::from("eidbaooo"));
        assert_eq!(case1, true);

        let case2 = Solution::check_inclusion(String::from("ab"), String::from("eidboaoo"));
        assert_eq!(case2, false);

        let case3 = Solution::check_inclusion(String::from("a"), String::from("eidbooo"));
        assert_eq!(case3, false);

        let case4 = Solution::check_inclusion(String::from("oobo"), String::from("eidbooo"));
        assert_eq!(case4, true);
    }
}