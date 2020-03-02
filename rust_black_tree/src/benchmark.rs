#[cfg(test)]
mod benchmark {
    use super::*;
    use crate::{avltree, rbtree};
    use std::time::Instant;

    #[test]
    #[ignore]
    pub fn benchmark() {
        for tree_size in &[10000, 40000, 70000, 100000, 130000] {
            print!("Time for Red Black Tree, {} elements: ", tree_size);
            std::io::stdout().flush().unwrap();
            let rbtree = rbtree::Tree::new();
            let now = Instant::now();
            insert_n_elements_and_search_lowest(rbtree, tree_size);
            println!("{} milliseconds", now.elapsed().as_millis());
            println!();

            print!("Time for AVL Tree, {} elements: ", tree_size);
            std::io::stdout().flush().unwrap();
            let avltree = rbtree::Tree::new();
            let now = Instant::now();
            insert_n_elements_and_search_lowest(avltree, tree_size);
            println!("{} milliseconds", now.elapsed().as_millis());
            println!();
        }
    }

    fn insert_n_elements_and_search_lowest<T>(tree: T, num_times: usize)
    //where T: // tree trait
    {
        for i in 0..num_times {
            tree.insert(i);
        }

        for i in 0..num_times / 10 {
            tree.contains(i);
        }
    }
}
