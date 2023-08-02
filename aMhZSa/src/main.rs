fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution { }

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut arr = Vec::new();
        let mut curr = head;
        while let Some(node) = curr.take() {
            arr.push(node.val);
            curr = node.next;
        }

        let mut prev = 0;
        let mut last = arr.len() - 1;
        while prev < last {
            if arr[prev] == arr[last] {
                prev += 1;
                last -= 1;
            } else {
                return false;
            }
        }

        true
    }
}