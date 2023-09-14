fn main() {
    println!("Hello, world!");
}

struct Solution { }

// impl Solution {
//     pub fn merge_alternately(word1: String, word2: String) -> String {
//         let mut left: usize = 0;
//         let mut right: usize = 0;
//         let chars1: Vec<char> = word1.chars().collect();
//         let chars2: Vec<char> = word2.chars().collect();
//         let mut ans: Vec<char> = Vec::new();
//         while left < chars1.len() || right < chars2.len()  {
//             if left < chars1.len() && right < chars2.len() {
//                 ans.push(chars1[left]);
//                 ans.push(chars2[right]);
//                 left += 1;
//                 right += 1;
//                 continue;
//             } 

//             while left < chars1.len() {
//                 ans.push(chars1[left]);
//                 left += 1;
//             }

//             while right < chars2.len() {
//                 ans.push(chars2[right]);
//                 right += 1;
//             }
//         }

//         let res: String = ans.into_iter().collect();
//         res
//     }
// }

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut left: usize = 0;
        let mut right: usize = 0;
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        let mut ans: Vec<char> = Vec::new();
        while left < chars1.len() || right < chars2.len()  {
            if left < chars1.len() {
                ans.push(chars1[left]);
                left += 1;
            }

            if right < chars2.len() {
                ans.push(chars2[right]);
                right += 1;
            }
        }

        let res: String = ans.into_iter().collect();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternatly() {
        let case1 = Solution::merge_alternately(String::from("abc"), String::from("pqr"));
        assert_eq!(case1, String::from("apbqcr"));


        let case2 = Solution::merge_alternately(String::from("ab"), String::from("pqrs"));
        assert_eq!(case2, String::from("apbqrs"));

        let case3 = Solution::merge_alternately(String::from("abcd"), String::from("pq"));
        assert_eq!(case3, String::from("apbqcd"));

    }
}