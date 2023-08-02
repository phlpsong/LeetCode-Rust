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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head.unwrap();
        let mut p1 = head.as_mut();
        let head2 = p1.next.take();
        if let Some(mut head2) = head2 {
            let mut p2 = head2.as_mut();
            while p2.next.is_some() {
                p1.next = p2.next.take();
                p1 = p1.next.as_mut().unwrap();
                p2.next = p1.next.take();
                if p2.next.is_some() {
                    p2 = p2.next.as_mut().unwrap();
                }
            }
            p1.next = Some(head2);
        }
        Some(head)
    }
}