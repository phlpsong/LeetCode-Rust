fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n+1).map(|i| 
            match (i % 3 == 0, i % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                _ => i.to_string()
            }
        ).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let case1 = Solution::fizz_buzz(3);
        assert_eq!(vec!["1".to_string(),"2".to_string(),"Fizz".to_string()], case1);

        let case2 = Solution::fizz_buzz(5);
        assert_eq!(vec!["1".to_string(),"2".to_string(),"Fizz".to_string(),"4".to_string(),"Buzz".to_string()], case2);
    }
}