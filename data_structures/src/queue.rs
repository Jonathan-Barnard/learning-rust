#[derive(Debug)]
pub struct Queue<T> {
    values_in: Vec<T>,
    values_out: Vec<T>,
}

//impl<T> Iterator for Queue<T> {
//    type Item = T;
//    fn next(&mut self) -> Option<T> {
//        self.dequeue()
//    }
//}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values_in: Vec::with_capacity(capacity),
            values_out: Vec::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values_in.is_empty() && self.values_out.is_empty()
    }

    pub fn enqueue(&mut self, x: T) {
        self.values_in.push(x);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.values_out.is_empty() {
            while let Some(v) = self.values_in.pop() {
                self.values_out.push(v);
            }
        }
        self.values_out.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.values_out.last().or(self.values_in.first())
    }

    pub fn len(&self) -> usize {
        self.values_in.len() + self.values_out.len()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> {
        self.values_out
            .drain(..)
            .rev()
            .chain(self.values_in.drain(..))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.values_out.iter().rev().chain(self.values_in.iter())
    }
}
