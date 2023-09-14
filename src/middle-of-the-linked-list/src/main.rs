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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow_node = &head;
        let mut fast_node = &head;

        while fast_node.as_ref().is_some() && fast_node.as_ref()?.next.is_some() {
            slow_node = &slow_node.as_ref()?.next;
            fast_node = &fast_node.as_ref()?.next.as_ref()?.next;
        }

        slow_node.clone()
    }
}