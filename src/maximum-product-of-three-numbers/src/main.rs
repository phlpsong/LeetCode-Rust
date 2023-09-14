use std::cmp;
fn main() {
    let res = Solution::maximum_product(vec![1, 2, 3, 4]);
    println!("res: {}", res);
}

struct Solution { }
// impl Solution {
//     pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
//         nums.sort();
//         let len = nums.len();
//         if len == 3 {
//             return nums[0] * nums[1] * nums[2];
//         }
//         return cmp::max(nums[0] * nums[1] * nums[len - 1], nums[len - 1] * nums[len - 2] * nums[len - 3]);
//     }
// }

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;

        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        let mut max3 = i32::MIN;

        for num in nums {
            if num < min1 {
                min2 = min1;
                min1 = num;
            } else if num < min2 {
                min2 = num;
            }

            if num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max3 = max2;
                max2 = num;
            } else if num > max3 {
                max3 = num;
            }
        }
        // println!("min1 {}, min2 {}, max1 {}, max2 {}, max3 {}", min1, min2, max1, max2, max3);
        return cmp::max(min1 * min2 * max1, max1 * max2 * max3);
    }
}