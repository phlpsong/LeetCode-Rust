fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = Vec::new();
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 0..m {
            let mut min = matrix[i][0];
            let mut min_idx = 0;
            for j in 1..n {
                if min > matrix[i][j] {
                    min = matrix[i][j];
                    min_idx = j;
                }
            }
            let mut max = min;
            for j in 0..m { max = max.max(matrix[j][min_idx]); }
            if min == max { ret.push(max); }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lucky_numbers() {
        let case1 = Solution::lucky_numbers(vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]]);
        assert_eq!(vec![15], case1);

        let case2 = Solution::lucky_numbers(vec![vec![1,10,4,2],vec![9,3,8,7],vec![15,16,17,12]]);
        assert_eq!(vec![12], case2);

        let case3 = Solution::lucky_numbers(vec![vec![7,8],vec![1,2]]);
        assert_eq!(vec![7], case3);
    }
}