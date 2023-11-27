fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; nums.len()];
        for &num in nums.iter() {
            count[num as usize] = count[num as usize] + 1;
        }

        for index in 1..count.len() {
            if count[index] > 1 {
                return index as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        let case1 = Solution::find_duplicate(vec![1,3,4,2,2]);
        assert_eq!(2, case1);

        let case2 = Solution::find_duplicate(vec![3,1,3,4,2]);
        assert_eq!(3, case2);

        let case3 = Solution::find_duplicate(vec![2,2,2,2,2]);
        assert_eq!(2, case3);
    }
}