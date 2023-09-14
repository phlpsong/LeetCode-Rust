fn main() {
    let res = Solution::climb_stairs(40);
    println!("res: {}", res);
}

struct Solution { }
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut stairs: Vec<i32> = vec![0, 1, 2];
        for index in 3..n + 1 {
            let index = index as usize;
            let count = stairs[index - 1] + stairs[index - 2];
            stairs.push(count);
        }

        stairs[n as usize]
    }
}