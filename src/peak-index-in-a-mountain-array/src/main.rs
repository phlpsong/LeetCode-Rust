fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut start = 1;
        let mut end = (arr.len() - 2) as i32;

        while start <= end {
            let mid = start + (end - start) / 2;
            let mid_num = arr[mid as usize];
            let left = mid - 1;
            let right = mid + 1;
            // let left_num = arr[left as usize];
            // let right_num = arr[right as usize];
            if mid_num > arr[left as usize] && mid_num > arr[right as usize] {
                return mid;
            } else if mid_num > arr[left as usize] && mid_num < arr[right as usize]  {
                start = mid + 1;
            } else if mid_num > arr[right as usize] && mid_num < arr[left as usize] {
                end = mid - 1;
            } 
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peak_index_mountain_array() {
        let case1 = Solution::peak_index_in_mountain_array(vec![0, 1, 0]);
        assert_eq!(case1, 1);

        let case2 = Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]);
        assert_eq!(case2, 1);

        let case3 = Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]);
        assert_eq!(case3, 1);

        let case4 = Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]);
        assert_eq!(case4, 2);

        let case5 = Solution::peak_index_in_mountain_array(vec![24,69,100,99,79,78,67,36,26,19]);
        assert_eq!(case5, 2);

        let case6 = Solution::peak_index_in_mountain_array(vec![3, 5, 3, 2, 0]);
        assert_eq!(case6, 1);
    }
}