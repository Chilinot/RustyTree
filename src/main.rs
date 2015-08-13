struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn new(value: i32) -> BinaryTree {
        BinaryTree {
            value: value,
            left: Option::None,
            right: Option::None,
        }
    }

    fn add(&mut self, value: i32) {
        if(value < self.value) {
            match self.left {
                Option::Some(l) => l.add(value),
                Option::None => self.left = Option::Some(Box::new(BinaryTree::new(value))),
            }
        }
        else {
            match self.right {
                Option::Some(r) => r.add(value),
                Option::None => self.right = Option::Some(Box::new(BinaryTree::new(value))),
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let tree = BinaryTree::new(12);

    tree.add(1);
}
