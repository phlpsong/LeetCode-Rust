fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut vec: Vec<char> = vec![];
        let chars: Vec<char> = s.chars().into_iter().collect();
        for char in chars {
            if !vec.is_empty() {
                if let Some(&val) = vec.last() {
                    if val == char {
                        vec.pop();
                    } else {
                        vec.push(char);
                    }
                } 
            } else {
                vec.push(char);
            }
        }
        let res: String = vec.into_iter().collect();
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let case1 = Solution::remove_duplicates("abbaca".to_string());
        assert_eq!("ca".to_string(), case1);
    }
}