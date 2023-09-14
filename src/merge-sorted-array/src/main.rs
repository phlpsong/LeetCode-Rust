fn main() {
    
    // let mut arr1 = vec![1];
    // Solution::merge(&mut arr1, 1, vec![], 0);
    // println!("arr: {:?}", arr1);

    let mut arr2 = vec![4,5,6,0,0,0];
    Solution::merge(&mut arr2, 3, vec![1,2,3], 3);
    println!("arr: {:?}", arr2);

    let mut arr3 = vec![1,2,4,5,6,0];
    Solution::merge(&mut arr3, 5, vec![3], 1);
    println!("arr: {:?}", arr3);
}

struct Solution {}

impl Solution {
    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
    //     let mut i = m as usize;
        
    //     for index in 0..n {
    //         nums1[i] = nums2[index as usize];
    //         i += 1;
    //     }

    //     nums1.sort();
    // }

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
        let mut index = (m + n - 1) as usize;
        let mut m = (m - 1) as usize;
        let mut n = (n - 1) as usize;
        
        while n >= 0 as usize {
            while m >= 0 as usize && nums1[m] > nums2[n] {
                println!("m: {}, n: {}", m, n);
                nums1.swap(index, m);
                m -= 1;
                index -= 1;
            }
            nums1[index] = nums2[n];
            n -= 1;
            index -= 1;
        }
    }
}