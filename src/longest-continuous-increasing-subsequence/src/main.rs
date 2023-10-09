fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max_len = 1;
        for index in 0..nums.len() {
            let mut curr = nums[index];
            let mut length = 1;
            let mut next = index + 1;
            while next < nums.len() && nums[next] > curr {
                length += 1;
                curr = nums[next];
                next += 1;

                max_len = max_len.max(length);
            }
        }

        return max_len;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_length_of_lcis() {
        let case1 = Solution::find_length_of_lcis(vec![1,3,5,4,7]);
        assert_eq!(3, case1);

        let case2 = Solution::find_length_of_lcis(vec![2,2,2,2,2]);
        assert_eq!(1, case2);
    }
}