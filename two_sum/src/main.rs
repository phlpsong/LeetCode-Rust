use std::collections::HashMap;

fn main() {
    let res1 = two_sum(vec![2,7,11,15], 9);
    println!("res1: {:?}", res1);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();
    let mut curr_index = 0;
    for num in nums {
        let left = target - num;
        match map.get(&left) {
            Some(&v) => {
                result.push(v);
                result.push(curr_index);
            },
            _ => {
                map.insert(num, curr_index);
            },
        }
        curr_index += 1;
    };

    result
}
