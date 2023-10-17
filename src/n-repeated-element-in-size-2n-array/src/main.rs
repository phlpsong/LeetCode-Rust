fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut cache = vec![0; 10000];
        for num in &nums {
            if cache[*num as usize] == 1 { return *num; }
            cache[*num as usize] += 1;
        }
        nums[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repeated_n_times() {
        let case1 = Solution::repeated_n_times(vec![1,2,3,3]);
        assert_eq!(3, case1);

        let case2 = Solution::repeated_n_times(vec![2,1,2,5,3,2]);
        assert_eq!(2, case2);

        let case3 = Solution::repeated_n_times(vec![5,1,5,2,5,3,5,4]);
        assert_eq!(5, case3);
    }
}