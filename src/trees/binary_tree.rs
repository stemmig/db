use std::fmt::{self, Formatter, Display};

pub struct BinaryTree{
    root: Option<Box<Node>>,
}

pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl BinaryTree {
    pub fn new() -> BinaryTree {
        BinaryTree { root: None}
    }

    pub fn append(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });

        if let Some(ref mut root) = self.root {
            BinaryTree::insert(root, new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert(node: &mut Box<Node>, new_node: Box<Node>) {
        if new_node.value < node.value {
            if let Some(ref mut left) = node.left {
                BinaryTree::insert(left, new_node);
            } else {
                node.left = Some(new_node);
            }
        } else {
            if let Some(ref mut right) = node.right {
                BinaryTree::insert(right, new_node);
            } else {
                node.right = Some(new_node);
            }
        }
    }

    // pub fn get(&mut self, idx: i32) -> i32 {
    //
    // }
    //
    // pub fn remove(&mut self, idx: i32) {
    //
    // }

    pub fn write_node(f: &mut Formatter<'_>, node: &Box<Node>) -> fmt::Result {
        // Write the Left Node
        if let Some(ref left ) = node.left {
            BinaryTree::write_node(f, left)?;
            // write!(f, ", ")?;
        }

        // Write the Value
        write!(f, "{}, ", node.value)?;

        // Write the Right Node
        if let Some(ref right) = node.right {
            BinaryTree::write_node(f, right)?;
            // write!(f, ", ")?;
        }

        Ok(())
    }
}

impl Display for BinaryTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        if let Some(ref root) = self.root {
            BinaryTree::write_node(f, root)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let mut tree = BinaryTree::new();
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_build() {
        let mut tree = BinaryTree::new();
        tree.root = Some( Box::new(Node {
            value: 42,
            left: None,
            right: None,
        }));
    }

    #[test]
    fn test_append() {
        let mut tree = BinaryTree::new();
        tree.append(15);
        tree.append(42);
        tree.append(9);
        tree.append(24);
    }

    #[test]
    fn test_display_tree() {
        let mut tree = BinaryTree::new();
        tree.append(15);
        tree.append(42);
        tree.append(9);
        tree.append(24);
        assert_eq!(tree.to_string(), "[9, 15, 24, 42, ]");
    }

    #[test]
    fn test_rebalance() {

    }

    #[test]
    fn test_remove() {

    }

    #[test]
    fn test_get() {

    }
}