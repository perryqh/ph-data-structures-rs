#[derive(Debug)]

struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            },
            Some(node) => {
                // can't self.insert_recursive because self is already mutable until the end of the match
                Tree::insert_recursive(node, value);
                // self.insert_iterative(value);
            }
        }
    }

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
            return;
        }
        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);

        while let Some(node) = q.pop() {
            if value > node.value {
                // 1) needed so the borrow checker knows that node.right is mutable
                match node.right {
                    ref mut right @ None => {
                        *right = Node::new(value).into();
                    },
                    Some(ref mut right) => {
                        q.push(right);
                    }
                }
            } else if value < node.value {
                // 2) needed so the borrow checker knows that node.right is mutable
                let left = &mut node.left;
                match left {
                    None => {
                        *left = Node::new(value).into();
                    },
                    Some(left) => {
                        q.push(left);
                    }
                }
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                },
                Some(right) => {
                    Self::insert_recursive(right, value);
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                },
                Some(left) => {
                    Self::insert_recursive(left, value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);

        assert_eq!(tree.root.is_some(), true);
        assert_eq!(tree.root.as_ref().unwrap().value, 8);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().value, 3);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().value, 1);
    }
}
