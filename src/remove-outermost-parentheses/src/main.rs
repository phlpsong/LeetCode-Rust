fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut res: Vec<char> = Vec::new();
        let mut stack = Vec::new();
        for ch in s.chars() {
            if ch == ')' {
                stack.pop();
            }
            if stack.len() > 0 {
                res.push(ch);
            } 
            if ch == '(' {
                stack.push(ch);
            }
        }

        let s: String = res.into_iter().collect();
        s
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_outer_parentheses() {
        let case1 = Solution::remove_outer_parentheses("(()())(())".to_string());
        assert_eq!("()()()".to_string(), case1);

        let case2 = Solution::remove_outer_parentheses("(()())(())(()(()))".to_string());
        assert_eq!("()()()()(())".to_string(), case2);

        let case3 = Solution::remove_outer_parentheses("()()".to_string());
        assert_eq!("".to_string(), case3);
    }
}