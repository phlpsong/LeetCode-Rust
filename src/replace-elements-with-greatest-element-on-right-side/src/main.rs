fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut mx = arr[n - 1];
        arr[n - 1] = -1;
        for i in (0..n-1).rev() {
           let t = mx;
           mx = mx.max(arr[i]);
           arr[i] = t;
        }
        arr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_elements() {
        let case1 = Solution::replace_elements(vec![17,18,5,4,6,1]);
        assert_eq!(vec![18,6,6,6,1,-1], case1);

        let case2 = Solution::replace_elements(vec![400]);
        assert_eq!(vec![-1], case2);
    }
}