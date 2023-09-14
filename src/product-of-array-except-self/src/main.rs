fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left_res = Vec::new();
        let mut right_res = Vec::new();

        let mut res = Vec::new();

        let size = nums.len() as i32;
        let mut tmp = 1;
        left_res.push(tmp);
        for index in 1..size {
            tmp = tmp * nums[(index - 1) as usize];
            left_res.push(tmp);
        }
        tmp = 1;
        right_res.push(tmp);
        for index in (0..(size - 1)).rev() {
            tmp = tmp * nums[(index + 1) as usize];
            right_res.push(tmp);
        }
        right_res.reverse();

        for index in 0..size {
            let index = index as usize;
            let multi_res = left_res[index] * right_res[index];
            res.push(multi_res);
        }

        res
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_product_except_self() {
        let case1 = Solution::product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(vec![24,12,8,6], case1);

        let case2 = Solution::product_except_self(vec![-1, 1, 0, -3, 3]);
        assert_eq!(vec![0, 0, 9, 0, 0], case2);
    }
}