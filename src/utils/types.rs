use std::collections::VecDeque;

/**
 * Type to store function calls
 */
pub struct Call {
    pub filename: String,
    pub content: String,
    pub idx: u16
}

/**
 * Type to store function definitions
 */
#[derive(Debug)]
pub struct Definition {
    pub content: String,
    pub name: String,
    pub idx: u16,
    pub params: Vec<String>,
    pub filename: String
}

//use this queue later to find {} corresponding to a function?
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