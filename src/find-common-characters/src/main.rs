fn main() {
    println!("Hello, world!");
}

struct Solution { }


const ASCII_A: u8 = 'a' as u8;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut freqs = vec![];
        for word in words {
            let mut freq = [0; 26];
            for letter in word.chars() {
                let index = (letter as u8 - ASCII_A) as usize;
                freq[index] += 1;
            }
            freqs.push(freq.clone());
        }
        let mut result = vec![];
        for index in 0..26 {
            let min = freqs.iter().map(|&f| f[index]).min().unwrap();
            for _ in 0..min {
                let index = index as u8;
                let letter = ((index + ASCII_A) as char).to_string();
                result.push(letter.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_common_chars() {
        let case1 = Solution::common_chars(vec!["bella".to_string(),"label".to_string(),"roller".to_string()]);
        assert_eq!(vec!["e".to_string(), "l".to_string(), "l".to_string()], case1);

        let case2 = Solution::common_chars(vec!["cool".to_string(),"lock".to_string(),"cook".to_string()]);
        assert_eq!(vec!["c".to_string(), "o".to_string()], case2);
    }
}