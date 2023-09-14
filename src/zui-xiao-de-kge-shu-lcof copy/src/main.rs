fn main() {
    let res = Solution::get_least_numbers(vec![2,9,4, 2, 9, 6,5,7,132, 3123, 12, 32, 45, 89, 54, 67, 234], 10);
    println!("res: {:?}", res);
}

struct Solution { }
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        let len = arr.len() as i32;
        let mut partition_index = -1;
        let mut start = 0;
        let mut end = len - 1;
        // 下标为 k - 1 的数
        let k = k - 1;
        while partition_index != k {
            partition_index = Solution::partition(&mut arr, start, end) as i32;
            if partition_index < k {
                start = partition_index + 1;
            } else {
                end = partition_index - 1;
            }
        }
        arr[..((k + 1) as usize)].iter().cloned().collect()
    }

    fn partition(arr: &mut Vec<i32>, left: i32, right: i32) -> usize {
        let mut i = left as usize;
        let mut j = right as usize;
        let tmp = arr[i];
        while i != j {
            while arr[j] >= tmp && i < j {
                j -= 1;
            }
            while arr[i] <= tmp && i < j {
                i += 1;
            }
            arr.swap(i, j);
        }
        arr[left as usize] = arr[i];
        arr[i] = tmp;
        i
    }
}