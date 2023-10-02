fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut vec = vec![0; (row_index + 1) as usize];
        vec[0] = 1;
        for i in 1..(row_index + 1) {
            println!("{}", i);
            for j in (1..(i + 1)).rev() {
                println!("{}, {}", i, j);
                vec[j as usize] += vec[(j - 1) as usize];
            }
        }
        return vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascals_triangle_ii() {
        let case1 = Solution::get_row(5);
        assert_eq!(case1, vec![1, 5, 10, 10, 5, 1]);

        let case2 = Solution::get_row(4);
        assert_eq!(case2, vec![1, 4, 6, 4, 1]);

        let case3 = Solution::get_row(3);
        assert_eq!(case3, vec![1, 3, 3, 1]);
    }
}