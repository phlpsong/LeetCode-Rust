fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut arr = vec![0; size];

        for index in 0..size {
            let num = nums[index] as usize;
            arr[num - 1] = arr[num - 1] + 1;
        }

        let mut res = vec![];
        for index in 0..arr.len() {
            if arr[index] == 2 {
                res.push((index + 1) as i32);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        let case1 = Solution::find_duplicates(vec![4,3,2,7,8,2,3,1]);
        assert_eq!(vec![2, 3], case1);

        let case2 = Solution::find_duplicates(vec![1,1,2]);
        assert_eq!(vec![1], case2);

        let case3 = Solution::find_duplicates(vec![1]);
        assert_eq!(0, case3.len());
    }
}