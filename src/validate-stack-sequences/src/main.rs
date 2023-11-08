fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = vec![];

        let mut index = 0;
        for num in pushed {
            stack.push(num);
            while !stack.is_empty() && stack[stack.len() - 1] == popped[index] {
                stack.pop();
                index += 1;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_stack_sequences() {
        let case1 = Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,5,3,2,1]);
        assert_eq!(true, case1);

        let case2 = Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,3,5,1,2]);
        assert_eq!(false, case2);
    }
}