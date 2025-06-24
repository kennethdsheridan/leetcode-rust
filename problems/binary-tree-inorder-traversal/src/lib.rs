use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // setup the DS
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = root;

        // main loop continues until we've processed all nodes
        // with these two conditions:
        // 1. current.is_some() - we have a node to process
        // 2. !stack.is_empty() - we have unfinished work (nodes waiting to be processed)
        while current.is_some() || !stack.is_empty() {
            // go left as far as possible simulating a recursive call to left child in recursive
            // version. We keep going left until we hit a leaf (None)
            while let Some(node) = current {
                // push to stack
                stack.push(node.clone());
                // go left
                current = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);

                // go right
                current = node.borrow().right.clone();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases here
        let mut root = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);

        node2.left = Some(Rc::new(RefCell::new(node3)));
        root.right = Some(Rc::new(RefCell::new(node2)));

        let result = Solution::inorder_traversal(Some(Rc::new(RefCell::new(root))));
        assert_eq!(result, vec![1, 3, 2]);
    }
}
