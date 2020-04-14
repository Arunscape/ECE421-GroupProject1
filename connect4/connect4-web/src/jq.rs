use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub fn mpsc<T> () -> (JSender<T>, JReceiver<T>) {
    let data = Rc::from(RefCell::from(VecDeque::new()));
    (JSender { data: data.clone() }, JReceiver { data })
}

#[derive(Debug)]
pub struct JReceiver<T> {
    data: Rc<RefCell<VecDeque<T>>>
}
#[derive(Clone, Debug)]
pub struct JSender<T> {
    data: Rc<RefCell<VecDeque<T>>>
}

impl <T> JReceiver<T> {
    pub fn recv(&self) -> Option<T> {
        self.data.borrow_mut().pop_front()
    }
}

impl <T> JSender<T> {
    pub fn send(&self, t: T) {
        self.data.borrow_mut().push_back(t)
    }
}
