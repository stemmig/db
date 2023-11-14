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

    pub fn append(&mut self, num: u8) {

    }
    //
    // pub fn get(&mut self, idx:u8) -> u8 {
    //     8
    // }
    //
    // pub fn remove(&mut self, idx:u8) {
    //
    // }
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