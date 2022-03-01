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

/// A binary search tree.
pub struct BinarySearchTree<T, U>
    where T: PartialOrd + Eq
{
    nodes: Arena<Node<T, U>>,
    root: Option<usize>
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
    }

    /// Find a key, if it exists, and return a reference to the data stored
    /// there, otherwise return None.
    pub fn search(&self, key: &T) -> Option<&U> {
        panic!("notimplemented");
    }

    // Find the node with the given key and return a mutable reference to it. If
    // no such node exists, return None.
    fn search_internal(&self, key: &T) -> Option<&mut Node<T, U>> {
        panic!("oops");
    }

    /// Delete the given key from the tree and return ownership of its data.
    /// If no such key exists, return None.
    pub fn remove(&mut self, key: &T) -> Option<U> {
        panic!("no tengo");
    }

    pub fn iter_in_order(&self) -> in_order_iterator<T, U> {
        panic!("Iterate 'er? I 'ardly know 'er!");
    }
}

pub struct in_order_iterator<T, U>
    where T: PartialOrd + Eq
{
    current: Option<Node<T, U>>
}

impl<T, U> Iterator for in_order_iterator<T, U>
    where T: PartialOrd + Eq
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("Iterate 'er? I 'ardly know 'er!");
    }
}
