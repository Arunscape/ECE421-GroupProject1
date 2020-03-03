use std::cell::RefCell;
use std::rc::Rc;

use super::tree::BaseTree;
use super::tree::Tree;

use super::node::Node;
use super::node::*;

/// a nice convenient macro which allows a user to initialize a tree with
/// a number of elements
/// usage: bst!{1, 2, 3, 4, 5, 6, 7, 8, 9, 0};
#[macro_export]
macro_rules! bst {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_tree = BSTree::new();
            $(
                temp_tree.insert($x);
            )*
            temp_tree
        }
    };
}

#[derive(Debug)]
pub struct RegularNode<T> {
    pub value: T,
    pub ptr: usize,
    pub parent: Option<usize>,
    pub lchild: Option<usize>,
    pub rchild: Option<usize>,
    data: Rc<RefCell<Vec<RegularNode<T>>>>,
}

impl<T> RegularNode<T> {
    fn new(val: T, selfptr: usize, data: Rc<RefCell<Vec<RegularNode<T>>>>) -> Self {
        Self {
            value: val,
            ptr: selfptr,
            parent: None,
            lchild: None,
            rchild: None,
            data: data,
        }
    }
}

impl<T: std::fmt::Debug + std::cmp::PartialOrd> Node<T> for RegularNode<T> {
    fn to_self_string(&self) -> String {
        format!("[P:{:?} V:{:?}]", self.parent, self.value)
    }

    fn is(&self, val: &T) -> bool {
        &self.value == val
    }
    fn greater(&self, val: &T) -> bool {
        &self.value > val
    }
    fn lesser(&self, val: &T) -> bool {
        &self.value < val
    }

    fn get_value(&self) -> &T {
        &self.value
    }
    /**
     * In order to return a reference to a value of a vector contained within a
     * refcell, a raw pointer is used. The unsafe code could be avoided by
     * replacing each call to self.get(n) with &self.data.borrow()[n] and each call
     * to self.get_mut(n) with &mut self.data.borrow()[n]
     */
    fn get(&self, ptr: usize) -> &RegularNode<T> {
        unsafe { &(*self.data.as_ptr())[ptr] }
    }

    fn get_mut(&self, ptr: usize) -> &mut RegularNode<T> {
        unsafe { &mut (*self.data.as_ptr())[ptr] }
    }

    fn get_child(&self, side: Side) -> Option<usize> {
        match side {
            Side::Left => self.lchild,
            Side::Right => self.rchild,
        }
    }

    fn set_child(&mut self, child: usize, side: Side) {
        self.set_child_opt(Some(child), side)
    }

    fn set_child_opt(&mut self, c: Option<usize>, side: Side) {
        match side {
            Side::Left => self.lchild = c,
            Side::Right => self.rchild = c,
        };
        if let Some(child) = c {
            self.get_mut(child).parent = Some(self.location());
        }
    }
    fn set_parent(&mut self, p: Option<usize>) {
        self.parent = p;
    }

    fn get_parent(&self) -> Option<usize> {
        self.parent
    }

    fn location(&self) -> usize {
        self.ptr
    }
}

/**
 * Arena based memory tree structure
*/
#[derive(Debug)]
pub struct BSTree<T> {
    root: Option<usize>,
    size: usize,
    data: Rc<RefCell<Vec<RegularNode<T>>>>,
    free: Vec<usize>,
}

impl<T> Tree<T> for BSTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    fn new() -> Self {
        Self {
            root: None,
            data: Rc::new(RefCell::new(Vec::new())),
            size: 0,
            free: Vec::new(),
        }
    }
}

impl<T> BaseTree<T> for BSTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    type MNode = RegularNode<T>;
    /**
     * In order to return a reference to a value of a vector contained within a refcell, a raw
     * pointer is used. The unsafe code could be avoided by replacing each call to self.get(n) with
     * &self.data.borrow()[n] and each call to self.get_mut(n) with &mut self.data.borrow()[n]. This
     * allows us to do the same thing with less keystrokes. It does make the program not
     * thread-safe, but a this data structure is a pretty terrible choice for a multi-threaded data
     * structure anyways, since re-balancing can require that most of the tree be locked to one
     * thread during an insertion or deletion
     */
    fn get(&self, val: usize) -> &Self::MNode {
        unsafe { &(*self.data.as_ptr())[val] }
    }

    fn get_mut(&self, val: usize) -> &mut Self::MNode {
        unsafe { &mut (*self.data.as_ptr())[val] }
    }

    fn get_root(&self) -> Option<usize> {
        self.root
    }

    fn set_root(&mut self, new_root: Option<usize>) {
        self.root = new_root
    }

    fn crement_size(&mut self, amount: isize) {
        self.size = (self.size as isize + amount) as usize;
    }

    fn attach_child(&self, p: usize, c: usize, side: Side) {
        self.get_mut(p).set_child(c, side)
    }

    fn rebalance_ins(&mut self, n: usize) {}

    fn rebalance_del(&mut self, n: usize, child: usize) {}

    fn delete_replace(&mut self, n: usize) -> usize {
        let node = self.get(n);
        match (node.lchild, node.rchild) {
            (Some(lc), Some(rc)) => {
                let p = node.parent;
                let successor = self.get(rc).find_min();
                self.delete_replace(successor);
                self.data.borrow_mut().swap(successor, n);

                self.get_mut(n).lchild = Some(lc);
                self.get_mut(n).rchild = Some(rc);
                self.get_mut(n).parent = p;
                self.get_mut(n).ptr = n;
                return successor;
            }
            (None, Some(_rc)) => self.replace_node(n, self.get(n).rchild),
            (Some(_lc), None) => self.replace_node(n, self.get(n).lchild),
            (None, None) => self.replace_node(n, None),
        };
        n
    }

    fn replace_node(&mut self, to_delete: usize, to_attach: Option<usize>) {
        let node = self.get(to_delete);
        if let Some(p) = node.parent {
            if node.is_child(Side::Left) {
                self.get_mut(p).lchild = to_attach;
            } else {
                self.get_mut(p).rchild = to_attach;
            }
        } else {
            self.root = to_attach;
        }
    }

    fn get_size(&self) -> usize {
        return self.size;
    }

    fn create_node(&mut self, val: T) -> usize {
        // update this so it reuses deleted slots
        if self.free.len() > 0 {
            let n = self.free.pop().expect("pop should not fail if len > 0");
            let mut d = self.get_mut(n);
            d.ptr = n;
            d.lchild = None;
            d.rchild = None;
            d.parent = None;
            n
        } else {
            let loc = self.data.borrow().len();
            self.data
                .borrow_mut()
                .push(RegularNode::new(val, loc, self.data.clone()));
            loc
        }
    }

    fn delete_node(&mut self, index: usize) {
        self.free.push(index);
    }
}

