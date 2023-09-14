fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut start = 0;
        let mut end = letters.len() as i32 - 1;
        while start <= end {
            let mid = start + (end - start) / 2;
            if target < letters[mid as usize] {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        letters[start as usize % letters.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greatest_letter() {
        let case1 = Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a');
        assert_eq!(case1, 'c');

        let case2 = Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c');
        assert_eq!(case2, 'f');


        let case3 = Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd');
        assert_eq!(case3, 'f');

        let case4 = Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'm');
        assert_eq!(case4, 'c');
    }
}