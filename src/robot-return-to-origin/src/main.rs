fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let char_vec: Vec<char> = moves.chars().collect();
        let mut horizon: i32 = 0;
        let mut vertical: i32 = 0;
        for c in char_vec {
            if c == 'U' {
                horizon += 1;
            }
            if c == 'D' {
                horizon -= 1;
            }
            if c == 'L' {
                vertical -= 1;
            }
            if c == 'R' {
                vertical += 1;
            }
        }
        return horizon == 0 && vertical == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_return_to_origin() {
        let case1 = Solution::judge_circle(String::from("UD"));
        assert_eq!(case1, true);

        let case2 = Solution::judge_circle(String::from("LL"));
        assert_eq!(case2, false);
    }
}
