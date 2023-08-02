fn main() {
    let res = Solution::dominant_index(vec![0,0,0,1]);
    println!("res: {}", res);
}

struct Solution { }
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut second = i32::MIN;
        let mut index = -1;
        for (i, num) in nums.iter().enumerate() {
            if *num > max {
                second = max;
                max = *num;
                index = i as i32;
            } else if *num > second {
                second = *num;
            }
        }
        if second == 0 || max / second >= 2 {
            return index;
        }
        -1
    }
}