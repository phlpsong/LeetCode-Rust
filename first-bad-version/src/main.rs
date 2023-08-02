fn main() {
    println!("Hello, world!");
}

struct Solution { }
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut start = 0;
        let mut end = n - 1;
        let mut ans = 0;
        while start <= end {
            let mid = start + (end - start) / 2;
            let is_bad = isBadVersion(mid + 1);
            if is_bad {
                ans = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        ans + 1
    }
}