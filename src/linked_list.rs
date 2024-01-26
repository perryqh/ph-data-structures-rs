struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// can call .into() on Node<T> to get Option<Rc<RefCell<Node<T>>>>
impl<T: Copy> From<Node<T>> for Option<Box<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);
        node.next = self.head.take();
        self.head = node.into();
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let mut node = self.head.take(); // self.head is now none
        match node {
            None => None,
            Some(mut node) => {
                self.head = node.next.take(); // node next is now take
                Some(node.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.pop_front(), None);
    }
}
