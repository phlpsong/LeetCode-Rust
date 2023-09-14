fn main() {
    println!("Hello, world!");
}

struct Solution { }

// impl Solution {
//     pub fn arrange_coins(n: i32) -> i32 {
//         let mut index = 1;
//         let mut left = n;
//         while left >= index {
//             left -= index;
//             index += 1;
//         }
//         index - 1
//     }
// }

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut left: i64 = 0;
        let mut right: i64 = n as i64;
        while left < right {
            let mid: i64 = left + right + 1 >> 1;
            if mid * (mid + 1) / 2 <= n as i64 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        right as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_coins() {
        let case1 = Solution::arrange_coins(5);
        assert_eq!(case1, 2);

        let case2 = Solution::arrange_coins(1);
        assert_eq!(case2, 1);

        let case3 = Solution::arrange_coins(i32::MAX);
        assert_eq!(case3, 65535);
    }
}