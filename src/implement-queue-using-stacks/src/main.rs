fn main() {
    println!("Hello, world!");
}

struct MyQueue {
    ins: Vec<i32>,
    out: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            ins: vec![],
            out: vec![],
        }
    }

    fn transfer(&mut self) {
        while let Some(x) = self.ins.pop() {
            self.out.push(x);
        }
    }
    
    fn push(&mut self, x: i32) {
        self.ins.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if (self.out.is_empty()) {
            self.transfer();
        }
        self.out.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if (self.out.is_empty()) {
            self.transfer();
        }
        *self.out.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.out.is_empty() && self.ins.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */