fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let size = arr.len();
        let step = size / 4;

        for index in step..size {
            if arr[index] == arr[index - step] {
                return arr[index];
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_special_integer() {
        let case1 = Solution::find_special_integer(vec![1,2,2,6,6,6,6,7,10]);
        assert_eq!(6, case1);

        let case1 = Solution::find_special_integer(vec![1,1]);
        assert_eq!(1, case1);
    }
}