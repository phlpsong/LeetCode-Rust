fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut cell = names.into_iter().zip(heights.into_iter()).collect::<Vec<_>>();
        cell.sort_by(|(_, height_0), (_, height_1)| height_1.cmp(height_0));
        let (names, _): (Vec<String>, Vec<i32>) = cell.iter().cloned().unzip();
        names
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_people() {
        let case1 = Solution::sort_people(vec!["Mary".to_string(),"John".to_string(),"Emma".to_string()], vec![180, 165, 170]);
        assert_eq!(vec!["Mary".to_string(),"Emma".to_string(),"John".to_string()], case1);

        let case2 = Solution::sort_people(vec!["Alice".to_string(),"Bob".to_string(),"Bob".to_string()], vec![155,185,150]);
        assert_eq!(vec!["Bob".to_string(),"Alice".to_string(),"Bob".to_string()], case2);
    }
}