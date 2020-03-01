pub mod avltree;
mod node;
pub mod rbtree;

pub fn test() {
    let mut tree = rbtree::Tree::new();
    for x in 0..10 {
        tree.insert(x);
        println!("{}", tree.to_string());
    }
}
