fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut res = vec![-1; nums.len()];
        let mut stack = vec![0usize];
        for index in 0..(2 * size - 1) {
            println!("stack: {:?}", stack);
            while let Some(&val) = stack.last() {
                if nums[val] < nums[index % size] {
                    res[val] = nums[index % size];
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(index % size);
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_greater_elements() {
        let case1 = Solution::next_greater_elements(vec![1,2,1]);
        assert_eq!(vec![2, -1, 2], case1);

        let case2 = Solution::next_greater_elements(vec![1,2,3,4,3]);
        assert_eq!(vec![2,3,4,-1,4], case2);
    }
}