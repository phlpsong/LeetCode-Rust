use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let size = arr.len();
        let target = size / 4;

        let mut map: HashMap<i32, usize> = HashMap::new();

        for index in 0..size {
            if let Some(&value) = map.get(&arr[index]) {
                map.insert(arr[index], value + 1);
            } else {
                map.insert(arr[index], 1);
            }
        }

        for (key, value) in map.into_iter() {
            if value > target {
                return key;
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