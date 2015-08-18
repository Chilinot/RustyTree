// Trait for displaying values. Our generic needs this for the print method.
use std::fmt::Display;

/// Link to a new node
type NodeLinke<T> = Option<Box<Node<T>>>;

struct Node<T: Ord + Display> {
    value: T,
    left: NodeLinke<T>,
    right: NodeLinke<T>,
}

impl<T: Ord + Display> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, value: T) {
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

    fn print(&self) {
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

fn main() {
    let mut tree = Node::new(12);

    tree.add(1);
    tree.add(2);
    tree.add(42);
    tree.add(30);
    tree.add(2);
    tree.add(121212121);

    tree.print();

    let mut string_tree = Node::new("Tjenare");

    string_tree.add("Halloj");
    string_tree.add("tjosan");
    string_tree.add("Is it working?");
    string_tree.add("Sorted");
    string_tree.add("Ö");
    string_tree.add("A");
    string_tree.add("C");
    string_tree.add("B");

    string_tree.print();
}
