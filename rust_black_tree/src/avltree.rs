use std::cell::RefCell;
use std::rc::Rc;

use super::tree::BaseTree;
use super::tree::Tree;

use super::node::Node;
use super::node::*;

#[derive(Debug)]
pub struct AVLNode<T> {
    pub value: T,
    pub ptr: usize,
    pub parent: Option<usize>,
    pub lchild: Option<usize>,
    pub rchild: Option<usize>,
    data: Rc<RefCell<Vec<AVLNode<T>>>>,
    // For AVL nodes...
    pub height: isize,
    pub balance_factor: isize,
}

impl<T> AVLNode<T> {
    fn new(val: T, selfptr: usize, data: Rc<RefCell<Vec<AVLNode<T>>>>) -> Self {
        Self {
            value: val,
            ptr: selfptr,
            parent: None,
            lchild: None,
            rchild: None,
            data: data,
            height: 1,
            balance_factor: 0,
        }
    }
}

impl<T: std::fmt::Debug + std::cmp::PartialOrd> Node<T> for AVLNode<T> {
    fn to_self_string(&self) -> String {
        format!(
            "[V:{:?} H:{:?} BF:{:?}]",
            self.value, self.height, self.balance_factor
        )
    }
    fn get_value(&self) -> &T {
        &self.value
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
    fn get(&self, ptr: usize) -> &AVLNode<T> {
        unsafe { &(*self.data.as_ptr())[ptr] }
    }

    fn get_mut(&self, ptr: usize) -> &mut AVLNode<T> {
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
pub struct AVLTree<T> {
    root: Option<usize>,
    size: usize,
    data: Rc<RefCell<Vec<AVLNode<T>>>>,
    free: Vec<usize>,
}

impl<T> Tree<T> for AVLTree<T>
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

impl<T> BaseTree<T> for AVLTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    type MNode = AVLNode<T>;
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
        self.retrace(n);
    }

    fn rebalance_del(&mut self, _n: usize, _child: usize) {}

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
                .push(AVLNode::new(val, loc, self.data.clone()));
            loc
        }
    }

    fn delete_node(&mut self, index: usize) {
        self.free.push(index);
    }
}

impl<T> AVLTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    fn retrace(&mut self, z: usize) {
        loop {
            //println!("Z= {:?}", self.get(z).value);
            //println!("X= {:?}", self.get(x).value);
            // get the parent of current node
            let x = self.get(z).parent;
            if !x.is_some() {
                // current node z is the root of the tree
                // nothing to do, return?
                return;
            }
            let x: usize = x.expect("Retrace get z parent");

            if self.get(z).is_child(Side::Right) {
                // The right subtree increases
                if self.is_heavy_on_side(Side::Right, x) {
                    if self.is_heavy_on_side(Side::Left, z) {
                        self.avl_rotate(Side::Right, z);
                        self.avl_rotate(Side::Left, x);
                    } else {
                        // TODO: rotates panic rn
                        // wiki has a differnet definiton of
                        // rotate than we do I think
                        self.avl_rotate(Side::Left, x);
                        //self.rotate(Side::Left, z);
                    }
                } else {
                    if self.is_heavy_on_side(Side::Left, x) {
                        self.set_balance_factor(x, 0);
                        break;
                    }
                    self.set_balance_factor(x, 1);
                    //Z = X; // Height(Z) increases by 1
                    //z = x;
                    self.retrace(x);
                    //continue;
                }
            } else {
                if self.is_heavy_on_side(Side::Left, x) {
                    if self.is_heavy_on_side(Side::Right, z) {
                        self.avl_rotate(Side::Left, z);
                        self.avl_rotate(Side::Right, x);
                    } else {
                        self.avl_rotate(Side::Right, x);
                    }
                } else {
                    if self.is_heavy_on_side(Side::Right, x) {
                        self.set_balance_factor(x, 0);
                        break; // Leave the loop
                    }
                    self.set_balance_factor(x, -1);
                    //Z = X; // Height(Z) increases by 1
                    //z = x;
                    self.retrace(x);
                    //continue;
                }
            }
            break;
        }
        // Unless loop is left via break, the height of the total tree increases by 1.
    }

    fn avl_rotate(&mut self, side: Side, n: usize) {
        // make an adjustment to account for differnt rotate
        // algorithm off wiki than implemented in tree...
        // ALSO adjust the balance factors
        if let Some(z) = self.get(n).get_child(!side) {
	        self.rotate(
	            side,
	            z
	        );
            match self.get_balance_factor(z)  {
                0 =>
                    {
                    self.set_balance_factor(n,1);
                    self.set_balance_factor(z,-1);
                    },
                _ =>
                    {
                    self.set_balance_factor(n,0);
                    self.set_balance_factor(z, 0);
                    },
            }
        } else {
            panic!("avl rotate unwrap");
        }

    }

    fn get_balance_factor(&self, n: usize) -> isize {
        self.get(n).balance_factor
    }

    fn set_balance_factor(&mut self, n: usize, bf: isize) {
        self.get_mut(n).balance_factor = bf;
    }

    fn is_heavy_on_side(&self, side: Side, n: usize) -> bool {
        // check the balance factor on side of node n
        match side {
            Side::Right => self.get_balance_factor(n) > 0,
            Side::Left => self.get_balance_factor(n) < 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _tree = AVLTree::<i32>::new();
    }

    #[test]
    fn insert_one() {
        let mut tree = AVLTree::<i32>::new();
        tree.insert(1);
        let root = tree.root.expect("tree root");
        assert_eq!(tree.get_balance_factor(root), 0);
        assert!(tree.is_heavy_on_side(Side::Right, root) == false);
        assert!(tree.is_heavy_on_side(Side::Left, root) == false);
    }

    #[test]
    fn balance_factor_helpers() {
        let mut tree = AVLTree::<i32>::new();
        tree.insert(1);
        let root = tree.root.expect("tree root");
        tree.set_balance_factor(root, 1);
        assert!(tree.is_heavy_on_side(Side::Right, root));
        tree.set_balance_factor(root, -1);
        assert!(tree.is_heavy_on_side(Side::Left, root));
    }

    #[test]
    fn insert_few() {
        let mut tree = AVLTree::<i32>::new();
        tree.insert(1);
        tree.insert(2);
        let root = tree.root.expect("tree root");
        assert!(tree.is_heavy_on_side(Side::Right, root));

        tree.insert(3);

        // assert that tree is balanced
        //assert_eq!(tree.to_string(), "UUHH");
        assert!(tree.is_heavy_on_side(Side::Right, root) == false);
        assert!(tree.is_heavy_on_side(Side::Left, root) == false);
    }

    #[test]
    fn rotate_crash_test() {
        // puts the smallest tree through all the combos
        // of rebalance rotations
        let mut tree = AVLTree::<i32>::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        let mut tree = AVLTree::<i32>::new();
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);

        let mut tree = AVLTree::<i32>::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        let mut tree = AVLTree::<i32>::new();
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        assert!(true);
    }

    //    #[test]
    //    fn insert_many() {
    //        let mut tree = AVLTree::<i32>::new();
    //        for i in 1..10 {
    //            tree.insert(i);
    //        }
    //        println!("{}", tree.to_pretty_string());
    //        assert_eq!("A BALANCED REEE", tree.to_pretty_string());
    //
    //    }
}
