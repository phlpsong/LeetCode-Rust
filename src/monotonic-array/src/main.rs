fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut is_increment = true;
        let mut is_decrement = true;
        let len = nums.len();
        if len == 0 { return false; }
        if len == 1 { return true; }
        for index in 1..len {
            if nums[index] < nums[index - 1] {
                is_increment = false;
            }
            if nums[index] > nums[index - 1] {
                is_decrement = false;
            }            
        }
        return is_decrement || is_increment;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_monotonic() {
        let case1 = Solution::is_monotonic(vec![1,2,2,3]);
        assert_eq!(true, case1);

        let case2 = Solution::is_monotonic(vec![6,5,4,4]);
        assert_eq!(true, case2);

        let case3 = Solution::is_monotonic(vec![1,3,2]);
        assert_eq!(false, case3);

        let case4 = Solution::is_monotonic(vec![1]);
        assert_eq!(true, case4);
    }
}