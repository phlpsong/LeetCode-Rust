fn main() {
    println!("Hello, world!");
}

struct Solution { }


impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.chunks(2).map(|v| v[0]).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_pair_sum() {
        let case1 = Solution::array_pair_sum(vec![1,4,3,2]);
        assert_eq!(4, case1);

        let case2 = Solution::array_pair_sum(vec![6,2,6,5,1,2]);
        assert_eq!(9, case2);
    }
}