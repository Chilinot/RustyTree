// Tell the compiler to insert the binary_tree module here.
mod binary_tree;

// Import the Node struct from the binary_tree module.
use binary_tree::Node;

fn main() {
    let mut tree = Node::new(12);

    tree.add(1);
    tree.add(2);
    tree.add(42);
    tree.add(30);
    tree.add(2);
    tree.add(121212121);

    println!("{}", tree.format());

    let mut string_tree = Node::new("Tjenare");

    string_tree.add("Halloj");
    string_tree.add("tjosan");
    string_tree.add("Is it working?");
    string_tree.add("Sorted");
    string_tree.add("Ö");
    string_tree.add("A");
    string_tree.add("C");
    string_tree.add("B");

    println!("{}", string_tree.format());
}
