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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut node = head.as_mut().unwrap();
        let mut duplicate = node.val;
        while let Some(next) = node.next.take() {
            if next.val == duplicate {
              node.next = next.next;
            } else {
              duplicate = next.val;
              node.next = Some(next);
              node = node.next.as_mut().unwrap();
            }
        }
        head
    }
}