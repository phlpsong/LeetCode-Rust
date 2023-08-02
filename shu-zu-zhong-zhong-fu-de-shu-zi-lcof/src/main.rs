fn main() {
    let res = Solution::find_repeat_number(vec![3,4,2,1,1,0]);
    println!("res: {}", res);
}

struct Solution { }

// impl Solution {
//     pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
//         let mut arr = nums;
//         arr.sort();
//         let mut res = arr[0];
//         for index in 1..arr.len() {
//             if arr[index] == res {
//                 return res;
//             } else {
//                 res = arr[index];
//             }
//         }
//         -1
//     }
// }

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut arr = nums;
        let mut index = 0;
        while index < arr.len() {
            if arr[index] == index as i32 {
                index += 1;
                continue;
            }
            if arr[arr[index] as usize] == arr[index] {
                return arr[index];
            }
            // swap arr[index] with arr[arr[index]]
            let j = arr[index] as usize;
            arr.swap(index, j);
        }
        -1
    }
}