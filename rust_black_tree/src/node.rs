use std::cmp::max;
use std::ops::Not;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

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

#[derive(Debug)]
pub struct DepthNode<T> {
    pub value: T,
    pub ptr: usize,
    pub parent: Option<usize>,
    pub lchild: Option<usize>,
    pub rchild: Option<usize>,
    pub height: usize,
}

pub trait Node<T> {
    // Base methods
    fn get(&self, i: usize) -> &Self;
    fn get_mut(&self, i: usize) -> &mut Self;
    fn location(&self) -> usize;
    fn get_parent(&self) -> Option<usize>;
    fn set_parent(&mut self, p: Option<usize>);
    fn get_child(&self, side: Side) -> Option<usize>;
    fn set_child(&mut self, child: usize, side: Side);
    fn set_child_opt(&mut self, child: Option<usize>, side: Side);
    fn to_self_string(&self) -> String;
    fn is(&self, val: &T) -> bool;
    fn greater(&self, val: &T) -> bool;
    fn lesser(&self, val: &T) -> bool;

    fn to_string(&self) -> String {
        let mut m_str = format!("({}", self.to_self_string());
        m_str = m_str
            + " "
            + &(if let Some(child) = self.get_child(Side::Left) {
                self.get(child).to_string()
            } else {
                String::from("()")
            });
        m_str = m_str
            + " "
            + &(if let Some(child) = self.get_child(Side::Right) {
                self.get(child).to_string()
            } else {
                String::from("()")
            });
        m_str + ")"
    }

    fn to_pretty_string(&self, indent: usize) -> String {
        let i = indent * 2;
        let mut m_str = format!("({}", self.to_self_string());
        m_str = m_str
            + "\n"
            + &" ".repeat(i)
            + &(if let Some(child) = self.get_child(Side::Left) {
                self.get(child).to_pretty_string(indent + 1)
            } else {
                String::from("()")
            });
        m_str = m_str
            + "\n"
            + &" ".repeat(i)
            + &(if let Some(child) = self.get_child(Side::Right) {
                self.get(child).to_pretty_string(indent + 1)
            } else {
                String::from("()")
            });
        m_str + ")"
    }

    fn get_height(&self) -> usize {
        let f = |c| Some(1 + self.get(c).get_height());
        max(
            self.get_child(Side::Left).and_then(f).unwrap_or(1),
            self.get_child(Side::Right).and_then(f).unwrap_or(1),
        )
    }

    fn get_size(&self) -> usize {
        let f = |c| Some(self.get(c).get_size());

        1 + self.get_child(Side::Left).and_then(f).unwrap_or(0)
          + self.get_child(Side::Right).and_then(f).unwrap_or(0)
    }

    fn find_min(&self) -> usize {
        if let Some(l) = self.get_child(Side::Left) {
            self.get(l).find_min()
        } else {
            self.location()
        }
    }

    fn side(&self) -> Side {
        if self.is_child(Side::Left) {
            Side::Left
        } else {
            Side::Right
        }
    }

