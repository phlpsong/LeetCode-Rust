use std::collections::VecDeque;

fn main() {
    let res = Solution::is_valid(String::from("(])"));
    println!("res: {}", res);
}

struct Solution { }
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut deque = VecDeque::new();
        let chars: Vec<char> = s.chars().collect();
        for char in chars {
            if char == '(' || char == '[' || char == '{' {
                deque.push_front(char);
            } else {
                if let Some(val) = deque.front() {
                    if (char == ')' && *val == '(') || (char == ']' && *val == '[') || (char == '}' && *val == '{') {
                        deque.pop_front();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        if deque.is_empty() { return true; }
        false
    }
}