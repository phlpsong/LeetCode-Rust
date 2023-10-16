fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            if num.to_string().chars().count() % 2 == 0 {
                result += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_numbers() {
        let case1 = Solution::find_numbers(vec![12,345,2,6,7896]);
        assert_eq!(2, case1);

        let case2 = Solution::find_numbers(vec![555,901,482,1771]);
        assert_eq!(1, case2);
    }
}