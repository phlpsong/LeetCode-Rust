fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let (mut l, mut r) = (2, 2);
        while r < nums.len() {
            if nums[l - 2] != nums[r] {
                nums[l] = nums[r];
                l += 1;
            }
            r += 1;
        }
        l as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums1 = vec![1,1,1,2,2,3];
        let case1 = Solution::remove_duplicates(&mut nums1);
        assert_eq!(5, case1);

        let mut nums2 = vec![0,0,1,1,1,1,2,3,3];
        let case2 = Solution::remove_duplicates(&mut nums2);
        assert_eq!(7, case2);
    }
}