fn main() {
    println!("Hello, world!");
}


struct Solution {

}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start: i32 = 0;
        let mut end = (nums.len() - 1) as i32;
        let mut res = nums.len() as i32;
        while start <= end {
            let mid = start + (end - start) / 2;
            let mid_num = nums[mid as usize];
            if mid_num < target {
                start = mid + 1;
            } else if mid_num >= target {
                res = mid;
                end = mid - 1;
            } 
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        let case1 = Solution::search_insert(vec![1,3,5,6], 5);
        assert_eq!(case1, 2);

        let case2 = Solution::search_insert(vec![1,3,5,6], 2);
        assert_eq!(case2, 1);

        // let case3 = Solution::search_insert(vec![1,3,5,6], 7);
        // assert_eq!(case3, 4);
    }
}