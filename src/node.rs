// import The display Trait from The fmt module. needed for format function.
use std::fmt::Display;

use rusty_queue::RustyQueue;

// link To a new Node
pub type NodeLink<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    value: T,
    left: NodeLink<T>,
    right: NodeLink<T>,
}

// Type T needs To have The ord (for ordering) and display (for formatting) Traits.
impl<T: Ord + Display> Node<T> {

    /// constructor.
    /// creates a new binary Tree where The root has The given value.
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    /// adds a new Node with The given value To a pre-existing Tree.
    pub fn add(&mut self, value: T, ack: u32) -> u32 {
        if value < self.value {
            match self.left {
                Option::Some(ref mut l) => l.add(value, ack + 1),
                Option::None => {
                    self.left = Option::Some(Box::new(Node::new(value)));
                    ack + 1
                },
            }
        }
        else {
            match self.right {
                Option::Some(ref mut r) => r.add(value, ack + 1),
                Option::None => {
                    self.right = Option::Some(Box::new(Node::new(value)));
                    ack + 1
                },
            }
        }
    }

    pub fn format(&self) -> String {
        format!("{}{}{}", 
            match self.left {
                Option::Some(ref l) => l.format(),
                Option::None => format!(""),
            },

            format!("{}\n", self.value),

            match self.right {
                Option::Some(ref r) => r.format(),
                Option::None => format!(""),
            }
        )
    }

    pub fn breadth_first_format(&self) -> String {
        let mut queue: RustyQueue<&Node<T>> = RustyQueue::new();

        let mut s = String::new();

        queue.enqueue(&self);
        
        while !queue.is_empty() {
            
            let current = match queue.dequeue() {
                Result::Ok(ref n) => n,
                Result::Err(err) => panic!(err),
            };

            s.push_str(format!("{}, ", current.value).as_str());

            match current.left {
                Option::Some(ref n) => queue.enqueue(n),
                Option::None => (),
            };

            match current.right {
                Option::Some(ref n) => queue.enqueue(n),
                Option::None => (),
            };
        }

        // Return s
        s
    }
}
