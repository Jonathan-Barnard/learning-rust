// mod queue;
//use crate::queue::Queue;
// mod stack;
// use crate::stack::Stack;

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

impl List {
    fn new() -> Self {
        List {head: None}
    }

    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

fn main() {
    let mut ll = List::new();
    ll.push(3);
    ll.push(2);
    ll.push(4);
    let v = ll.pop();
    println!("{}", v.unwrap());
    println!("{:?}", ll)
}
