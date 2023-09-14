fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
         let left = Solution::binary_search(&nums, target, true);
         let right = Solution::binary_search(&nums, target, false) - 1;
         println!("left: {}, right: {}", left, right);
         if left <= right && right < nums.len() as i32 && nums[left as usize] == target && nums[right as usize] == target {
            return vec![left, right];
         }

         vec![-1, -1]
    }

    fn binary_search(nums: &Vec<i32>, target: i32, lower: bool) -> i32 {
        let mut start = 0;
        let mut end = nums.len() as i32 - 1;
        let mut ans = nums.len() as i32;
        while start <= end {
            let mid = start + (end - start) / 2;
            if nums[mid as usize] > target || (lower && nums[mid as usize] >= target) {
                end = mid - 1;
                ans = mid;
            } else {
                start = mid + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        let case1 = Solution::search_range(vec![5,7,7,8,8,10], 8);
        assert_eq!(case1, [3, 4]);
    }
}