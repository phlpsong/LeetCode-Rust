fn main() {
    println!("Hello, world!");
}

struct MinStack {
    vec: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { vec : Vec::new() }
    }
    
    fn push(&mut self, x: i32) {
        self.vec.push(x);
    }
    
    fn pop(&mut self) {
        self.vec.pop();
    }
    
    fn top(&self) -> i32 {
        if self.vec.len() > 0 {
            let l = self.vec.len() - 1;
            return self.vec[l];
        }
        -1
    }
    
    fn get_min(&self) -> i32 {
        if let Some(m) = self.vec.iter().min() {
            return *m;
        }
        -1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */