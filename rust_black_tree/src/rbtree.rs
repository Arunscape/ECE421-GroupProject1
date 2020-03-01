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

impl<T> Tree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            root: None,
            data: Vec::new(),
            size: 0,
            free: Vec::new(),
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        let n = self.find(val);
        &Node::get(&self.data, n).value == val
    }

    pub fn insert(&mut self, val: T) {
        if let Some(_root) = self.root {
            let n = self.find(&val);
            let node = Node::get(&self.data, n);
            if node.value == val {
                // value already in tree
                self.size -= 1;
            } else {
                let side = if node.value < val {
                    Side::Right
                } else {
                    Side::Left
                };
                let node = self.create_node(val);
                Node::set_child(&mut self.data, n, node, side);
                self.fix_ins_color(node);
            }
        } else {
            self.root = Some(self.create_node(val));
        }
        self.size += 1;
    }

    pub fn delete(&mut self, val: T) -> bool {
        if !self.contains(&val) {
            false
        } else {
            let n = self.find(&val);
            let del = self.delete_replace(n);
            self.delete_node(del);
            self.size -= 1;
            self.fix_del_color(del, n);
            true
        }
    }

    // child is the new node in the location, n is being deleted
    pub fn fix_del_color(&mut self, n: usize, child: usize) {
        dbg!("Fix_del_color");
        if !Node::get(&self.data, n).is_red() {
            if Node::get(&self.data, child).is_red() {
                Node::get_mut(&mut self.data, child).color = Color::Black;
            } else {
                self.delete_case_1(child);
            }
        }
    }

    // sets a node to black if it exists. This is fine, cause all
    // nodes that don't exist are by definition black anyways
    fn set_maybe_black (&mut self, no: Option<usize>) {
        if let Some(n) = no {
            Node::get_mut(&mut self.data, n).color = Color::Black;
        }
    }

    fn delete_case_1(&mut self, n: usize) {
        dbg!("delete case 1");
        if Node::get(&self.data, n).parent.is_some() {
            self.delete_case_2(n);
        }
    }

    fn delete_case_2(&mut self, n: usize) {
        dbg!("delete case 2");
        let s = Node::get(&self.data, n).get_sibling(&self.data);
        if Node::get(&self.data, n).is_sibling_black(&self.data) {
            let p = Node::get(&self.data, n).parent.expect("D2 P");
            self.set_maybe_black(s);
            Node::get_mut(&mut self.data, p).color = Color::Red;
            self.rotate(Node::get(&self.data, n).side(&self.data), p);
        }
        self.delete_case_3(n);
    }

    fn delete_case_3(&mut self, n: usize) {
        dbg!("delete case 3");
        let s = Node::get(&self.data, n).get_sibling(&self.data).expect("D3 S");
        let p = Node::get(&self.data, n).parent.expect("D3 P");
        if Node::get(&self.data, n).is_parent_black(&self.data)
            && !Node::get(&self.data, s).is_red()
            && Node::get(&self.data, s).is_child_black(&self.data, Side::Left)
            && Node::get(&self.data, s).is_child_black(&self.data, Side::Right)
        {
            self.delete_case_1(p);
        } else {
            self.delete_case_4(p);
        }
    }

    fn delete_case_4(&mut self, n: usize) {
        dbg!("delete case 4");
        let node = Node::get(&self.data, n);
        let s = node.get_sibling(&self.data).expect("D4 S");
        let p = node.parent.expect("D4 P");

        if !node.is_parent_black(&self.data)
            && node.is_sibling_black(&self.data)
            && Node::get(&self.data, s).is_child_black(&self.data, Side::Left)
            && Node::get(&self.data, s).is_child_black(&self.data, Side::Right)
        {
            Node::get_mut(&mut self.data, s).color = Color::Red;
            Node::get_mut(&mut self.data, p).color = Color::Black;
        } else {
            self.delete_case_5(n)
        }
    }

    fn delete_case_5(&mut self, n: usize) {
        dbg!("delete case 5");
        let s = Node::get(&self.data, n).get_sibling(&self.data).expect("D5 S");
        if !Node::get(&self.data, s).is_red() {
            if Node::get(&self.data, n).is_child(&self.data, Side::Left)
                && Node::get(&self.data, s).is_child_black(&self.data, Side::Right)
                && !Node::get(&self.data, s).is_child_black(&self.data, Side::Left)
            {
                let scl = Node::get(&self.data, s).get_child(Side::Left);
                Node::get_mut(&mut self.data, s).color = Color::Red;
                self.set_maybe_black(scl);
                self.rotate(Side::Right, s);
            } else if Node::get(&self.data, n).is_child(&self.data, Side::Right)
                && Node::get(&self.data, s).is_child_black(&self.data, Side::Left)
                && !Node::get(&self.data, s).is_child_black(&self.data, Side::Right)
            {
                let scr = Node::get(&self.data, s).get_child(Side::Right);
                Node::get_mut(&mut self.data, s).color = Color::Red;
                self.set_maybe_black(scr);
                self.rotate(Side::Left, s);
            }
        }
        self.delete_case_6(n)
    }

    fn delete_case_6(&mut self, n: usize) {
        dbg!("delete case 6");
        let s = Node::get(&self.data, n).get_sibling(&self.data).expect("D6 S");
        let p = Node::get(&self.data, n).parent.expect("D6 P");
        let pc = Node::get(&self.data, p).color;
        Node::get_mut(&mut self.data, s).color = pc;
        Node::get_mut(&mut self.data, p).color = Color::Black;

        if Node::get(&self.data, n).is_child(&self.data, Side::Left) {
            let scr = Node::get(&self.data, s).get_child(Side::Right);
            self.set_maybe_black(scr);
            self.rotate(Side::Left, p);
        } else {
            let scl = Node::get(&self.data, s).get_child(Side::Left);
            self.set_maybe_black(scl);
            self.rotate(Side::Right, p);
        }
    }

    fn delete_replace(&mut self, n: usize) -> usize {
        let node = Node::get(&self.data, n);
        match (node.lchild, node.rchild) {
            (Some(lc), Some(rc)) => {
                let p = node.parent;
                let successor = Node::get(&self.data, rc).find_min(&self.data);
                self.delete_replace(successor);
                self.data.swap(successor, n);

                Node::get_mut(&mut self.data, n).lchild = Some(lc);
                Node::get_mut(&mut self.data, n).rchild = Some(rc);
                Node::get_mut(&mut self.data, n).parent = p;
                Node::get_mut(&mut self.data, n).ptr = n;
                return successor;
            }
            (None, Some(_rc)) => self.replace_node(n, Node::get(&self.data, n).rchild),
            (Some(_lc), None) => self.replace_node(n, Node::get(&self.data, n).lchild),
            (None, None) => self.replace_node(n, None),
        };
        n
    }

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

    pub fn to_pretty_string(&self) -> String {
        if let Some(root) = self.root {
            Node::get(&self.data, root).to_pretty_string(&self.data, 0)
        } else {
            String::from("(Empty tree)")
        }
    }

    fn fix_ins_color(&mut self, n: usize) {
        Node::get_mut(&mut self.data, n).color = Color::Red;
        if let Some(p) = Node::get(&self.data, n).parent {
            if !Node::get(&self.data, p).is_red() {
                // parent is black
                // do nothing
            } else if Node::get(&self.data, n).get_uncle(&self.data).is_some()
                && Node::get(
                    &self.data,
                    Node::get(&self.data, n).get_uncle(&self.data).unwrap(),
                )
                .is_red()
            {
                // uncle exists and is red
                let p = Node::get(&self.data, n).parent.unwrap();
                let u = Node::get(&self.data, n).get_uncle(&self.data).unwrap();
                Node::get_mut(&mut self.data, p).color = Color::Black;
                Node::get_mut(&mut self.data, u).color = Color::Black;
                self.fix_ins_color(Node::get(&self.data, p).parent.unwrap());
            } else {
                // uncle is black
                self.do_ins_hard_case(n);
            }
        } else {
        }
        Node::get_mut(&mut self.data, self.root.unwrap()).color = Color::Black;
    }

    fn do_ins_hard_case(&mut self, nn: usize) {
        let mut n = nn;
        let mut p = Node::get(&self.data, n).parent.unwrap();
        if Node::get(&self.data, p).is_child(&self.data, Side::Left)
            && Node::get(&self.data, n).is_child(&self.data, Side::Right)
        {
            self.rotate(Side::Left, n);
            n = Node::get(&self.data, n).get_child(Side::Left).unwrap();
        }

        p = Node::get(&self.data, n).parent.unwrap();
        if Node::get(&self.data, p).is_child(&self.data, Side::Right)
            && Node::get(&self.data, n).is_child(&self.data, Side::Left)
        {
            self.rotate(Side::Right, n);
            n = Node::get(&self.data, n).get_child(Side::Right).unwrap();
        }
        self.do_ins_hard_case2(n);
    }

    fn do_ins_hard_case2(&mut self, n: usize) {
        let p = Node::get(&self.data, n).parent.unwrap();
        let g = Node::get(&self.data, p).parent.unwrap();

        Node::get_mut(&mut self.data, p).color = Color::Black;
        Node::get_mut(&mut self.data, g).color = Color::Red;
        if Node::get(&self.data, p).is_child(&self.data, Side::Right) {
            self.rotate(Side::Left, p);
        } else if Node::get(&self.data, p).is_child(&self.data, Side::Right) {
            self.rotate(Side::Right, p);
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
    fn make_fake_tree_node_no_balance() -> Tree<i32> {
        let mut tree = Tree::new();
        for x in vec![50, 25, 75, 15, 35, 65, 85, 0, 20, 30, 40, 60, 70, 80, 100] {
            tree.insert(x);
        }

        tree
    }

    #[test]
    fn test_tree_print() {
        let tree = make_fake_tree_node_no_balance();
        assert_eq!(tree.to_string(), "([P:None C:Black V:50] ([P:Some(0) C:Red V:25] ([P:Some(1) C:Black V:15] ([P:Some(3) C:Red V:0] () ()) ([P:Some(3) C:Red V:20] () ())) ([P:Some(1) C:Black V:35] ([P:Some(4) C:Red V:30] () ()) ([P:Some(4) C:Red V:40] () ()))) ([P:Some(0) C:Red V:75] ([P:Some(2) C:Black V:65] ([P:Some(5) C:Red V:60] () ()) ([P:Some(5) C:Red V:70] () ())) ([P:Some(2) C:Black V:85] ([P:Some(6) C:Red V:80] () ()) ([P:Some(6) C:Red V:100] () ()))))");
        let tree_empty = Tree::<i32>::new();
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
        let mut tree = Tree::<i32>::new();
        for x in 0..10 {
            double_size_test(&tree, x as usize);
            tree.insert(x);
            double_size_test(&tree, (x + 1) as usize);
        }

        assert_eq!(tree.to_string(), "([P:None C:Black V:3] ([P:Some(3) C:Black V:1] ([P:Some(1) C:Black V:0] () ()) ([P:Some(1) C:Black V:2] () ())) ([P:Some(3) C:Black V:5] ([P:Some(5) C:Black V:4] () ()) ([P:Some(5) C:Red V:7] ([P:Some(7) C:Black V:6] () ()) ([P:Some(7) C:Black V:8] () ([P:Some(8) C:Red V:9] () ())))))");
    }

    fn double_size_test<T: PartialEq + PartialOrd + std::fmt::Debug>(
        tree: &Tree<T>,
        expect: usize,
    ) {
        assert_eq!(tree.get_size(), expect);
        assert_eq!(tree.get_size_recursive(), expect);
    }

    #[test]
    fn test_height() {
        let tree = make_fake_tree_node_no_balance();
        assert_eq!(tree.get_height(), 4);

        let mut tree2 = Tree::<i32>::new();
        for x in 0..10 {
            tree2.insert(x);
        }
        assert_eq!(tree2.get_height(), 5);

        let tree3 = Tree::<i32>::new();
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
        assert_eq!(tree.data.len(), 15);
        tree.insert(0);
        assert_eq!(tree.data.len(), 15);
        double_size_test(&tree, 15);

        tree.delete(50);
        double_size_test(&tree, 14);
        assert_eq!(tree.data.len(), 15);
        tree.insert(50);
        assert_eq!(tree.data.len(), 15);
        double_size_test(&tree, 15);
    }

    #[test]
    fn test_delete() {
        let mut tree = make_fake_tree_node_no_balance();
        double_size_test(&tree, 15);
        tree.delete(100);
        dbg!(tree.to_pretty_string());
        tree.delete(80);
        dbg!(tree.to_pretty_string());
        tree.delete(85);
        assert_eq!(tree.to_string(), "uuhhh");

    }
}
