use crate::doubly_linked_list::{List, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Weak;

pub struct LRU<K: Copy + Eq + Hash, T: Copy> {
    pub list: List<T>,
    pub map: HashMap<K, Weak<RefCell<Node<T>>>>,
    pub capacity: usize,
}

impl<K: Copy + Eq + Hash, T: Copy> LRU<K, T> {
    pub fn new() -> Self {
        LRU::with_capacity(10)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        LRU {
            list: List::new(),
            map: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    pub fn get(&mut self, k: K) -> Option<T> {
        let ptr = self.map.get(&k);
        if ptr.is_none() {
            return None;
        }
        let ptr = ptr.unwrap();
        let ptr = ptr.upgrade();
        match ptr {
            None => None,
            Some(node) => {
                let value = node.borrow().value;
                self.list.move_node_to_back(node);
                Some(value)
            }
        }
    }

    pub fn put(&mut self, k: K, v: T) {
        let ptr = self.map.get_mut(&k);
        let ptr = if ptr.is_some() {
            ptr.unwrap().upgrade()
        } else {
            None
        };
        match ptr {
            None => {
                self.list.push_back(v);
                if let Some(tail) = self.list.get_weak_tail() {
                    self.map.insert(k, tail);
                }
                if self.list.len() > self.capacity {
                    let head = self.list.pop_front();
                }
            }
            Some(node) => {
                node.borrow_mut().value = v;
                self.list.move_node_to_back(node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_builds_lru() {
        let mut lru = LRU::with_capacity(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), Some(1));
        lru.put(3, 3);
        assert_eq!(lru.get(2), None);
        lru.put(4, 4);
        assert_eq!(lru.get(1), None);
        assert_eq!(lru.get(3), Some(3));
        assert_eq!(lru.get(4), Some(4));
    }

    #[test]
    fn works_builds_lru_str() {
        let mut lru = LRU::new();
        lru.put(1, "foo");
        lru.put(2, "bar");
        lru.put(3, "fizz");
        lru.put(4, "buzz");
        lru.put(5, "bazz");

        assert_eq!(lru.get(3), Some("fizz"));
        assert_eq!(lru.get(2), Some("bar"));

        let mut iter = lru.list.iter();
        assert_eq!(iter.next_back(), Some("bar"));
        assert_eq!(iter.next_back(), Some("fizz"));
        assert_eq!(iter.next_back(), Some("bazz"));
        assert_eq!(iter.next_back(), Some("buzz"));
        assert_eq!(iter.next_back(), Some("foo"));
        assert_eq!(iter.next_back(), None);
    }
}
