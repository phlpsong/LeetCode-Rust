fn main() {
    let res = Solution::length_of_last_word(String::from("   fly me   to   the moon  "));
    println!("res: {}", res);
}

struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let count = chars.len();
        let mut curr_index: i32 = count as i32 - 1;
        let mut size = 0;
        while curr_index >= 0 {
            if chars[curr_index as usize] == ' ' {
                curr_index -= 1;
                if size != 0 {
                    return size;
                } else {
                    continue;
                }
            }

            size += 1;
            curr_index -= 1;
        }
        size
    }
}