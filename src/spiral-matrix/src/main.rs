struct Solution { }

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let rows = matrix.len() as i32;
        let columns = matrix[0].len() as i32;

        let mut left = 0;
        let mut right = columns - 1;
        let mut top = 0;
        let mut bottom = rows - 1;

        while left <= right && top <= bottom  {
            // top
            for column in left..(right + 1) {
                res.push(matrix[top as usize][column as usize]);
            }
            // right
            for row in (top + 1)..(bottom + 1) {
                res.push(matrix[row as usize][right as usize]);
            }

            if left < right && top < bottom {
                for column in ((left + 1)..right).rev() {
                    res.push(matrix[bottom as usize][column as usize]);
                }
                for row in ((top + 1)..(bottom + 1)).rev() {
                    res.push(matrix[row as usize][left as usize])
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
    use std::vec;

    use super::*;

    #[test]
    fn test_spiral_order() {
        let case1 = Solution::spiral_order(vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]]);
        assert_eq!(case1, vec![1,2,3,6,9,8,7,4,5]);

        let case2 = Solution::spiral_order(vec![vec![1,2,3,4], vec![5,6,7,8], vec![9,10,11,12]]);
        assert_eq!(case2, vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    }
}