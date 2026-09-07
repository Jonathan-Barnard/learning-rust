// mod queue;
//use crate::queue::Queue;
// mod stack;
// use crate::stack::Stack;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: List,
}

#[derive(Debug)]
enum List {
    Empty,
    More(Box<Node>),
}

impl List {
    fn new() -> Self {
        List::Empty
    }

    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: std::mem::replace(self, List::Empty),
        });

        *self = List::More(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        println!("{:?}", self);
        match self {
            List::Empty => None,
            List::More(node) => {
                *self = node.next;
                Some(node.elem)
            }
        }
    }
}

fn main() {
    let mut ll = List::new();
    ll.push(3);
    ll.push(2);
    let v = ll.pop();
    println!("{}", v.unwrap());
    println!("{:?}", ll)
}
