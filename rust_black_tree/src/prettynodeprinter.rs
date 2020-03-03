use super::node::Node;
use super::node::Side;

use super::avltree::AVLNode;
use super::rbtree::ColorNode;
use super::unbalancetree::RegularNode;

const LEFT: &str = "╱";
const RIGHT: &str = "╲";
const ZWSP: &str = "​"; // this string contains a zero width space

pub fn printprettybst<T: std::fmt::Debug + std::cmp::PartialOrd>(
    node: &RegularNode<T>,
) -> Option<String> {
    print_node_pretty(node)
}

pub fn printprettyrb<T: std::fmt::Debug + std::cmp::PartialOrd>(
    node: &ColorNode<T>,
) -> Option<String> {
    print_node_pretty(node)
}

pub fn printprettyavl<T: std::fmt::Debug + std::cmp::PartialOrd>(
    node: &AVLNode<T>,
) -> Option<String> {
    print_node_pretty(node)
}

fn print_node_pretty<T: std::fmt::Debug, N: Node<T>>(node: &N) -> Option<String> {
    let (grid_width, grid_height) = if let Some((w, h)) = term_size::dimensions() {
        (w, h)
    } else {
        (150, 100)
    };
    let mut grid: Vec<Vec<String>> = Vec::with_capacity(grid_height);
    let mut used_depth = 0;
    // make grid
    for row in 0..grid_height {
        grid.push(Vec::with_capacity(grid_width));
        for _ in 0..grid_width {
            grid[row].push(" ".to_string());
        }
    }

    // fill grid
    fn fill_grid<T: std::fmt::Debug, N: Node<T>>(
        x: usize,
        depth: usize,
        n: &N,
        node: &N,
        grid: &mut Vec<Vec<String>>,
        ud: &mut usize,
    ) -> bool {
        let (val_str, cw) = n.to_self_string_display();
        let cw = cw+1;
        let val_str = " ".to_string() + &val_str;
        let lw = n
            .get_child(Side::Left)
            .map(|x| node.get(x).get_size())
            .unwrap_or(0);
        let rw = n
            .get_child(Side::Right)
            .map(|x| node.get(x).get_size())
            .unwrap_or(0);
        let mw = std::cmp::max(lw, rw) * cw;
        if x + mw >= grid[0].len() {
            return false;
        }
        if x < mw {
            return false;
        }
        if x + cw/2 > grid[0].len() {
            return false;
        }
        if depth + mw >= grid.len() {
            return false;
        }

        // write node
        grid[depth][x  - cw / 2] = val_str;
        for i in 1..cw {
            grid[depth][x  - cw / 2 + i] = ZWSP.to_string();
        }
        *ud = std::cmp::max(*ud, depth) + 1;
        if let Some(c) = n.get_child(Side::Left) {
            for i in 1..mw {
                grid[depth + i][x - i] = String::from(LEFT);
            }
            if !fill_grid(x - mw, depth + mw, node.get(c), node, grid, ud) {
                return false;
            }
        }
        if let Some(c) = n.get_child(Side::Right) {
            for i in 1..mw {
                grid[depth + i][x + i] = String::from(RIGHT);
            }
            if !fill_grid(x + mw, depth + mw, node.get(c), node, grid, ud) {
                return false;
            }
        }
        return true;
    }
    if !fill_grid(grid_width / 2, 0, node, node, &mut grid, &mut used_depth) {
        return None;
    }

    // grid to string
    let mut res = String::from("");
    for x in 0..used_depth {
        res += &(grid[x].join("").trim_end().to_string() + "\n")
    }
    Some(res)
}

// visual tests for node
#[cfg(test)]
mod tests {
    use super::super::rbtree::RBTree;
    use super::super::avltree::AVLTree;
    use super::super::tree::{BaseTree, Tree};
    use super::*;

    #[test]
    fn test_easy() {
        let mut t = RBTree::new();
        t.insert(3);
        t.insert(1);
        t.insert(5);
        t.insert(0);
        t.insert(2);
        t.insert(4);
        t.insert(6);

        if let Some(s) = printprettyrb(t.get(t.get_root().unwrap())) {
            println!("{}", s);
        }
        // assert!(false);
    }

    #[test]
    fn test_2() {
        let mut t = RBTree::new();
        for x in 0..20 {
            t.insert(x);
        }
        if let Some(s) = printprettyrb(t.get(t.get_root().unwrap())) {
            println!("{}", s);
        }
        // assert!(false);
    }

    #[test]
    fn test_3() {
        let mut t = RBTree::new();
        for x in &[100, 232, 754, 877, 123, 654, 546, 324, 654, 876] {
            t.insert(x);
        }
        if let Some(s) = printprettyrb(t.get(t.get_root().unwrap())) {
            println!("{}", s);
        }
        // assert!(false);
    }

    #[test]
    fn test_4() {
        let mut t = RBTree::new();
        for x in &[0, 20, -2, 30, -1, -3, 10, 40, -4] {
            t.insert(x);
        }
        if let Some(s) = printprettyrb(t.get(t.get_root().unwrap())) {
            println!("{}", s);
        }
        // assert!(false);
    }

    #[test]
    fn test_avl() {
        let mut t = AVLTree::new();
        for x in &[0, 20, -2, 30, -1, -3, 10, 40, -4] {
            t.insert(x);
        }
        if let Some(s) = printprettyavl(t.get(t.get_root().unwrap())) {
            println!("{}", s);
        }
        // assert!(false);
    }
}
