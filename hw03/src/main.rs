// main.rs

extern crate hw03;
use hw03::second::BST;

fn main() {
    let mut bst = BST::new();

    for i in 1..10 {
        bst.insert(i);
    }
    
    for (i, e) in bst.into_iter().enumerate() {
        println!("{} => {}", i, e);
    }
}
