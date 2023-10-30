fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stone_game() {
        let case1 = Solution::stone_game(vec![5,3,4,5]);
        assert_eq!(true, case1);

        let case2 = Solution::stone_game(vec![3,7,2,3]);
        assert_eq!(true, case2);
    }
}