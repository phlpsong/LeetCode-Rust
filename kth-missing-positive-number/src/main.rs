fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        if arr[0] > k {
            return k;
        }
        let mut left = 0;
        let mut right = arr.len() as i32;
        while left < right {
            let mid = (left + right) >> 1;
            let mut x = 0;
            if mid >= arr.len() as i32 {
                x = i32::MAX;
            } else {
                x = arr[mid as usize];
            }
            if x - mid - 1 >= k {
                right = mid;
            } else if x - mid - 1 < k {
                left = mid + 1;
            }
        }
        // 当前 index 缺失的整数个数:   arr[index] - index - 1
        // 对应第 k 个缺失数字，还差数字: k - 当前缺失的个数
        // 第 k 个缺失的数字 arr[index] + (k - 当前缺失的个数)
        return k - (arr[(left - 1) as usize] - (left - 1) - 1) + arr[(left - 1) as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_positive() {
        let case1 = Solution::find_kth_positive(vec![2,3,4,7,11], 5);
        assert_eq!(case1, 9);

        let case2 = Solution::find_kth_positive(vec![1, 2, 3, 4], 2);
        assert_eq!(case2, 6);
    }
}