impl<T> BSTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    fn get_size_recursive(&self) -> usize {
        if let Some(root) = self.root {
            self.get(root).get_size()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_fake_tree_node_no_balance() -> BSTree<i32> {
        let mut tree = BSTree::new();
        for x in vec![50, 25, 75, 15, 35, 65, 85, 0, 20, 30, 40, 60, 70, 80, 100] {
            tree.insert(x);
        }

        tree
    }

    #[test]
    fn test_tree_print() {
        let tree = make_fake_tree_node_no_balance();
        assert_eq!(tree.to_string(), "([P:None V:50] ([P:Some(0) V:25] ([P:Some(1) V:15] ([P:Some(3) V:0] () ()) ([P:Some(3) V:20] () ())) ([P:Some(1) V:35] ([P:Some(4) V:30] () ()) ([P:Some(4) V:40] () ()))) ([P:Some(0) V:75] ([P:Some(2) V:65] ([P:Some(5) V:60] () ()) ([P:Some(5) V:70] () ())) ([P:Some(2) V:85] ([P:Some(6) V:80] () ()) ([P:Some(6) V:100] () ()))))");
        let tree_empty = BSTree::<i32>::new();
        assert_eq!(tree_empty.to_string(), "(Empty tree)");
    }

    #[test]
    fn test_contains() {
        let tree = make_fake_tree_node_no_balance();
        assert!(tree.contains(&100));
        assert!(tree.contains(&0));
        assert!(tree.contains(&50));
        assert!(tree.contains(&25));
        assert!(tree.contains(&75));
        assert!(tree.contains(&60));
        assert!(tree.contains(&40));

        assert!(!tree.contains(&42));
        assert!(!tree.contains(&99));
        assert!(!tree.contains(&1));
    }

    #[test]
    fn test_insert() {
        let mut tree = BSTree::<i32>::new();
        for x in 0..10 {
            double_size_test(&tree, x as usize);
            tree.insert(x);
            double_size_test(&tree, (x + 1) as usize);
        }

        assert_eq!(tree.to_string(), "([P:None V:0] () ([P:Some(0) V:1] () ([P:Some(1) V:2] () ([P:Some(2) V:3] () ([P:Some(3) V:4] () ([P:Some(4) V:5] () ([P:Some(5) V:6] () ([P:Some(6) V:7] () ([P:Some(7) V:8] () ([P:Some(8) V:9] () ()))))))))))");
    }

    fn double_size_test<T: PartialEq + PartialOrd + std::fmt::Debug>(
        tree: &BSTree<T>,
        expect: usize,
    ) {
        assert_eq!(tree.get_size(), expect);
        assert_eq!(tree.get_size_recursive(), expect);
    }

    #[test]
    fn test_height() {
        let tree = make_fake_tree_node_no_balance();
        assert_eq!(tree.get_height(), 4);

        let mut tree2 = BSTree::<i32>::new();
        for x in 0..10 {
            tree2.insert(x);
        }
        assert_eq!(tree2.get_height(), 10);

        let tree3 = BSTree::<i32>::new();
        assert_eq!(tree3.get_height(), 0);
    }

    #[test]
    fn test_delete_mem() {
        let mut tree = make_fake_tree_node_no_balance();
        double_size_test(&tree, 15);

        tree.delete(1);
        double_size_test(&tree, 15);

        tree.delete(0);
        double_size_test(&tree, 14);
        assert_eq!(tree.data.borrow().len(), 15);
        tree.insert(0);
        assert_eq!(tree.data.borrow().len(), 15);
        double_size_test(&tree, 15);

        tree.delete(50);
        double_size_test(&tree, 14);
        assert_eq!(tree.data.borrow().len(), 15);
        tree.insert(50);
        assert_eq!(tree.data.borrow().len(), 15);
        double_size_test(&tree, 15);
    }

    #[test]
    fn test_delete() {
        let mut tree = make_fake_tree_node_no_balance();
        double_size_test(&tree, 15);
        tree.delete(100);
        tree.delete(80);
        tree.delete(85);
        assert_eq!(tree.to_string(), "([P:None V:50] ([P:Some(0) V:25] ([P:Some(1) V:15] ([P:Some(3) V:0] () ()) ([P:Some(3) V:20] () ())) ([P:Some(1) V:35] ([P:Some(4) V:30] () ()) ([P:Some(4) V:40] () ()))) ([P:Some(0) V:75] ([P:Some(2) V:65] ([P:Some(5) V:60] () ()) ([P:Some(5) V:70] () ())) ()))");
    }
}
