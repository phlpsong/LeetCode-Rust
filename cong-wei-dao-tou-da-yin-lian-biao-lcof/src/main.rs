use std::vec;

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
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = Vec::new();

        let mut head = head;
        while let Some(node) = head.as_ref() {
            let val = node.val;
            res.push(val);
            head = head.unwrap().next;
        }

        res.reverse();
        res
    }
}