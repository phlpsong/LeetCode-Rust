fn main() {
    println!("Hello, world!");
}

struct Solution { }
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start: i32 = 0;
        let mut end = nums.len() as i32 - 1;
        while start <= end {
            let mid = start + (end - start) / 2;
            let mid_num = nums[mid as usize];
            if mid_num > target {
                end = mid - 1;
            } else if mid_num < target {
                start = mid + 1;
            } else {
                return mid as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let case1 = Solution::search(vec![-1,0,3,5,9,12], 9);
        assert_eq!(case1, 4);

        let case2 = Solution::search(vec![-1,0,3,5,9,12], 2);
        assert_eq!(case2, -1);

        let case3 = Solution::search(vec![-1,0,3,5,9,12, 13], 13);
        assert_eq!(case3, 6);

        let case4 = Solution::search(vec![5], -5);
        assert_eq!(case4, -1);
    }
}