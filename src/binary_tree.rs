// Trait for displaying values. Our generic needs this for the print method.
use std::fmt::Display;

/// Link to a new node
type NodeLink<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    value: T,
    left: NodeLink<T>,
    right: NodeLink<T>,
}

impl<T: Ord + Display> Node<T> {

    /// Constructor.
    /// Creates a new binary tree where the root has the given value.
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    /// Adds a new node with the given value to a pre-existing tree.
    pub fn add(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Option::Some(ref mut l) => l.add(value),
                Option::None => self.left = Option::Some(Box::new(Node::new(value))),
            }
        }
        else {
            match self.right {
                Option::Some(ref mut r) => r.add(value),
                Option::None => self.right = Option::Some(Box::new(Node::new(value))),
            }
        }
    }

    /// Prints the tree to the console.
    pub fn print(&self) {
        match self.left {
            Option::Some(ref l) => l.print(),
            Option::None => (),
        }

        println!("{}", self.value);

        match self.right {
            Option::Some(ref r) => r.print(),
            Option::None => (),
        }
    }
}
