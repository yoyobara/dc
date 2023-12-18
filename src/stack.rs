use std::collections::LinkedList;

pub struct Stack<T> {
    inner_ll: LinkedList<T>
}

impl<T> Stack<T> {
    fn push(&mut self, value: T) {
        self.inner_ll.push_front(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.inner_ll.pop_front()
    }

    fn top(&self) -> Option<&T> {
        self.inner_ll.front()
    }

    fn is_empty(&self) -> bool {
        self.inner_ll.is_empty()
    }
}
