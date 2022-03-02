use crate::arena::Arena;

#[cfg(test)]
mod binary_search_tree_tests {
    use super::*;

    // Insert some elements into a tree and retrieve them.
    #[test]
    fn insert_and_search() {
        let mut tree = BinarySearchTree::new();

        for i in 0..100 {
            tree.insert(i, 2 * i);
            // Find each new element as we insert it.
            assert_eq!(*(tree.search(&i).unwrap()), i * 2);
        }
        
        // Search for everything again now that we have all of the elements in the tree.
        for i in 0..100 {
            assert_eq!(*(tree.search(&i).unwrap()), i * 2);
        }
    }

    // Search an empty tree, then insert some values and search for something
    // that doesn't exist. In both cases, None should be returned.
    #[test]
    fn search_nonexistent() {
        let mut tree = BinarySearchTree::new();
        
        for i in 2..102 {
            assert!(tree.search(&1).is_none());
            tree.insert(i, i);
        }
    }

    // Insert some elements and remove them.
    // Verify that ownership of the data is returned.
    // Verify that they have been deleted by searching for them.
    #[test]
    fn insert_and_remove() {
        let mut tree = BinarySearchTree::new();

        for i in 0..100 {
            tree.insert(i, 2 * i);
        }
        for i in 0..100 {
            assert_eq!(tree.remove(&i).unwrap(), i * 2);
            assert!(tree.search(&i).is_none());
        }
    }

    // Attempt to insert duplicate keys. Should panic.
    #[test]
    #[should_panic(expected = "Insertion of duplicate keys is not supported.")]
    fn insert_duplicates() {
        let mut tree = BinarySearchTree::new();
        tree.insert(1, 1);
        tree.insert(1, 2);
    }

    // Create a tree, insert some elements, get an in-order iterator, and use it
    // to traverse the tree. Verify that iteration occurs in ascending order.
    #[test]
    fn in_order_traversal() {
        let mut tree = BinarySearchTree::new();

        for i in 0..100 {
            tree.insert(i, i);
        }

        let mut iter = tree.iter_in_order();
        let mut j = 0;
        // Make sure the inerted items are all there in the correct order.
        for i in iter {
            assert_eq!(i, j);
            j += 1;
        }
        // Make sure we got through all 100 inserted elements.
        assert_eq!(j, 100);
    }
}


struct Node<T, U> {
    key: T,
    data: U,
    // Indices of the left and right child in the "arena" holding this tree.
    left: Option<usize>,
    right: Option<usize>
}

/// An unbalanced binary search tree.
pub struct BinarySearchTree<T, U>
    where T: PartialOrd + Eq
{
    nodes: Arena<Node<T, U>>,
    root: Option<usize>
}

impl<T, U> Node<T,U>
    where T: PartialOrd + Eq
{
    fn new(key: T, data: U)
        -> Node<T, U>
    {
        Node {
            key,
            data,
            left: None,
            right: None
        }
    }
}

impl<T, U> BinarySearchTree<T, U>
    where T: PartialOrd + Eq
{
    /// Create an empty BinarySearchTree.
    pub fn new() -> BinarySearchTree<T, U> {
        BinarySearchTree {
            nodes: Arena::new(),
            root: None
        }
    }

    /// Insert an item into the binary search tree.
    /// # Panics
    /// Panics if the key (item) is already present in the tree.
    pub fn insert(&mut self, key: T, data: U) {
        let new_node = Node::new(key, data);
        
        match self.root {
            Some(root) => self.insert_internal(root, new_node),
            None => self.root = Some(self.nodes.insert(new_node))
        }
    }

    // Recursively search for the proper location for the new node and insert it.
    // The caller must ensure that the "current" parameter indexes to an existing Node.
    fn insert_internal(&mut self, current: usize, new_node: Node<T, U>) {
        let next_is_left;
        let current_node = match &self.nodes[current] {
            Some(node) => {
                node
            },
            None => {
                panic!("An invalid node index was passed to insert_internal().\
                    This is probably a bug in BinarySearchTree's implementation.");
            }
        };

        if  new_node.key < current_node.key {
            next_is_left = true;
        }
        else if new_node.key > current_node.key {
            next_is_left = false;
        }
        else {
            panic!("Insertion of duplicate keys is not supported.");
        }

        if next_is_left {
            match current_node.left {
                Some(next_index) => {
                    self.insert_internal(next_index, new_node);
                },
                None => {
                    let left = Some(self.nodes.insert(new_node));
                    // We know we can unwrap because we're inside a "match current_node..." block.
                    let current_node = self.nodes.borrow_mutable(current).unwrap();
                    current_node.left = left;
                }
            }
        }
        else {
            match current_node.right {
                Some(next_index) => {
                    self.insert_internal(next_index, new_node);
                },
                None => {
                    let right = Some(self.nodes.insert(new_node));
                    // We know we can unwrap because we're inside a "match current_node..." block.
                    let current_node = self.nodes.borrow_mutable(current).unwrap();
                    current_node.right = right;
                }
            }
        }
    }

    /// Find a key, if it exists, and return a reference to the data stored
    /// there, otherwise return None.
    pub fn search(&self, key: &T) -> Option<&U> {
        match self.root {
            Some(root) => match self.search_internal(root, key) {
                // We can unwrap here because search_internal is expected to
                // either return a valid index or None.
                Some(i) => Some(&(self.nodes[i].as_ref().unwrap().data)),
                None => None
            }
            None => None
        }
    }

    // Find the node with the given key and return its index. If no such node
    // exists, return None.
    fn search_internal(&self, current: usize, key: &T)
        -> Option<usize>
    {
        let current_node = match &self.nodes[current] {
            Some(node) => node,
            None => {
                panic!("An invalid node index was passed to search_internal().\
                This is probably a bug in BinarySearchTree's implementation.");
            }
        };

        if *key < current_node.key {
            match current_node.left {
                // Search the left subtree.
                Some(left_index) => self.search_internal(left_index, key),
                // The given key does not exist in the tree.
                None => None
            }
        }
        else if *key > current_node.key {
            match current_node.right {
                // Search the right subtree.
                Some(right_index) => self.search_internal(right_index, key),
                // The given key does not exist in the tree.
                None => None
            }
        }
        else {
            // We've found the key.
            Some(current)
        }
    }

    /// Delete the given key from the tree and return ownership of its data.
    /// If no such key exists, return None.
    pub fn remove(&mut self, key: &T) -> Option<U> {
        panic!("no tengo");
    }

    pub fn iter_in_order(&self) -> InOrderIterator<T, U> {
        panic!("Iterate 'er? I 'ardly know 'er!");
    }
}

pub struct InOrderIterator<T, U>
    where T: PartialOrd + Eq
{
    current: Option<Node<T, U>>
}

impl<T, U> Iterator for InOrderIterator<T, U>
    where T: PartialOrd + Eq
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("Iterate 'er? I 'ardly know 'er!");
    }
}
