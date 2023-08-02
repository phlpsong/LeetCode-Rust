fn main() {
    println!("Hello, world!");
}

struct Solution { }

/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut start = 0;
        let mut end = n - 1;

        while start <= end {
            let mid: i32 = start + (end - start) / 2;
            let mid_num = mid + 1;
            let guess: i32 = guess(mid_num);
            if guess == -1 {
                end = mid - 1;
            } else if guess == 1 {
                start = mid + 1
            } else {
                return mid;
            }
        }

        -1
    }
}