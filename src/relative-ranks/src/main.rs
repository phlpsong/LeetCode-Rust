fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut ans = vec!["".to_string(); score.len()];
        let mut map = score.iter().enumerate().collect::<Vec<_>>();
        map.sort_unstable_by(|a, b| b.1.cmp(a.1));
        map.iter().enumerate().for_each(|i| {
            ans[i.1 .0] = match i.0 {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i.0 + 1).to_string(),
            }
        });
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_relative_ranks() {
        let case1 = Solution::find_relative_ranks(vec![5,4,3,2,1]);
        assert_eq!(vec!["Gold Medal","Silver Medal","Bronze Medal","4","5"], case1);

        let case2 = Solution::find_relative_ranks(vec![10,3,8,9,4]);
        assert_eq!(vec!["Gold Medal","5","Bronze Medal","Silver Medal","4"], case2);
    }
}