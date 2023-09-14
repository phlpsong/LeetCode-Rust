struct Solution { }

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut res =  vec![vec![0; n as usize]; m as usize];
        for row in 0..m {
            res[row as usize][0] = 1;
        }

        for column in 0..n {
            res[0][column as usize] = 1;
        }

        println!("arr: {:?}", res);

        for row in 1..m {
            for column in 1..n {
                let row = row as usize;
                let column = column as usize;
                res[row][column] = res[row - 1][column] + res[row][column - 1]; 
            }
        }

        res[(m - 1) as usize][(n - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        let case1 = Solution::unique_paths(3, 7);
        assert_eq!(case1, 28);

        let case2 = Solution::unique_paths(3, 2);
        assert_eq!(case2, 3);
    }
}