use super::node::Node;
use super::node::*;

/**
 * Arena based memory tree structure
*/
#[derive(Debug)]
pub struct Tree<T> {
    root: Option<usize>,
    size: usize,
    data: Vec<Node<T>>,
    free: Vec<usize>,
}



trait Tree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    pub fn new() -> Self;

    pub fn contains(&self, val: &T) -> bool {
        let n = self.find(val);
        &Node::get(&self.data, n).value == val
    }

    pub fn insert(&mut self, val: T);

    pub fn delete(&mut self, val: T) -> bool;

    fn replace_node(&mut self, to_delete: usize, to_attach: Option<usize>) {
        let node = Node::get(&self.data, to_delete);
        if let Some(p) = node.parent {
            if node.is_child(&self.data, Side::Left) {
                Node::get_mut(&mut self.data, p).lchild = to_attach;
            } else {
                Node::get_mut(&mut self.data, p).rchild = to_attach;
            }
        } else {
            self.root = to_attach;
        }
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }

    fn find(&self, val: &T) -> usize {
        let mut n = self.root.unwrap();
        loop {
            let node = Node::get(&self.data, n);
            if &node.value < val && node.rchild.is_some() {
                n = node.rchild.unwrap();
            } else if &node.value > val && node.lchild.is_some() {
                n = node.lchild.unwrap();
            } else {
                return n;
            }
        }
    }

    pub fn to_string(&self) -> String {
        if let Some(root) = self.root {
            Node::get(&self.data, root).to_string(&self.data)
        } else {
            String::from("(Empty tree)")
        }
    }

    fn rotate(&mut self, side: Side, n: usize) {
        let p = Node::get(&self.data, n).parent.unwrap();

        if let Some(c) = Node::get(&self.data, n).get_child(side) {
            Node::set_child(&mut self.data, p, c, !side);
        } else {
            match !side {
                Side::Left => Node::get_mut(&mut self.data, p).lchild = None,
                Side::Right => Node::get_mut(&mut self.data, p).rchild = None,
            }
        }
        if let Some(g) = Node::get(&self.data, p).parent {
            Node::get_mut(&mut self.data, n).parent = Some(g);
            let pside = if Node::get(&self.data, p).is_child(&self.data, Side::Left) {
                Side::Left
            } else {
                Side::Right
            };
            Node::set_child(&mut self.data, g, n, pside);
        } else {
            self.root = Some(n);
            Node::get_mut(&mut self.data, n).parent = None
        }
        Node::set_child(&mut self.data, n, p, side);
    }

    pub fn get_size_recursive(&self) -> usize {
        if let Some(root) = self.root {
            Node::get(&self.data, root).get_size(&self.data)
        } else {
            0
        }
    }

    pub fn get_height(&self) -> usize {
        if let Some(root) = self.root {
            Node::get(&self.data, root).get_height(&self.data)
        } else {
            0
        }
    }

    fn create_node(&mut self, val: T) -> usize {
        // update this so it reuses deleted slots
        if self.free.len() > 0 {
            let n = self.free.pop().expect("pop should not fail if len > 0");
            self.data[n].ptr = n;
            self.data[n].lchild = None;
            self.data[n].rchild = None;
            self.data[n].parent = None;
            n
        } else {
            let loc = self.data.len();
            self.data.push(Node::new(val, loc));
            loc
        }
    }

    fn delete_node(&mut self, index: usize) {
        self.free.push(index);
    }
}

trait RbTree<T>
where 
    T: Tree,
{
    
    pub fn fix_del_color(&mut self, n: usize, del: usize);
    fn delete_replace(&mut self, n: usize) -> usize;
    fn fix_ins_color(&mut self, n: usize) -> ();
    fn do_ins_hard_case(&mut self, nn: usize) -> ();
    fn do_ins_hard_case2(&mut self, n: usize) -> ();
}
