fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        fn dfs(candidates: &Vec<i32>, curr: i32, target: i32, index: usize, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if curr == target {
                ans.push(tmp.to_vec());
                return;
            } else if curr > target {
                return;
            }
            for i in index..candidates.len() {
                tmp.push(candidates[i]);
                dfs(candidates, curr + candidates[i], target, i, tmp, ans);
                tmp.pop();
            }
        }

        let mut ans = Vec::with_capacity(15);
        dfs(&candidates, 0, target, 0, &mut Vec::with_capacity(64), &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum() {
        let case1 = Solution::combination_sum(vec![2,3,6,7], 7);
        let res1 = vec![vec![2,2,3], vec![7]];
        assert_eq!(case1, res1);
    }
}