use super::node::Node;
use super::node::*;

pub trait BaseTree<T> {
    type MNode: Node<T>;
    fn get(&self, val: usize) -> &Self::MNode;
    fn get_mut(&self, val: usize) -> &mut Self::MNode;

    fn delete_node(&mut self, index: usize);
    fn create_node(&mut self, val: T) -> usize;

    fn rebalance_ins(&mut self, n: usize);
    fn rebalance_del(&mut self, n: usize, child: usize);

    fn delete_replace(&mut self, n: usize) -> usize;
    fn replace_node(&mut self, to_delete: usize, to_attach: Option<usize>);

    fn attach_child(&self, p: usize, c: usize, side: Side);

    fn get_root(&self) -> Option<usize>;
    fn set_root(&mut self, new_root: Option<usize>);

    fn get_size(&self) -> usize;
    fn crement_size(&mut self, val: isize);
}

pub trait Tree<T: std::fmt::Debug>: BaseTree<T> {
    fn new() -> Self;

    fn contains(&self, val: &T) -> bool {
        let n = self.find(val);
        self.get(n).is(val)
    }

    fn insert(&mut self, val: T) {
        if let Some(_root) = self.get_root() {
            let n = self.find(&val);
            let node = self.get(n);
            if node.is(&val) {
                // value already in tree
                self.crement_size(-1);
            } else {
                let side = if node.lesser(&val) {
                    Side::Right
                } else {
                    Side::Left
                };
                let node = self.create_node(val);
                self.attach_child(n, node, side);
                self.rebalance_ins(node);
            }
        } else {
            let r = Some(self.create_node(val));
            self.set_root(r);
        }
        self.crement_size(1);
    }

    fn delete(&mut self, val: T) -> bool {
        if !self.contains(&val) {
            false
        } else {
            let n = self.find(&val);
            let del = self.delete_replace(n);
            self.rebalance_del(del, n);
            self.delete_node(del);
            self.crement_size(-1);
            true
        }
    }

    fn rotate(&mut self, side: Side, n: usize) {
        let p = self.get(n).get_parent().expect("P rotate");

        if let Some(c) = self.get(n).get_child(side) {
            self.attach_child(p, c, !side);
        } else {
            match !side {
                Side::Left => self.get_mut(p).set_child_opt(None, Side::Left),
                Side::Right => self.get_mut(p).set_child_opt(None, Side::Right),
            }
        }
        if let Some(g) = self.get(p).get_parent() {
            self.get_mut(n).set_parent(Some(g));
            let pside = if self.get(p).is_child(Side::Left) {
                Side::Left
            } else {
                Side::Right
            };
            self.attach_child(g, n, pside);
        } else {
            self.set_root(Some(n));
            self.get_mut(n).set_parent(None);
        }
        self.attach_child(n, p, side);
    }

    fn find(&self, val: &T) -> usize {
        let mut n = self.get_root().expect("n find");
        loop {
            let node = self.get(n);
            if node.lesser(val) && node.get_child(Side::Right).is_some() {
                n = node.get_child(Side::Right).expect("find n right child");
            } else if node.greater(val) && node.get_child(Side::Left).is_some() {
                n = node.get_child(Side::Left).expect("find n left child");
            } else {
                return n;
            }
        }
    }

    fn get_height(&self) -> usize {
        if let Some(root) = self.get_root() {
            self.get(root).get_height()
        } else {
            0
        }
    }

    fn to_string(&self) -> String {
        if let Some(root) = self.get_root() {
            self.get(root).to_string()
        } else {
            String::from("(Empty tree)")
        }
    }

    fn to_pretty_string(&self) -> String {
        if let Some(root) = self.get_root() {
            self.get(root).to_pretty_string(1)
        } else {
            String::from("(Empty tree)")
        }
    }
}
