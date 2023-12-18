use std::collections::LinkedList;

pub struct Stack<T> {
    inner_ll: LinkedList<T>
}

impl<T> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.inner_ll.push_front(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner_ll.pop_front()
    }

    pub fn top(&self) -> Option<&T> {
        self.inner_ll.front()
    }

    pub fn is_empty(&self) -> bool {
        self.inner_ll.is_empty()
    }

    pub fn new() -> Stack<T> {
        Stack { inner_ll: LinkedList::new() }
    }
}
