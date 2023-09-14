fn main() {
    println!("Hello, world!");
}
struct Solution { }

// impl Solution {
//     pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
//         let mut set = HashSet::new();
//         let size = nums.len() as i32;
//         for index in nums.iter() {
//             set.insert(*index);
//         }
//         let mut res = Vec::new();
//         for index in 1..(size + 1) {
//             if set.contains(&index) {
//                 continue;
//             } else {
//                 res.push(index);
//             }
//         }
//         res
//     }
// }

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len() as i32;
        for index in 0..size {
            let x = (nums[index as usize] - 1) % size;
            nums[x as usize] = nums[x as usize] + size;
        }
        println!("nums: {:?}", nums);
        let mut res = Vec::new();
        for index in 0..size {
            if nums[index as usize] <= size {
                res.push(index + 1);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        let case1 = Solution::find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]);
        assert_eq!(case1, vec![5, 6]);

        let case2 = Solution::find_disappeared_numbers(vec![1, 1]);
        assert_eq!(case2, vec![2]);
    }
}

