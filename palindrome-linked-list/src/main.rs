use std::collections::VecDeque;

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
        if head.is_none() {
            return true;
        }
        let mut deque = VecDeque::new();
        let mut node = head;
        while let Some(n) = node.take() {
            deque.push_back(n.val);
            node = n.next;
        }

        println!("deq: {:?}", deque);
        while !deque.is_empty() {
            let front = deque.front();
            let last = deque.back();
            if front != last {
                return false;
            }
            deque.pop_back();
            deque.pop_front();
        }

        true
    }
}