use std::{cmp};

fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() { return vec![];}
        let mut result: Vec<Vec<i32>> = vec![];
        // let mut intervals = intervals;
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        println!("intervals: {:?}", intervals);
        for item in intervals {
            let left = item[0];
            let right = item[1];

            if result.is_empty() || result[result.len() - 1][1] < left {
                result.push(item);
            } else {
                let last_right = result[result.len() - 1][1];
                let max_rigfht = cmp::max(right, last_right);
                let len = result.len();
                result[len - 1][1] = max_rigfht;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_cases() {
        let intervals1: Vec<Vec<i32>> = vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]];
        let intervals2: Vec<Vec<i32>> = vec![vec![1,4],vec![4,5]];
        let intervals3: Vec<Vec<i32>> = vec![];
        let intervals4: Vec<Vec<i32>> = vec![vec![1, 2]];
        let intervals5: Vec<Vec<i32>> = vec![vec![1,4], vec![0,4]];

        assert_eq!(Solution::merge(intervals1), vec![vec![1,6],vec![8,10],vec![15,18]]);
        assert_eq!(Solution::merge(intervals2), vec![vec![1,5]]);
        assert_eq!(Solution::merge(intervals3).is_empty(), true);
        assert_eq!(Solution::merge(intervals4), vec![vec![1,2]]);
        assert_eq!(Solution::merge(intervals5), vec![vec![0,4]]);
    }
}