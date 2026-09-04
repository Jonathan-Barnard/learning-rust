#[derive(Debug)]
struct Queue<T> {
    values_in: Vec<T>,
    values_out: Vec<T>,
}

impl<T> Iterator for Queue<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.dequeue()
    }
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self::with_capacity(0)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            values_in: Vec::with_capacity(capacity),
            values_out: Vec::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.values_in.is_empty() && self.values_out.is_empty()
    }

    fn enqueue(&mut self, x: T) {
        self.values_in.push(x);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.values_out.is_empty() {
            while let Some(v) = self.values_in.pop() {
                self.values_out.push(v);
            }
        }
        self.values_out.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.values_out.last().or(self.values_in.first())
    }

    fn len(&self) -> usize {
        self.values_in.len() + self.values_out.len()
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.values_out.iter().rev().chain(self.values_in.iter())
    }
}

fn main() {
    let mut q = Queue::with_capacity(100);
    q.enqueue(42);
    q.enqueue(23);
    q.enqueue(2);

    let q_ref = &mut q;
    for v in q.iter() {
        println!("{}, ", v)
    }

    q.enqueue(4);

    println!("{}", q.len());
    println!("{:?}", q.peek());
    q.dequeue();
    q.dequeue();
    q.enqueue(4);
    q.dequeue();

    if q.is_empty() {
        println!("Empty")
    } else {
        println!("Not Empty")
    };

    println!("{:?}", q);
}
