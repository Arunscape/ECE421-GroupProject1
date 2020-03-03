use super::node::Node;
use super::node::Side;
use super::tree::BaseTree;
use super::tree::Tree;

use super::rbtree::ColorNode;
use super::avltree::AVLNode;
use super::unbalancetree::RegularNode;
use super::rbtree::RBTree;

const LEFT: char = '╱';
const RIGHT: char = '╲';

pub fn printprettybst<T: std::fmt::Debug + std::cmp::PartialOrd>(node: &RegularNode<T>) -> Option<String> {
    print_node_pretty(node)
}

pub fn printprettyrb<T: std::fmt::Debug + std::cmp::PartialOrd>(node: &ColorNode<T>) -> Option<String> {
    print_node_pretty(node)
}

pub fn printprettyavl<T: std::fmt::Debug + std::cmp::PartialOrd>(node: &AVLNode<T>) -> Option<String> {
    print_node_pretty(node)
}


fn print_node_pretty<T: std::fmt::Debug, N: Node<T>>(node: &N) -> Option<String> {
    let (grid_width, grid_height) =
        if let Some((w, h)) = term_size::dimensions() {
            (w, h)
        } else {
            (150, 100)
        };
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(grid_height);
    let mut used_depth = 0;
    // make grid
    for row in 0..grid_height {
        grid.push(Vec::with_capacity(grid_width));
        for _ in 0..grid_width {
            grid[row].push(' ');
        }
    }

    // fill grid
    fn fill_grid<T: std::fmt::Debug, N: Node<T>>(
        x: usize,
        depth: usize,
        n: &N,
        node: &N,
        grid: &mut Vec<Vec<char>>,
        ud: &mut usize,
    ) -> bool {
        let val_str = format!("{:?}", n.get_value());
        let cw = val_str.len();
        let lw = n
            .get_child(Side::Left)
            .map(|x| node.get(x).get_size())
            .unwrap_or(0);
        let rw = n
            .get_child(Side::Right)
            .map(|x| node.get(x).get_size())
            .unwrap_or(0);
        let mw = std::cmp::max(lw, rw) * cw;
        if depth >= grid.len() {
            return false;
        }
        if x >= grid[0].len() {
            return false;
        }

        // write node
        let mut i = 0;
        for c in val_str.chars() {
            grid[depth][x + i - cw/2] = c;
            i += 1;
        }
        *ud = std::cmp::max(*ud, depth) + 1;
        if let Some(c) = n.get_child(Side::Left) {
            for i in 1..mw {
                grid[depth + i][x - i] = LEFT;
            }
            if !fill_grid(x - mw, depth + mw, node.get(c), node, grid, ud) {
                return false;
            }
        }
        if let Some(c) = n.get_child(Side::Right) {
            for i in 1..mw {
                grid[depth + i][x + i] = RIGHT;
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
        res += &(grid[x].iter().collect::<String>() + "\n")
    }
    Some(res)
}


// visual tests for node
#[cfg(test)]
mod tests {
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
}
