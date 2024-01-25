use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T: Copy> {
    pub value: T,
    pub next: Option<NodePtr<T>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>, // weak reference to avoid reference cycles
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

// can call .into() on Node<T> to get Option<Rc<RefCell<Node<T>>>>
impl<T: Copy> From<Node<T>> for Option<Rc<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

// Rc<RefCell<_>> allows us to have multiple mutable references to the same data
type NodePtr<T> = Rc<RefCell<Node<T>>>;

pub struct List<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
    count: usize,
}

impl<T: Copy> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn push_front(&mut self, value: T) {
        // mutable so we can assign previous node
        let mut node = Node::new(value);

        // take() moves the value out of the option, leaving None in its place
        // this is important because we can't have a mutable reference to self.tail AND
        // set self.tail = node.into() at the same time
        match &self.head.take() {
            None => {
                // empty list
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_head) => {
                node.next = Some(current_head.clone());
                self.head = node.into();
                if let Some(h) = &self.head {
                    current_head.borrow_mut().prev = Some(Rc::downgrade(h));
                }
            }
        }
        self.count += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();
                match next {
                    None => {
                        self.tail.take(); // also set tail to None
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }
                self.count -= 1;
                Some(head.value)
            }
        }
    }

    pub fn push_back(&mut self, value: T) {
        // mutable so we can assign previous node
        let mut node = Node::new(value);

        // take() moves the value out of the option, leaving None in its place
        // this is important because we can't have a mutable reference to self.tail AND
        // set self.tail = node.into() at the same time
        match &self.tail.take() {
            None => {
                // empty list
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(current_tail)); // weak reference
                self.tail = node.into();
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
        self.count += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &self.tail.take() {
            None => None,
            Some(tail) => {
                let mut tail = tail.borrow_mut();
                let prev = tail.prev.take();
                match prev {
                    None => {
                        self.head.take();
                    }
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                };
                self.count -= 1;
                Some(tail.value)
            }
        }
    }

    pub fn remove_node(&mut self, node: &mut NodePtr<T>) {
        let (prev, next) = {
            let mut node = node.borrow_mut();
            let prev = match node.prev.take() {
                None => None,
                Some(prev) => prev.upgrade(),
            };
            let next = node.next.take();
            (prev, next)
        };
        match (prev, next) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head.replace(next);
            }
            (Some(prev), None) => {
                prev.borrow_mut().next = None;
                self.tail.replace(prev);
            }
            (Some(prev), Some(next)) => {
                next.borrow_mut().prev = Some(Rc::downgrade(&prev));
                prev.borrow_mut().next.replace(next);
            }
        }
    }

    pub fn move_node_to_back(&mut self, mut node: NodePtr<T>) {
        self.remove_node(&mut node);
        self.push_node_back(node);
    }

    pub fn push_node_back(&mut self, node: NodePtr<T>) {
        match self.tail.take() {
            None => {
                self.head.replace(node);
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.borrow_mut().prev.replace(Rc::downgrade(&current_tail));
                self.tail.replace(node);
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
    }

    pub fn get_weak_tail(&self) -> Option<Weak<RefCell<Node<T>>>> {
        match &self.tail {
            None => None,
            Some(tail) => Some(Rc::downgrade(tail)),
        }
    }

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            current: self.head.clone(),
            current_back: self.tail.clone(),
        }
    }
}

// Help the compiler understand that we want to drop the entire list
impl<T: Copy> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

pub struct ListIterator<T: Copy> {
    current: Option<NodePtr<T>>,
    current_back: Option<NodePtr<T>>,
}

impl<T: Copy> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &self.current_back.take() {
            None => None,
            Some(current_back) => {
                let current_back = current_back.borrow();
                match &current_back.prev {
                    None => Some(current_back.value),
                    Some(prev) => {
                        self.current_back = prev.upgrade();
                        Some(current_back.value)
                    }
                }
            }
        }
    }
}

impl<T: Copy> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current.take() {
            None => None,
            Some(current) => {
                let current = current.borrow();
                let next = current.next.clone();
                self.current = next;
                Some(current.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_list_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_build_list_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_front_and_back() {
        let mut list = List::new();
        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        list.push_back(4);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn works_builds_list_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        list.push_back(4);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn works_builds_list_iter_rev() {
        let mut list = List::new();
        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        list.push_back(4);

        let mut iter = list.iter().rev();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}
