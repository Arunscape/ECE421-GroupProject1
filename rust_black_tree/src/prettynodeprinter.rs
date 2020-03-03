use super::node::Side;
use super::node::Node;
use super::tree::Tree;
use super::tree::BaseTree;

use super::rbtree::ColorNode;
use super::rbtree::RBTree;

use std::collections::VecDeque;

pub fn printpretty<T: std::fmt::Debug+std::cmp::PartialOrd> (node: &ColorNode<T>) -> String {
    print_node_pretty(node)

}

fn bfs_print_node<T: std::fmt::Debug, N: Node<T>>
    (depth: &mut usize, x: &mut usize, res: &mut String,
                  nodes: &mut VecDeque<Option<usize>>, node: &N) {
    if let Some(n) = nodes.pop_front() {
        if let Some(n) = n {
            let n = node.get(n);
            let d = n.get_depth();
            let x_new = get_x(n);
            if d > *depth {
                *depth = d;
                *x = 0;
                *res += "\n";
            }
            if x_new > *x {
                *res += &" ".repeat(x_new - *x);
                *x = x_new;
            }
            *res += &format!("{}", n.location());
            nodes.push_back(n.get_child(Side::Left));
            nodes.push_back(n.get_child(Side::Right));
        }
        bfs_print_node(depth, x, res, nodes, node);
    }
}

fn print_node_pretty<T: std::fmt::Debug, N: Node<T>>(node: &N) -> String {
    let mut nodes: VecDeque<Option<usize>> = VecDeque::new();
    nodes.push_back(Some(node.location()));
    let mut depth = node.get_depth();
    let mut x = 0;
    let mut res = String::new();
    bfs_print_node(&mut depth, &mut x, &mut res, &mut nodes, node);
    res
}

fn get_x<T: std::fmt::Debug, N: Node<T>>(node: &N) -> usize {
    const NODE_WIDTH: usize = 5;
    fn inner<T: std::fmt::Debug, N: Node<T>>(n: Option<&N>, node: &N) -> usize {
        if let Some(n) = n {
            let cl = inner(n.get_child(Side::Left).map(|x| node.get(x)), node);
            let cr = inner(n.get_child(Side::Right).map(|x| node.get(x)), node);
            (if n.greater(node.get_value()) { 0 } else { NODE_WIDTH }) + cl + cr
        } else {
            0
        }
    }
    fn rooter<T: std::fmt::Debug, N: Node<T>>(n: &N) -> &N {
        if let Some(p) = n.get_parent() {
            rooter(n.get(p))
        } else {
            n
        }
    }
    let root = rooter(node);
    inner(Some(root), node) - node.get_depth()
}

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

        println!("{}", printpretty(t.get(t.get_root().unwrap())));
        assert!(false);
    }

    #[test]
    fn test_2() {
        let mut t = RBTree::new();
        for x in 0..20 {t.insert(x);}
        println!("{}", printpretty(t.get(t.get_root().unwrap())));
        assert!(false);
    }

}
