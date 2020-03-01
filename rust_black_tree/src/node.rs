use std::cmp::max;
use std::ops::Not;

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
pub struct Node<T> {
    pub value: T,
    pub ptr: usize,
    pub parent: Option<usize>,
    pub lchild: Option<usize>,
    pub rchild: Option<usize>,
    pub color: Color,
}

impl<T> Node<T>
where
    T: std::fmt::Debug,
{
    pub fn new(val: T, selfptr: usize) -> Self {
        Node::<T> {
            value: val,
            ptr: selfptr,
            parent: None,
            lchild: None,
            rchild: None,
            color: Color::Black,
        }
    }

    pub fn is_red(&self) -> bool {
        match self.color {
            Color::Red => true,
            Color::Black => false,
        }
    }

    pub fn get_child(&self, side: Side) -> Option<usize> {
        match side {
            Side::Left => self.lchild,
            Side::Right => self.rchild,
        }
    }

    pub fn set_child(data: &mut Vec<Node<T>>, selfptr: usize, child: usize, side: Side) {
        match side {
            Side::Left => data[selfptr].lchild = Some(child),
            Side::Right => data[selfptr].rchild = Some(child),
        };
        data[child].parent = Some(selfptr);
    }

    pub fn is_child(&self, data: &Vec<Node<T>>, side: Side) -> bool {
        if let Some(p) = self.parent {
            let parent = Node::get(data, p);
            parent.get_child(side).is_some()
                && parent.get_child(side).unwrap() == self.ptr
        } else {
            false
        }
    }

    // Nil nodes are black children too
    pub fn is_child_black(&self, data: &Vec<Node<T>>, side: Side) -> bool{
        let child = self.get_child(side);
        if child.is_some() && Node::get(data, child.unwrap()).is_red() {
            false
        } else {
            true
        }
    }

    // this will panic of called on root node
    pub fn is_parent_black(&self, data: &Vec<Node<T>>) -> bool {
        let p = self.parent.unwrap();
        !Node::get(data, p).is_red()
    }

    // Nil nodes are black children too
    pub fn is_sibling_black(&self, data: &Vec<Node<T>>) -> bool {
        let sib = self.get_sibling(data);
        if sib.is_some() && Node::get(data, sib.unwrap()).is_red() {
            false
        } else {
            true
        }
    }

    pub fn side(&self, data: &Vec<Node<T>>) -> Side {
        if self.is_child(data, Side::Left) {
            Side::Left
        } else {
            Side::Right
        }
    }

    pub fn get_sibling(&self, data: &Vec<Node<T>>) -> Option<usize> {
        if let Some(p) = self.parent {
            let parent = Node::get(data, p);
            if self.is_child(data, Side::Left) {
                parent.rchild
            } else if self.is_child(data, Side::Right) {
                parent.lchild
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_uncle(&self, data: &Vec<Node<T>>) -> Option<usize> {
        self.parent
            .and_then(|p| Some(Node::get(data, p)))
            .and_then(|p| p.get_sibling(data))
    }

    pub fn to_string(&self, data: &Vec<Node<T>>) -> String {
        let mut m_str = format!(
            "([P:{:?} C:{:?} V:{:?}]",
            self.parent, self.color, self.value
        );
        m_str = m_str
            + " "
            + &(if let Some(child) = self.get_child(Side::Left) {
                data[child].to_string(data)
            } else {
                String::from("()")
            });
        m_str = m_str
            + " "
            + &(if let Some(child) = self.get_child(Side::Right) {
                data[child].to_string(data)
            } else {
                String::from("()")
            });
        m_str + ")"
    }

    pub fn get(data: &Vec<Node<T>>, ptr: usize) -> &Node<T> {
        &data[ptr]
    }

    pub fn get_mut(data: &mut Vec<Node<T>>, ptr: usize) -> &mut Node<T> {
        &mut data[ptr]
    }

    pub fn get_height(&self, data: &Vec<Node<T>>) -> usize {
        let f = |c| Some(1 + Node::get(data, c).get_height(data));
        max(
            self.lchild.and_then(f).unwrap_or(1),
            self.rchild.and_then(f).unwrap_or(1),
        )
    }

    pub fn get_size(&self, data: &Vec<Node<T>>) -> usize {
        let f = |c| Some(Node::get(data, c).get_size(data));

        1 + self.lchild.and_then(f).unwrap_or(0) + self.rchild.and_then(f).unwrap_or(0)
    }

    pub fn find_min(&self, data: &Vec<Node<T>>) -> usize {
        if let Some(l) = self.lchild {
            Node::get(data, l).find_min(data)
        } else {
            self.ptr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn make_fake_tree_node() -> Vec<Node<i32>> {
        let mut v = vec![
            Node::new(5, 0),
            Node::new(4, 1),
            Node::new(6, 2),
            Node::new(0, 3),
            Node::new(1, 4),
            Node::new(7, 5),
        ];
        let ptrs: Vec<usize> = v.iter().map(|v| v.ptr).collect();
        Node::set_child(&mut v, ptrs[0], ptrs[4], Side::Left);
        Node::set_child(&mut v, ptrs[0], ptrs[2], Side::Right);
        Node::set_child(&mut v, ptrs[4], ptrs[3], Side::Left);
        Node::set_child(&mut v, ptrs[4], ptrs[1], Side::Right);
        Node::set_child(&mut v, ptrs[2], ptrs[5], Side::Right);
        v
    }

    #[test]
    fn print_tree_test() {
        let data = make_fake_tree_node();
        let root = &data[0];
        assert_eq!(root.to_string(&data), "([P:None C:Black V:5] ([P:Some(0) C:Black V:1] ([P:Some(4) C:Black V:0] () ()) ([P:Some(4) C:Black V:4] () ())) ([P:Some(0) C:Black V:6] () ([P:Some(2) C:Black V:7] () ())))");
    }

    #[test]
    fn get_child_test() {
        let data = make_fake_tree_node();
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
        let data = make_fake_tree_node();
        assert_eq!(data[2].get_sibling(&data), Some(4));
        assert_eq!(data[4].get_sibling(&data), Some(2));
        assert_eq!(data[0].get_sibling(&data), None);
        assert_eq!(data[3].get_sibling(&data), Some(1));
        assert_eq!(data[1].get_sibling(&data), Some(3));
        assert_eq!(data[5].get_sibling(&data), None);
    }

    #[test]
    fn get_uncle_test() {
        let data = make_fake_tree_node();
        assert_eq!(data[0].get_uncle(&data), None);
        assert_eq!(data[1].get_uncle(&data), Some(2));
        assert_eq!(data[2].get_uncle(&data), None);
        assert_eq!(data[3].get_uncle(&data), Some(2));
        assert_eq!(data[4].get_uncle(&data), None);
        assert_eq!(data[5].get_uncle(&data), Some(4));
    }

    #[test]
    fn get_size() {
        let data = make_fake_tree_node();
        assert_eq!(data[0].get_size(&data), 6);
        assert_eq!(data[4].get_size(&data), 3);
        assert_eq!(data[5].get_size(&data), 1);
    }

    #[test]
    fn get_height() {
        let data = make_fake_tree_node();
        assert_eq!(data[0].get_height(&data), 3);
        assert_eq!(data[4].get_height(&data), 2);
        assert_eq!(data[5].get_height(&data), 1);
    }

    #[test]
    fn find_min() {
        let data = make_fake_tree_node();
        assert_eq!(Node::get(&data, data[0].find_min(&data)).value, 0);
        assert_eq!(Node::get(&data, data[2].find_min(&data)).value, 6);
    }
}
