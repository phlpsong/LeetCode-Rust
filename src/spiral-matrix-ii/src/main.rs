fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let len = n as usize;
        let mut res = vec![vec![0; len]; len];
        let mut top = 0;
        let mut right = n - 1;
        let mut left = 0;
        let mut bottom = n - 1;
        let mut value = 1;
        while left <= right && top <= bottom {
            for column in left..(right + 1) {
                res[top as usize][column as usize] = value;
                value += 1;
            }

            for row in (top + 1)..(bottom + 1) {
                res[row as usize][right as usize] = value;
                value += 1;
            }

            if left < right && top < bottom {
                for column in (left..right).rev() {
                    res[bottom as usize][column as usize] = value;
                    value += 1;
                }

                for row in ((top + 1)..bottom).rev() {
                    res[row as usize][left as usize] = value;
                    value += 1;
                }
            }

            left += 1;
            right -= 1;
            top += 1;
            bottom -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_matrix() {
        let case1 = Solution::generate_matrix(3);
        assert_eq!(case1, vec![vec![1,2,3], vec![8,9,4], vec![7,6,5]]);

        let case2 = Solution::generate_matrix(1);
        assert_eq!(case2, vec![vec![1]]);
    }
}