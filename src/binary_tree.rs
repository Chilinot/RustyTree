use std::fmt::Display;
use node::Node;
use node::NodeLink;
use rusty_queue::RustyQueue;

// This struct is used to store metadata about the tree, such as the current height of the tree, 
// or amount of nodes within it.
pub struct BinaryTree<T> {
    root: NodeLink<T>,  // The beginning of the tree.
    height: usize,      // The longest branch in the tree, from root to leaf has this many nodes in its path.
    node_amount: usize, // The amount of nodes currently stored in this tree.
}

impl<T: Ord + Display> BinaryTree<T> {

    /// Construct a new BinaryTree object.
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            root: None,
            height: 0_usize,
            node_amount: 0_usize,
        }
    }

    /// Insert a new value in the tree.
    pub fn insert(&mut self, value: T) {
        //TODO Update tree height!
        let height: usize = match self.root {
            Option::Some(ref mut root_node) => root_node.add(value, 1),
            Option::None => {
                self.root = Option::Some(Box::new(Node::new(value)));
                1_usize // Return the height: 1
            }
        };
        
        self.node_amount += 1;

        if self.height < height {
            self.height = height;
        }
    }

    /// Return an ordered string representation of the values stored inside the tree.
    pub fn format(&self) -> String {
        match self.root {
            Option::Some(ref root_node) => root_node.format(),
            Option::None => format!("{}", "The tree is empty!"),
        }
    }

    /// Get the current size of the tree.
    pub fn get_size(&self) -> usize {
        self.node_amount
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
    
    pub fn breadth_first_format(&self) -> Result<String, &str> {
        let mut queue: RustyQueue<&Node<T>> = RustyQueue::new();

        let mut s = String::new();

        // Retrieve the root node of the tree. If the tree is empty, return error message.
        let root_node = match self.root {
            Option::Some(ref rn) => rn,
            Option::None => return Err("The tree is empty!"),
        };

        queue.enqueue(root_node);
        
        while !queue.is_empty() {
            
            let current = queue.dequeue().unwrap();

            s = format!("{}{}, ", s, current.value);

            match current.left {
                Option::Some(ref n) => queue.enqueue(n),
                Option::None => (),
            };

            match current.right {
                Option::Some(ref n) => queue.enqueue(n),
                Option::None => (),
            };
        }

        // Return the formatted string
        Ok(s)
    }
}
