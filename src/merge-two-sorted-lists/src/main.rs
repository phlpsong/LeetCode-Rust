fn main() {
    println!("Hello, world!");
}

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut node = &mut head;
        let mut list1 = list1;
        let mut list2 = list2;
        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
          if l1.val < l2.val {
            node.next = list1;
            node = node.next.as_mut().unwrap();
            list1 = node.next.take(); // take out and node.next become None here
          } else {
            node.next = list2;
            node = node.next.as_mut().unwrap();
            list2 = node.next.take();
          }
        }
        node.next = if list1.is_none() { list2 } else { list1 };
        head.next
    }
}