use std::rc::Rc;
use std::cell::RefCell;

use super::tree::BaseTree;
use super::tree::Tree;

use super::node::Node;
use super::node::*;

#[derive(Debug)]
pub struct ColorNode<T> {
    pub value: T,
    pub ptr: usize,
    pub parent: Option<usize>,
    pub lchild: Option<usize>,
    pub rchild: Option<usize>,
    pub color: Color,
    data: Rc<RefCell<Vec<ColorNode<T>>>>,
}

pub trait ColoredNode<T>: Node<T> {
    fn new(val: T, selfptr: usize, data: Rc<RefCell<Vec<ColorNode<T>>>>) -> Self;
    fn is_red(&self) -> bool;
    fn is_child_black(&self, side: Side) -> bool;
    fn is_parent_black(&self) -> bool;
    fn is_sibling_black(&self) -> bool;
}

impl <T> ColoredNode<T> for ColorNode<T>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialOrd,
{
    fn new(val: T, selfptr: usize, data: Rc<RefCell<Vec<ColorNode<T>>>>) -> Self {
        Self {
            value: val,
            ptr: selfptr,
            parent: None,
            lchild: None,
            rchild: None,
            color: Color::Black,
            data: data
        }
    }

    fn is_red(&self) -> bool {
        match self.color {
            Color::Red => true,
            Color::Black => false,
        }
    }

    // Nil nodes are black children too
    fn is_child_black(&self, side: Side) -> bool{
        let child = self.get_child(side);
        if child.is_some() && self.get(child.unwrap()).is_red() {
            false
        } else {
            true
        }
    }

    // this will panic of called on root node
    fn is_parent_black(&self) -> bool {
        let p = self.parent.unwrap();
        !self.get(p).is_red()
    }

    // Nil nodes are black children too
    fn is_sibling_black(&self) -> bool {
        let sib = self.get_sibling();
        if sib.is_some() && self.get(sib.unwrap()).is_red() {
            false
        } else {
            true
        }
    }
}

impl <T: std::fmt::Debug+std::cmp::PartialOrd> Node<T> for ColorNode<T> {
    fn to_self_string(&self) -> String {
        format!("[P:{:?} C:{:?} V:{:?}]", self.parent, self.color, self.value)
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


    /**
    * In order to return a reference to a value of a vector contained within a
    * refcell, a raw pointer is used. The unsafe code could be avoided by
    * replacing each call to self.get(n) with &self.data.borrow()[n] and each call
    * to self.get_mut(n) with &mut self.data.borrow()[n]
    */
    fn get(&self, ptr: usize) -> &ColorNode<T> {
        unsafe {
            &(*self.data.as_ptr())[ptr]
        }
    }

    fn get_mut(&self, ptr: usize) -> &mut ColorNode<T> {
        unsafe {
            &mut (*self.data.as_ptr())[ptr]
        }
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
pub struct RBTree<T> {
    root: Option<usize>,
    size: usize,
    data: Rc<RefCell<Vec<ColorNode<T>>>>,
    free: Vec<usize>,
}

impl<T> Tree<T> for RBTree<T>
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

impl<T> BaseTree<T> for RBTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    type MNode = ColorNode<T>;
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

    fn rebalance_ins(&mut self, n: usize) {
        self.fix_ins_color(n)
    }

    fn rebalance_del(&mut self, n: usize, child: usize) {
        self.fix_del_color(n, child)
    }

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
                .push(ColorNode::new(val, loc, self.data.clone()));
            loc
        }
    }

    fn delete_node(&mut self, index: usize) {
        self.free.push(index);
    }
}

