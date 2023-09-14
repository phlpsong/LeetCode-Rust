fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut left_max_num = nums[0];
        let mut right_min = Vec::new();

        let mut min = nums[nums.len() - 1];
        for index in (0..nums.len()).rev() {
            if nums[index] < min {
                min = nums[index];
            }
            right_min.push(min);
        }
        right_min.reverse();

        let mut index = 0;
        let mut res = index;
        while (index + 1) < nums.len() {
            if nums[index] > left_max_num {
                left_max_num = nums[index];
            }
            if left_max_num <= right_min[index + 1] {
                res = index;
                break;
            }
            index += 1;
        }

        (res + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_disjoint() {
        let case1 = Solution::partition_disjoint(vec![5, 0, 3, 8, 6]);
        assert_eq!(case1, 3);

        let case2 = Solution::partition_disjoint(vec![1,1,1,0,6,12]);
        assert_eq!(case2, 4);

        let case3 = Solution::partition_disjoint(vec![1,1]);
        assert_eq!(case3, 1);

        let case4 = Solution::partition_disjoint(vec![6,0,8,30,37,6,75,98,39,90,63,74,52,92,64]);
        assert_eq!(case4, 2);
    }
}