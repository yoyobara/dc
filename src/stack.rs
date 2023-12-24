use std::{collections::LinkedList, fmt::Debug};

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

    pub fn len(&self) -> usize {
        self.inner_ll.len()
    }
    
    pub fn clear(&mut self) {
        self.inner_ll.clear();
    }

    pub fn new() -> Stack<T> {
        Stack { inner_ll: LinkedList::new() }
    }
}

impl<T: Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner_ll, f)
    }
}
