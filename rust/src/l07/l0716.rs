pub struct MaxStack {
    stack: Vec<Node<i32>>,
    max_stack: Vec<usize>,
}

struct Node<T> {
    val: T,
    deleted:bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }
    
    pub fn push(& mut self, x: i32) {
        self.stack.push(Node{val:x, deleted:false});
        if self.max_stack.is_empty() {
            self.max_stack.push(self.stack.len() -1);
            return;
        } 
        let &max = self.max_stack.last().unwrap();
        if x >= self.stack[max].val {
            self.max_stack.push(self.stack.len()-1);
        }
    }
    
    pub fn pop(& mut self) -> i32 {
        let v= self.stack.pop().unwrap();
        while let Some( Node{val: _ , deleted: true}) = self.stack.last(){
            self.stack.pop();
        }
        
        while let Some(&max) = self.max_stack.last() {
            if max < self.stack.len(){
                break;
            }
            self.max_stack.pop();
        }

        v.val
    }
    
    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }
    
    pub fn peek_max(&self) -> i32 {
        self.stack[*self.max_stack.last().unwrap()].val
    }
    
    pub fn pop_max(&mut self) -> i32 {
        let &max = self.max_stack.last().unwrap();
        if max == self.stack.len()-1 {
            return self.pop();
        }
        self.max_stack.pop().unwrap();
        self.stack[max].deleted = true;
        let mut max_val = i32::MIN;
        let mut max_index = usize::MAX;
        for i in max+1..self.stack.len() {
            if self.stack[i].deleted {
                continue;
            }
            if self.stack[i].val > max_val {
                max_val = self.stack[i].val;
                max_index = i;
            }
        }
        assert!(max_index != usize::MAX);

        if self.max_stack.is_empty() || self.stack[*self.max_stack.last().unwrap()].val < max_val {
            self.max_stack.push(max_index);
        }

        self.stack[max].val
    }
}

// debug and test

impl Default for MaxStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = MaxStack::new();
        obj.push(5);
        obj.push(1);
        obj.push(5);
        assert_eq!(obj.top(), 5);
        assert_eq!(obj.pop_max(), 5);
        assert_eq!(obj.top(), 1);
        assert_eq!(obj.peek_max(), 5);
        assert_eq!(obj.pop(), 1);
        assert_eq!(obj.top(), 5);
    }

    #[test]
    fn test_2() {
        let mut obj = MaxStack::new();
        obj.push(37);
        assert_eq!(obj.top(), 37);
        assert_eq!(obj.pop_max(), 37);
        obj.push(24);
        assert_eq!(obj.top(), 24);
        assert_eq!(obj.peek_max(), 24);
        obj.push(42);
        obj.push(18);
        assert_eq!(obj.peek_max(), 42);
        assert_eq!(obj.top(), 18);
        assert_eq!(obj.pop_max(), 42);
        obj.push(-67);
        assert_eq!(obj.pop_max(), 24);
        assert_eq!(obj.pop(), -67);
        assert_eq!(obj.peek_max(), 18);
        obj.push(-95);
        obj.push(9);
        assert_eq!(obj.pop(), 9);
    }

    #[test]
    fn test_3() {
        let mut obj = MaxStack::new();
        obj.push(-81);
        obj.push(-41);
        obj.push(38);
        obj.push(18);
        assert_eq!(obj.top(), 18);
        assert_eq!(obj.top(), 18);
        obj.push(-39);
        assert_eq!(obj.pop(), -39);
        assert_eq!(obj.pop_max(), 38);
        assert_eq!(obj.pop_max(), 18);
        assert_eq!(obj.peek_max(), -41);
        assert_eq!(obj.top(), -41);
        obj.push(56);
    }
}