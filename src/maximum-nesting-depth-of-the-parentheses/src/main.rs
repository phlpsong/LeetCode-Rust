fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut stack = vec![];
        let mut res: i32 = 0;

        let mut curr: i32 = 0;
        s.chars().for_each(|ch| {
            
            if ch == '(' {
                stack.push(ch);
                curr = stack.len() as i32;
                res = res.max(curr);
            } else {
                if !stack.is_empty() {
                    if let Some(&val) = stack.last() {
                        if ch == ')' {
                            stack.pop();
                        }
                    }
                } else {
                    curr = 0;
                }
            }
        });
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_depth() {
        let case1 = Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string());
        assert_eq!(3, case1);

        let case2 = Solution::max_depth("(1)+((2))+(((3)))".to_string());
        assert_eq!(3, case2);
    }
}