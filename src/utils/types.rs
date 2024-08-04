use std::collections::VecDeque;

pub struct Call {
    pub filename: String,
    pub content: String,
    pub idx: u16
}
#[derive(Debug)]
pub struct Definition {
    pub content: String,
    pub name: String,
    pub idx: u16,
    pub params: Vec<String>,
    pub filename: String
}


pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.elements.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }
}