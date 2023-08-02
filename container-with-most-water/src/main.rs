use std::cmp;

struct Solution { }
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let size = height.len();
        let mut start = 0;
        let mut end = size - 1;
        let mut max_area = 0;
        while start < end {
            let min_height = cmp::min(height[start], height[end]);
            let area = min_height * (end - start) as i32;
            max_area = cmp::max(max_area, area);
            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        let case1 = Solution::max_area(vec![1,8,6,2,5,4,8,3,7]);
        assert_eq!(case1, 49);

        let case2 = Solution::max_area(vec![1,1]);
        assert_eq!(case2, 1);
    }
}