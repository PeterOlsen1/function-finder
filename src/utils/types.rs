use std::collections::VecDeque;

/**
 * Type to store data about function calls
 */
#[derive(Debug)]
pub struct Call {
    pub filename: String,
    pub content: String,
    pub idx: u16,
    pub params: Vec<String>,
    pub await_flag: bool
}

/**
 * Type to store data about function definitions
 */
#[derive(Debug)]
pub struct Definition {
    pub content: String,
    pub name: String,
    pub idx: u16,
    pub params: Vec<String>,
    pub filename: String,
    pub async_flag: bool,
    pub export_flag: bool
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