fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {

    fn is_vowel(ch: char) -> bool {
        return ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' 
            || ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U' 
    }

    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = chars.len() - 1;
        while start < end {
            if !Solution::is_vowel(chars[start]) {
                start += 1;
                continue;
            }

            if !Solution::is_vowel(chars[end]) {
                end -= 1;
                continue;
            }

            chars.swap(start, end);
            start += 1;
            end -= 1;
        }

        let res: String = chars.into_iter().collect();
        res 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_vowels() {
        let case1 = Solution::reverse_vowels("hello".to_string());
        assert_eq!("holle".to_string(), case1);

        let case2 = Solution::reverse_vowels("leetcode".to_string());
        assert_eq!("leotcede".to_string(), case2);
    }
}