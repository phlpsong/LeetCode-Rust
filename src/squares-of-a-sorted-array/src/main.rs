fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len() as i32;
        let mut left: i32 = 0;
        let mut right: i32 = size - 1;
        let mut res = vec![0; nums.len()];
        let mut curr_index = nums.len();
        while left <= right {
            curr_index -= 1;
            if nums[left as usize] + nums[right as usize] < 0 {
                res[curr_index] = nums[left as usize] * nums[left as usize];
                println!("left: {}", left);
                left += 1;
            } else {
                res[curr_index] = nums[right as usize] * nums[right as usize];
                println!("right: {}", right);
                right -= 1;
            }
        }

        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        // let case1 = Solution::sorted_squares(vec![-4,-1,0,3,10]);
        // assert_eq!(vec![0,1,9,16,100], case1);

        // let case2 = Solution::sorted_squares(vec![-7,-3,2,3,11]);
        // assert_eq!(vec![4,9,9,49,121], case2);

        let case3 = Solution::sorted_squares(vec![1]);
        assert_eq!(vec![1], case3);
    }
}