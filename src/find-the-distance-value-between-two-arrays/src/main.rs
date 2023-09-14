fn main() {
    println!("Hello, world!");
}

struct Solution { }
// impl Solution {
//     pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
//         let mut count = 0;
//         for a1 in &arr1 {
//             count += 1;
//             for a2 in &arr2 {
//                 if (a1 - a2).abs() <= d {
//                     count -= 1;
//                     break;
//                 }
//             }
//         }
//         count
//     }
// }

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        arr2.sort();
        let mut ans = 0;
        for num1 in arr1.iter() {
            if !Self::binary_search(&arr2, num1 - d, num1 + d) {
                ans += 1;
            }
        }
        ans
    }

    fn binary_search(arr: &Vec<i32>, start: i32, end: i32) -> bool {
        let mut lo = 0;
        let mut hi = arr.len() - 1;
        while lo <= hi && hi < arr.len() {
            let mid = lo + (hi - lo) / 2;
            if arr[mid] >= start && arr[mid] <= end {
                return true;
            }

            if arr[mid] < start {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        false
    }
}