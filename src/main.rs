// Tell the compiler we want the external library "rusty_tree"
extern crate rusty_tree;

// Import the BinaryTree struct from the binary_tree module.
use rusty_tree::binary_tree::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(1);
    tree.insert(2);
    tree.insert(42);
    tree.insert(30);
    tree.insert(2);
    tree.insert(121212121);

    println!("{}", tree.format());
    println!("Node amount: {}", tree.get_size());
    println!("Height: {}", tree.get_height());
    println!("{}\n", tree.breadth_first_format().unwrap());

    let mut string_tree = BinaryTree::new();

    string_tree.insert("Halloj");
    string_tree.insert("tjosan");
    string_tree.insert("Is it working?");
    string_tree.insert("Sorted");
    string_tree.insert("Ö");
    string_tree.insert("A");
    string_tree.insert("C");
    string_tree.insert("B");

    println!("{}", string_tree.format());
    println!("Node amount: {}", string_tree.get_size());
    println!("Height: {}\n", string_tree.get_height());
    println!("{}\n", string_tree.breadth_first_format().unwrap());


    let mut ub_tree = BinaryTree::new();

    ub_tree.insert(1);
    ub_tree.insert(2);
    ub_tree.insert(3);
    ub_tree.insert(4);
    ub_tree.insert(5);
    ub_tree.insert(6);
    ub_tree.insert(7);
    ub_tree.insert(8);

    println!("{}", ub_tree.format());
    println!("Node amount: {}", ub_tree.get_size());
    println!("Height: {}", ub_tree.get_height());
    println!("{}\n", ub_tree.breadth_first_format().unwrap());
}
