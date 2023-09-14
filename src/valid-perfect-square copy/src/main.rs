fn main() {
    println!("Hello, world!");
}

struct Solution { }

// impl Solution {
//     pub fn is_perfect_square(num: i32) -> bool {
//         if num == 1 { return true; }
//         let mut start: i64 = 0;
//         let mut end: i64 = (num - 1) as i64;
//         while start <= end {
//             let mid: i64 = start + (end - start) / 2;
//             let tmp: i64 = mid * mid;
//             if tmp == num as i64 {
//                 return true;
//             } else if tmp > num as i64 {
//                 end = mid - 1;
//             } else {
//                 start = mid + 1;
//             }
//         }
//         false
//     }
// }

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut count: i64 = 1;
        let mut sum: i64 = count;
        while sum <= num as i64 {
            if sum == num as i64 {
                return true;
            }
            count += 2;
            sum += count;
        }

        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        let case1 = Solution::is_perfect_square(16);
        assert_eq!(case1, true);

        let case2 = Solution::is_perfect_square(14);
        assert_eq!(case2, false);

        let case3 = Solution::is_perfect_square(1);
        assert_eq!(case3, true);

        let case3 = Solution::is_perfect_square(808201);
        assert_eq!(case3, true);
    }
}