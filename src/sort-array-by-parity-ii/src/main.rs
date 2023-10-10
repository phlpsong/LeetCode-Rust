fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut even_nums: Vec<i32> = vec![];
        let mut odd_nums: Vec<i32> = vec![];
        for index in 0..nums.len() {
            if nums[index] % 2 == 0 {
                even_nums.push(nums[index]);
            } else {
                odd_nums.push(nums[index]);
            }
        }

        let mut res = vec![];
        let mut even_index = 0;
        let mut odd_index = 0;
        for index in 0..nums.len() {
            if index % 2 == 0 {
                res.push(even_nums[even_index]);
                even_index += 1;
            } else {
                res.push(odd_nums[odd_index]);
                odd_index += 1;
            }
        }
        return res;
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