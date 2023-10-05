fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        let mut res = vec![0; len];
        let mut start: usize = 0;
        let mut end: usize = len - 1;
        for index in 0..len {
            if nums[index] % 2 == 0 {
                res[start] = nums[index];
                start += 1;
            } else {
                res[end] = nums[index];
                if end > 0 {
                    end -= 1;
                }
            }
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_by_parity() {
        let case1 = Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
        assert_eq!(vec![2, 4, 1, 3], case1);

        let case2 = Solution::sort_array_by_parity(vec![0]);
        assert_eq!(vec![0], case2);

        let case3 = Solution::sort_array_by_parity(vec![1, 3]);
        assert_eq!(vec![3, 1], case3);

        let case4 = Solution::sort_array_by_parity(vec![0, 1]);
        assert_eq!(vec![0, 1], case4);
    }
}