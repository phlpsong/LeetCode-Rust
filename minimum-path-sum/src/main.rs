use std::cmp;
fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as usize;
        let columns = grid[0].len() as usize;
        let mut res = vec![vec![0; columns]; rows];
        res[0][0] = grid[0][0];
        for row in 1..rows {
            res[row][0] = res[row - 1][0] + grid[row][0];
        }

        for column in 1..columns {
            res[0][column] = res[0][column - 1] + grid[0][column];
        }

        for row in 1..rows {
            for column in 1..columns {
                res[row][column] = cmp::min(res[row - 1][column], res[row][column - 1]) + grid[row][column];
            }
        }

        res[rows - 1][columns - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        let case1 = Solution::min_path_sum(vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]]);
        assert_eq!(case1, 7);

        let case2 = Solution::min_path_sum(vec![vec![1,2,3], vec![4,5,6]]);
        assert_eq!(case2, 12);
    }
}