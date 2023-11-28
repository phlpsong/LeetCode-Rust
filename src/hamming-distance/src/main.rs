fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        let case1 = Solution::hamming_distance(1, 4);
        assert_eq!(2, case1);

        let case2 = Solution::hamming_distance(3, 1);
        assert_eq!(1, case2);
    }
}