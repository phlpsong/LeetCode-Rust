fn main() {
    let mut vec1: Vec<i32> = vec![1, 1, 2];
    let res1 = remove_duplicates(&mut vec1) ;
    println!("res: {}", res1);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 1 { 
        return 1;
    }
    let mut insert_index = 0;
    let mut next_index = insert_index + 1;
    while next_index < nums.len() && insert_index < nums.len() {
        if nums[next_index] == nums[insert_index] {
            next_index += 1;
        } else {
            if next_index - insert_index > 1 {
                insert_index += 1;
                nums[insert_index] = nums[next_index];
                next_index += 1
            } else {
                insert_index += 1;
                next_index += 1;
            }
        }
    } 
    (insert_index as i32) + 1
}