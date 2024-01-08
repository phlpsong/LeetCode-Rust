struct Solution { }

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
      fn sort(nums: &mut Vec<i32>, start: usize, end: usize) {
        if start >= end {
          return;
        }
        let index = partition(nums, start, end);
        sort(nums, start, if index > 1 { index - 1 } else { 0 });
        sort(nums, index + 1, end);
      }

      fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> usize {
        let index = start + ((end - start) >> 1);
        let pivot = nums[index];
        nums.swap(index, end);

        let mut j = start;
        for i in start..end {
          if nums[i] < pivot {
            nums.swap(i, j);
            j += 1;
          }
        }

        nums.swap(j, end);
        j
      }

      let mut nums = nums;
      let len = nums.len();
      sort(&mut nums, 0, len-1);
      nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_array() {
        let case1 = Solution::sort_array(vec![5,2,3,1]);
        assert_eq!(vec![1,2,3,5], case1);

        let case2 = Solution::sort_array(vec![5,1,1,2,0,0]);
        assert_eq!(vec![0,0,1,1,2,5], case2);
    }
}