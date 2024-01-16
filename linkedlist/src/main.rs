use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
struct List<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Debug> List<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    fn insert(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    fn print_list(&self) {
        let mut curr = (self.head).as_ref();
        while curr.is_some() {
            println!("{:?}", curr.unwrap().elem);
            curr = (&curr.unwrap().next).as_ref();
        }
    }

    fn front(&self) -> Option<&Box<Node<T>>> {
        (self.head).as_ref()
    }

    fn back(&self) -> Option<&Box<Node<T>>> {
        let mut curr = self.front();
        while curr.is_some() {
            let curr_node = curr.unwrap();
            if curr_node.next.is_none() {
                return Some(curr_node);
            }
            curr = curr_node.next.as_ref();
        }
        None
    }
}

fn main() {
    let list = List::<i32>::new();
    println!("{}", list.len());
}
