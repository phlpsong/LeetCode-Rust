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
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut new_head = ListNode { val: 0, next: head};
        let mut node = &mut new_head;

        while let Some(next) = node.next.take() {
            if next.val == val {
              node.next = next.next;
            } else {
              node.next = Some(next);
              node = node.next.as_mut().unwrap();
            }
        }
        new_head.next
    }
}