use std::cmp::min;

fn main() {
    println!("Hello, world!");
}

struct Solution {

}

// impl Solution {
//     pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
//         let mut count = 0;
//         let mut min_size = -1;
//         let mut start = 0;
//         let mut end = 0;
//         while start < nums.len() && end < nums.len() {
//             count += nums[end];
//             if count >= target {
//                 let curr_size: i32 = (end - start) as i32;
//                 if min_size == -1 {
//                     min_size = curr_size + 1;
//                 } else {
//                     min_size = min(min_size, curr_size + 1);
//                 }
//                 count -= nums[start];
//                 start += 1;
//                 count -= nums[end];
//             } else {
//                 end += 1;
//             }
//         }

//         if min_size == -1 {
//             return 0;
//         }
//         min_size
//     }
// }

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let size = nums.len();
        if size == 0 { return 0;}
        let mut ans = i32::MAX;
        let mut start = 0;
        let mut end = 0;
        let mut sum = 0;

        while end < size {
            sum += nums[end];
            while sum >= target {
                let curr = (end - start) as i32;
                ans = min(ans, curr + 1);
                sum -= nums[start];
                start += 1;
            }
            end += 1
        }
        if ans == i32::MAX {
            return 0;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let case1 = Solution::min_sub_array_len(7, vec![2, 3, 1, 2 , 4, 3]);
        assert_eq!(case1, 2);

        let case2 = Solution::min_sub_array_len(4, vec![1, 4, 4]);
        assert_eq!(case2, 1);

        let case3 = Solution::min_sub_array_len(11, vec![1, 4, 4]);
        assert_eq!(case3, 0);

        let case4 = Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]);
        assert_eq!(case4, 3);
    }
}