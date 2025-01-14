use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}


#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}


impl<T: Ord> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}


impl<T: Ord> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    
    fn insert(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut root) => root.insert(value),
        }
    }

    // 在二叉搜索树中搜索值
    fn search(&self, value: T) -> bool {
        match self.root {
            None => false,
            Some(ref root) => root.search(value),
        }
    }
}


impl<T: Ord> TreeNode<T> {
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let None = self.left {
                    self.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.left.as_mut().unwrap().insert(value);
                }
            },
            Ordering::Greater => {
                if let None = self.right {
                    self.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.right.as_mut().unwrap().insert(value);
                }
            },
            Ordering::Equal => {}
        }
    }

    
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let None = self.left {
                    return false;
                } else {
                    return self.left.as_ref().unwrap().search(value);
                }
            },
            Ordering::Greater => {
                if let None = self.right {
                    return false;
                } else {
                    return self.right.as_ref().unwrap().search(value);
                }
            },
            Ordering::Equal => true,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();
        assert_eq!(bst.search(1), false);
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        bst.insert(1);
        assert_eq!(bst.search(1), true);
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}