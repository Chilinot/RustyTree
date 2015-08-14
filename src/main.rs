struct BinaryTree {
    value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn new(value: i32) -> BinaryTree {
        BinaryTree {
            value: value,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, value: i32) {
        if value < self.value {
            match self.left {
                Option::Some(ref mut l) => l.add(value),
                Option::None => self.left = Option::Some(Box::new(BinaryTree::new(value))),
            }
        }
        else {
            match self.right {
                Option::Some(ref mut r) => r.add(value),
                Option::None => self.right = Option::Some(Box::new(BinaryTree::new(value))),
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
    let mut tree = BinaryTree::new(12);

    tree.add(1);
    tree.add(2);
    tree.add(42);
    tree.add(30);
    tree.add(2);
    tree.add(121212121);

    tree.print();
}
