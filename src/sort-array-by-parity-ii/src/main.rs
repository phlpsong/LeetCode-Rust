fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut next = 1;

        for index in (0..n).step_by(2) {
            if nums[index] % 2 == 1 {
                while nums[next] % 2 == 1 {
                    next += 2;
                }
                nums.swap(index, next);
            }
        }
        return nums;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_array_by_parity_ii() {
        let case1 = Solution::sort_array_by_parity_ii(vec![4,2,5,7]);
        assert_eq!(vec![4, 5, 2, 7], case1);

        let case2 = Solution::sort_array_by_parity_ii(vec![2, 3]);
        assert_eq!(vec![2, 3], case2);
    }
}