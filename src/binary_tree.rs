use node::Node;
use node::NodeLink;

use std::fmt::Display;

// This struct is used to store metadata about the tree, such as the current height of the tree, 
// or amount of nodes within it.
pub struct BinaryTree<T> {
    root: NodeLink<T>, // The beginning of the tree.
    max_height: u32, // The longest branch in the tree, from root to leaf has this many nodes in its path.
    node_amount: u32, // The amount of nodes currently stored in this tree.
}

impl<T: Ord + Display> BinaryTree<T> {

    /// Construct a new BinaryTree object.
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            root: None,
            max_height: 0_u32,
            node_amount: 0_u32,
        }
    }

    /// Insert a new value in the tree.
    pub fn insert(&mut self, value: T) {
        //TODO Update tree height!
        match self.root {
            Option::Some(ref mut root_node) => root_node.add(value),
            Option::None => self.root = Option::Some(Box::new(Node::new(value))),
        }
        
        self.node_amount += 1;
    }

    /// Return an ordered string representation of the values stored inside the tree.
    pub fn format(&self) -> String {
        match self.root {
            Option::Some(ref root_node) => root_node.format(),
            Option::None => format!("{}", "The tree is empty!"),
        }
    }

    /// Get the current size of the tree.
    pub fn get_size(&self) -> u32 {
        self.node_amount
    }
}
