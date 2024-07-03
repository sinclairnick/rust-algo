use std::cmp::Ordering;

type HeapNode<T> = Option<Box<TreeNode<T>>>;

pub struct TreeNode<T> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
}

impl<T: Ord + Copy> TreeNode<T> {
    pub fn from_value(value: T) -> Self {
        TreeNode {
            value,
            left: Option::default(),
            right: Option::default(),
        }
    }

    pub fn insert(&mut self, item: T) -> () {
        match item.cmp(&self.value) {
            Ordering::Equal => {
                // Do nothing
                return;
            }
            Ordering::Greater => {
                if let Some(right) = &mut self.right {
                    right.insert(item);
                } else {
                    self.right = Some(Box::from(TreeNode {
                        value: item,
                        left: None,
                        right: None,
                    }))
                }
            }
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(item);
                } else {
                    self.left = Some(Box::from(TreeNode {
                        value: item,
                        left: None,
                        right: None,
                    }));
                }
            }
        };
    }

    pub fn delete(&mut self, value: T) -> HeapNode<T> {
        // Traverse right tree
        if value < self.value {
            self.left = match &mut self.left {
                Some(left) => left.delete(value),
                None => None,
            };
        }

        // Traverse left tree
        if value > self.value {
            self.right = match &mut self.right {
                Some(right) => right.delete(value),
                None => None,
            };
        }

        // Current node is match, handle child nodes
        if self.left.is_none() {
            return self.right.take();
        }

        if self.right.is_none() {
            return self.left.take();
        }

        // TODO: Not sure how to properly do in-order reassignment

        return None;
    }

    pub fn lookup(&self, value: T) -> Option<T> {
        if self.value == value {
            return Some(self.value);
        }

        if value < self.value {
            return match &self.left {
                Some(left) => left.lookup(value),
                None => None,
            };
        }

        if value > self.value {
            return match &self.right {
                Some(right) => right.lookup(value),
                None => None,
            };
        }

        return None;
    }

    fn min<'p>(&self) -> &Self {
        match self.left {
            Some(ref left) => left.min(),
            None => &self,
        }
    }
}

#[cfg(test)]
mod test {
    use super::TreeNode;

    #[test]
    fn test_create() {
        let tree = TreeNode::from_value(1);

        assert_eq!(tree.value, 1);
        assert!(tree.left.is_none());
        assert!(tree.right.is_none());
    }

    #[test]
    fn test_inserts_node() {
        let mut tree = TreeNode::from_value(1);
        tree.insert(2);

        assert!(tree.right.is_some());
        assert_eq!(tree.right.unwrap().value, 2);
    }

    #[test]
    fn test_inserts_node_nested() {
        let mut tree = TreeNode::from_value(1);
        tree.insert(3);
        tree.insert(2);

        // 3 -> root.right
        assert!(tree.right.is_some());
        let root_right = tree.right.unwrap();
        assert_eq!(root_right.value, 3);

        // 2 -> root.right.left
        assert!(root_right.left.is_some());
        let root_right_left = root_right.left.unwrap();
        assert_eq!(root_right_left.value, 2);
    }

    #[test]
    fn test_deletes_node() {
        let mut tree = TreeNode::from_value(1);

        assert!(tree.delete(1).is_none());
    }

    #[test]
    fn test_deletes_node_nested() {
        let mut tree = TreeNode::from_value(1);
        tree.insert(2);

        assert_eq!(tree.value, 1);
        assert!(tree.right.is_some());

        tree.delete(2);
        assert!(tree.right.is_none());
    }

    #[test]
    fn test_lookup_node_present() {
        let mut tree = TreeNode::from_value(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(tree.lookup(1), Some(1));
        assert_eq!(tree.lookup(2), Some(2));
        assert_eq!(tree.lookup(3), Some(3));
    }

    #[test]
    fn test_lookup_node_absent() {
        let mut tree = TreeNode::from_value(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(tree.lookup(1), Some(1));
        assert_eq!(tree.lookup(2), Some(2));
        assert_eq!(tree.lookup(3), Some(3));
        assert_eq!(tree.lookup(4), None);
    }
}
