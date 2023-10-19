fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = vec![];

        s.chars().for_each(|ch| {
            match stack.last() {
                Some(&val) if val == ch => {
                    stack.pop();
                }
                _ => stack.push(ch),
            }
        });
        stack.into_iter().collect()
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