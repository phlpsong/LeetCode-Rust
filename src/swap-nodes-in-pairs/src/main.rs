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
// impl Solution {
//     pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         head.and_then(|mut n| {
//             match n.next {
//                 Some(mut m) => {
//                     n.next = Self::swap_pairs(m.next);
//                     m.next = Some(n);
//                     Some(m)
//                 },
//                 None => Some(n)
//             }
//         })

//     }
// }

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy = ListNode { val: 0, next: head };
        let mut curr = &mut dummy;

        while curr.next.is_some() {
            let mut head_node = curr.next.take().unwrap();
            if let Some(mut rear_node) = head_node.next.take() {
                head_node.next = rear_node.next;
                rear_node.next = Some(head_node);
                curr.next = Some(rear_node);
                curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                curr.next = Some(head_node);
                break;
            }
        }
        dummy.next

    }
}