    fn get_sibling(&self) -> Option<usize> {
        if let Some(p) = self.get_parent() {
            let parent = self.get(p);
            if self.is_child(Side::Left) {
                parent.get_child(Side::Right)
            } else if self.is_child(Side::Right) {
                parent.get_child(Side::Left)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_uncle(&self) -> Option<usize> {
        self.get_parent()
            .and_then(|p| Some(self.get(p)))
            .and_then(|p| p.get_sibling())
    }

    fn is_child(&self, side: Side) -> bool {
        if let Some(p) = self.get_parent() {
            let parent = self.get(p);
            parent.get_child(side).is_some()
                && parent.get_child(side).unwrap() == self.location()
        } else {
            false
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn attach_child(data: &mut Vec<ColorNode<i32>>, p: usize, c: usize, side: Side) {
        let par = &mut data[p];
        par.set_child(c, side)
    }
    /*
    ========
    = Data =
    ========
    [0, 1, 2, 3, 4, 5] // ptrs
    [5, 4, 6, 0, 1, 7] // vals
    ([P:None C:Black V:5]
         ([P:Some(0) C:Black V:1]
              ([P:Some(4) C:Black V:0])
              ([P:Some(4) C:Black V:4]))
         ([P:Some(0) C:Black V:6]
              ([P:Some(2) C:Black V:7])))
    */
    fn make_fake_tree_node() -> Rc<RefCell<Vec<ColorNode<i32>>>> {
        let rc = Rc::new(RefCell::new(Vec::new()));
        let mut v = vec![
            ColorNode::new(5, 0, rc.clone()),
            ColorNode::new(4, 1, rc.clone()),
            ColorNode::new(6, 2, rc.clone()),
            ColorNode::new(0, 3, rc.clone()),
            ColorNode::new(1, 4, rc.clone()),
            ColorNode::new(7, 5, rc.clone()),
        ];
        (*rc).replace(v);
        let ptrs: Vec<usize> = (*rc).borrow_mut().iter().map(|v| v.ptr).collect();
        attach_child(&mut (*rc).borrow_mut(), ptrs[0], ptrs[4], Side::Left);
        attach_child(&mut (*rc).borrow_mut(), ptrs[0], ptrs[2], Side::Right);
        attach_child(&mut (*rc).borrow_mut(), ptrs[4], ptrs[3], Side::Left);
        attach_child(&mut (*rc).borrow_mut(), ptrs[4], ptrs[1], Side::Right);
        attach_child(&mut (*rc).borrow_mut(), ptrs[2], ptrs[5], Side::Right);
        rc
    }

    #[test]
    fn print_tree_test() {
        let rc = make_fake_tree_node();
        let data = rc.borrow_mut();
        let root = &data[0];
        assert_eq!(root.to_string(), "([P:None C:Black V:5] ([P:Some(0) C:Black V:1] ([P:Some(4) C:Black V:0] () ()) ([P:Some(4) C:Black V:4] () ())) ([P:Some(0) C:Black V:6] () ([P:Some(2) C:Black V:7] () ())))");
    }

    #[test]
    fn get_child_test() {
        let rc = make_fake_tree_node();
        let data = rc.borrow_mut();
        let root = &data[0];

        assert_eq!(root.get_child(Side::Left), Some(4));
        assert_eq!(root.get_child(Side::Right), Some(2));
        assert_eq!(data[2].get_child(Side::Right), Some(5));
        assert_eq!(
            data[root.get_child(Side::Left).unwrap()].get_child(Side::Right),
            Some(1)
        );
        assert_eq!(
            data[root.get_child(Side::Left).unwrap()].get_child(Side::Left),
            Some(3)
        );

        assert_eq!(root.value, 5);
        assert_eq!(data[root.get_child(Side::Left).unwrap()].value, 1);
    }

    #[test]
    fn get_sibling_test() {
        let rc = make_fake_tree_node();
        let data = rc.borrow_mut();
        assert_eq!(data[2].get_sibling(), Some(4));
        assert_eq!(data[4].get_sibling(), Some(2));
        assert_eq!(data[0].get_sibling(), None);
        assert_eq!(data[3].get_sibling(), Some(1));
        assert_eq!(data[1].get_sibling(), Some(3));
        assert_eq!(data[5].get_sibling(), None);
    }

    #[test]
    fn get_uncle_test() {
        let rc = make_fake_tree_node();
        let data = rc.borrow_mut();
        assert_eq!(data[0].get_uncle(), None);
        assert_eq!(data[1].get_uncle(), Some(2));
        assert_eq!(data[2].get_uncle(), None);
        assert_eq!(data[3].get_uncle(), Some(2));
        assert_eq!(data[4].get_uncle(), None);
        assert_eq!(data[5].get_uncle(), Some(4));
    }

    #[test]
    fn get_size() {
        let rc = make_fake_tree_node();
        let data = rc.borrow_mut();
        assert_eq!(data[0].get_size(), 6);
        assert_eq!(data[4].get_size(), 3);
        assert_eq!(data[5].get_size(), 1);
    }

    #[test]
    fn get_height() {
        let rc = make_fake_tree_node();
        let data = rc.borrow_mut();
        assert_eq!(data[0].get_height(), 3);
        assert_eq!(data[4].get_height(), 2);
        assert_eq!(data[5].get_height(), 1);
    }

    #[test]
    fn find_min() {
        let rc = make_fake_tree_node();
        let data = rc.borrow_mut();
        assert_eq!(data[(data[0].find_min())].value, 0);
        assert_eq!(data[(data[2].find_min())].value, 6);
    }
}
