#[derive(Debug)]
pub struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn add(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn remove(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.values.last()
    }
}
