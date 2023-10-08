fn main() {
    println!("Hello, world!");
}

struct Solution { }

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut res: Vec<i32> = vec![0; operations.len()];
        let mut point_index = 0;
        let mut iter_index = 0;
        while iter_index < operations.len() {
            let ele = &operations[iter_index];
            if ele == "C" {
                point_index -= 1;
                iter_index += 1;
                continue;
            }
            if ele == "D" {
                res[point_index] = 2 * res[point_index - 1];
            } else if ele == "+" {
                res[point_index] = res[point_index - 1] + res[point_index - 2];
            } else {
                res[point_index] = operations[iter_index].parse::<i32>().unwrap();
            }
            point_index += 1;
            iter_index += 1;
        }
        
        let mut count = 0;
        for index in 0..point_index {
            count += res[index];
        }
        return count;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cal_points() {
        let case1 = Solution::cal_points(vec!["5".into(),"2".into(),"C".into(),"D".into(),"+".into()]);
        assert_eq!(30, case1);

        let case2 = Solution::cal_points(vec!["5".into(),"-2".into(),"4".into(),"C".into(),"D".into(),"9".into(),"+".into(),"+".into()]);
        assert_eq!(27, case2);

        let case3 = Solution::cal_points(vec!["1".into()]);
        assert_eq!(1, case3);
    }
}