fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b { -1 } else { a.len().max(b.len()) as i32 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_lu_slength() {
        let case1 = Solution::find_lu_slength("aba".to_string(), "cdc".to_string());
        assert_eq!(3, case1);

        let case2 = Solution::find_lu_slength("aaa".to_string(), "bbb".to_string());
        assert_eq!(3, case2);

        let case3 = Solution::find_lu_slength("aaa".to_string(), "aaa".to_string());
        assert_eq!(-1, case3);
    }
}