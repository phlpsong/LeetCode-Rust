fn main() {
    println!("Hello, world!");
}

struct Solution {}

// impl Solution {
//     pub fn find_peak_element(nums: Vec<i32>) -> i32 {
//         if nums.len() == 1 {
//             return 0;
//         }
//         let mut start = 0;
//         let mut end = (nums.len() - 1) as i32;
//         while start <= end {
//             let mid = start + (end - start) / 2;
//             let mid_num = nums[mid as usize];
//             let left = mid - 1;
//             let right = mid + 1;

//             let mut great_than_left = false;
//             if left < 0 {
//                 great_than_left = true;
//             } else {
//                 great_than_left = mid_num > nums[left as usize];
//             }
//             let mut great_than_right = false;
//             if right >= nums.len() as i32 {
//                 great_than_right = true;
//             } else {
//                 great_than_right = mid_num > nums[right as usize];
//             }

//             if great_than_left && great_than_right {
//                 return mid;
//             } else if great_than_left && !great_than_right {
//                 start = mid + 1;
//             } else {
//                 end = mid - 1;
//             }
//         }
//         start
//     }
// }


impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut start = 0;
        let mut end = (nums.len() - 2) as i32;
        while start <= end {
            let mid = start + (end - start) / 2;
            let mid_num = nums[mid as usize];
            let right = mid + 1;
            if mid_num > nums[right as usize] {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_peak_element() {
        let case1 = Solution::find_peak_element(vec![1, 2, 3, 1]);
        assert_eq!(case1, 2);

        let case2 = Solution::find_peak_element(vec![1,2,1,3,5,6,4]);
        assert_eq!(case2, 5);

        let case3 = Solution::find_peak_element(vec![1,2,3]);
        assert_eq!(case3, 2);

        let case4 = Solution::find_peak_element(vec![1]);
        assert_eq!(case4, 0);

        let case5 = Solution::find_peak_element(vec![i32::MIN, i32::MIN]);
        assert_eq!(case5, 1);
    }
}