impl<T> RBTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    // child is the new node in the location, n is being deleted
    fn fix_del_color(&mut self, n: usize, child: usize) {
        dbg!("Fix_del_color");
        if !self.get(n).is_red() {
            if self.get(child).is_red() {
                self.get_mut(child).color = Color::Black;
            } else {
                self.delete_case_1(child);
            }
        }
    }

    // sets a node to black if it exists. This is fine, cause all
    // nodes that don't exist are by definition black anyways
    fn set_maybe_black(&mut self, no: Option<usize>) {
        if let Some(n) = no {
            self.get_mut(n).color = Color::Black;
        }
    }

    fn delete_case_1(&mut self, n: usize) {
        dbg!("delete case 1");
        if self.get(n).parent.is_some() {
            self.delete_case_2(n);
        }
    }

    fn delete_case_2(&mut self, n: usize) {
        dbg!("delete case 2");
        let s = self.get(n).get_sibling();
        if self.get(n).is_sibling_black() {
            let p = self.get(n).parent.expect("D2 P");
            self.set_maybe_black(s);
            self.get_mut(p).color = Color::Red;
            self.rotate(self.get(n).side(), p);
        }
        self.delete_case_3(n);
    }

    fn delete_case_3(&mut self, n: usize) {
        dbg!("delete case 3");
        let s = self.get(n).get_sibling().expect("D3 S");
        let p = self.get(n).parent.expect("D3 P");
        if self.get(n).is_parent_black()
            && !self.get(s).is_red()
            && self.get(s).is_child_black(Side::Left)
            && self.get(s).is_child_black(Side::Right)
        {
            self.delete_case_1(p);
        } else {
            self.delete_case_4(p);
        }
    }

    fn delete_case_4(&mut self, n: usize) {
        dbg!("delete case 4");
        let node = self.get(n);
        let s = node.get_sibling().expect("D4 S");
        let p = node.parent.expect("D4 P");

        if !node.is_parent_black()
            && node.is_sibling_black()
            && self.get(s).is_child_black(Side::Left)
            && self.get(s).is_child_black(Side::Right)
        {
            self.get_mut(s).color = Color::Red;
            self.get_mut(p).color = Color::Black;
        } else {
            self.delete_case_5(n)
        }
    }

    fn delete_case_5(&mut self, n: usize) {
        dbg!("delete case 5");
        let s = self.get(n).get_sibling().expect("D5 S");
        if !self.get(s).is_red() {
            if self.get(n).is_child(Side::Left)
                && self.get(s).is_child_black(Side::Right)
                && !self.get(s).is_child_black(Side::Left)
            {
                let scl = self.get(s).get_child(Side::Left);
                self.get_mut(s).color = Color::Red;
                self.set_maybe_black(scl);
                self.rotate(Side::Right, s);
            } else if self.get(n).is_child(Side::Right)
                && self.get(s).is_child_black(Side::Left)
                && !self.get(s).is_child_black(Side::Right)
            {
                let scr = self.get(s).get_child(Side::Right);
                self.get_mut(s).color = Color::Red;
                self.set_maybe_black(scr);
                self.rotate(Side::Left, s);
            }
        }
        self.delete_case_6(n)
    }

    fn delete_case_6(&mut self, n: usize) {
        dbg!("delete case 6");
        let s = self.get(n).get_sibling().expect("D6 S");
        let p = self.get(n).parent.expect("D6 P");
        let pc = self.get(p).color;
        self.get_mut(s).color = pc;
        self.get_mut(p).color = Color::Black;

        if self.get(n).is_child(Side::Left) {
            let scr = self.get(s).get_child(Side::Right);
            self.set_maybe_black(scr);
            self.rotate(Side::Left, p);
        } else {
            let scl = self.get(s).get_child(Side::Left);
            self.set_maybe_black(scl);
            self.rotate(Side::Right, p);
        }
    }

    fn fix_ins_color(&mut self, n: usize) {
        self.get_mut(n).color = Color::Red;
        if let Some(p) = self.get(n).parent {
            if !self.get(p).is_red() {
                // parent is black
                // do nothing
            } else if self.get(n).get_uncle().is_some()
                && self.get(self.get(n).get_uncle().unwrap()).is_red()
            {
                // uncle exists and is red
                let p = self.get(n).parent.unwrap();
                let u = self.get(n).get_uncle().unwrap();
                self.get_mut(p).color = Color::Black;
                self.get_mut(u).color = Color::Black;
                self.fix_ins_color(self.get(p).parent.unwrap());
            } else {
                // uncle is black
                self.do_ins_hard_case(n);
            }
        } else {
        }
        self.get_mut(self.root.unwrap()).color = Color::Black;
    }

    fn do_ins_hard_case(&mut self, nn: usize) {
        let mut n = nn;
        let mut p = self.get(n).parent.unwrap();
        if self.get(p).is_child(Side::Left) && self.get(n).is_child(Side::Right) {
            self.rotate(Side::Left, n);
            n = self.get(n).get_child(Side::Left).unwrap();
        }

        p = self.get(n).parent.unwrap();
        if self.get(p).is_child(Side::Right) && self.get(n).is_child(Side::Left) {
            self.rotate(Side::Right, n);
            n = self.get(n).get_child(Side::Right).unwrap();
        }
        self.do_ins_hard_case2(n);
    }

    fn do_ins_hard_case2(&mut self, n: usize) {
        let p = self.get(n).parent.unwrap();
        let g = self.get(p).parent.unwrap();

        self.get_mut(p).color = Color::Black;
        self.get_mut(g).color = Color::Red;
        if self.get(p).is_child(Side::Right) {
            self.rotate(Side::Left, p);
        } else if self.get(p).is_child(Side::Right) {
            self.rotate(Side::Right, p);
        }
    }

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

    /** ([P:None C:Black V:50
        ([P:Some(0) C:Black V:25
            ([P:Some(1) C:Black V:15
                ([P:Some(3) C:Black V:0])
                ([P:Some(3) C:Black V:20])])
            ([P:Some(1) C:Black V:35
                ([P:Some(4) C:Black V:30])
                ([P:Some(4) C:Black V:40])])])
        ([P:Some(0) C:Black V:75
            ([P:Some(2) C:Black V:65
                ([P:Some(5) C:Black V:60])
                ([P:Some(5) C:Black V:70])])
            ([P:Some(2) C:Black V:85
                ([P:Some(6) C:Black V:80])
                ([P:Some(6) C:Black V:100])])])])
    */
    fn make_fake_tree_node_no_balance() -> RBTree<i32> {
        let mut tree = RBTree::new();
        for x in vec![50, 25, 75, 15, 35, 65, 85, 0, 20, 30, 40, 60, 70, 80, 100] {
            tree.insert(x);
        }

        tree
    }

    #[test]
    fn test_tree_print() {
        let tree = make_fake_tree_node_no_balance();
        assert_eq!(tree.to_string(), "([P:None C:Black V:50] ([P:Some(0) C:Red V:25] ([P:Some(1) C:Black V:15] ([P:Some(3) C:Red V:0] () ()) ([P:Some(3) C:Red V:20] () ())) ([P:Some(1) C:Black V:35] ([P:Some(4) C:Red V:30] () ()) ([P:Some(4) C:Red V:40] () ()))) ([P:Some(0) C:Red V:75] ([P:Some(2) C:Black V:65] ([P:Some(5) C:Red V:60] () ()) ([P:Some(5) C:Red V:70] () ())) ([P:Some(2) C:Black V:85] ([P:Some(6) C:Red V:80] () ()) ([P:Some(6) C:Red V:100] () ()))))");
        let tree_empty = RBTree::<i32>::new();
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
        let mut tree = RBTree::<i32>::new();
        for x in 0..10 {
            double_size_test(&tree, x as usize);
            tree.insert(x);
            double_size_test(&tree, (x + 1) as usize);
        }

        assert_eq!(tree.to_string(), "([P:None C:Black V:3] ([P:Some(3) C:Black V:1] ([P:Some(1) C:Black V:0] () ()) ([P:Some(1) C:Black V:2] () ())) ([P:Some(3) C:Black V:5] ([P:Some(5) C:Black V:4] () ()) ([P:Some(5) C:Red V:7] ([P:Some(7) C:Black V:6] () ()) ([P:Some(7) C:Black V:8] () ([P:Some(8) C:Red V:9] () ())))))");
    }

    fn double_size_test<T: PartialEq + PartialOrd + std::fmt::Debug>(
        tree: &RBTree<T>,
        expect: usize,
    ) {
        assert_eq!(tree.get_size(), expect);
        assert_eq!(tree.get_size_recursive(), expect);
    }

    #[test]
    fn test_height() {
        let tree = make_fake_tree_node_no_balance();
        assert_eq!(tree.get_height(), 4);

        let mut tree2 = RBTree::<i32>::new();
        for x in 0..10 {
            tree2.insert(x);
        }
        assert_eq!(tree2.get_height(), 5);

        let tree3 = RBTree::<i32>::new();
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
        println!("{}", tree.to_pretty_string());
        tree.delete(80);
        println!("{}", tree.to_pretty_string());
        tree.delete(85);
        assert_eq!(tree.to_string(), "uuhhh");
    }
}
