#![allow(dead_code)]

use std::fmt::Debug;


struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

pub struct List<T> {
    head: Option<Box<Node<T>>>
}

#[allow(dead_code)]
impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, value: T) {
        self.head = Some(Box::new(Node { value, next: self.head.take() }))
    }

    pub fn pop_front(&mut self) {
        self.head = self.head.take().and_then(|v| v.next);
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nd = &self.head;

        loop {
            match nd {
                Some(v) => {
                    write!(f, "{:?} -> ", v.value)?;
                    nd = &v.next;
                },
                None => {
                    write!(f, "nil\n")?;
                    break;
                }
            }
        }
        Ok(())
    }
}
