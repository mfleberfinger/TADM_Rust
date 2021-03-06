use crate::arena::Arena;

#[cfg(test)]
mod binary_search_tree_tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Calculate and return a hash of the given value.
    fn calculate_hash<T: Hash>(input: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }

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

        // Repeat the test but insert keys in descending order.
        let mut tree = BinarySearchTree::new();
        for i in (0..100).rev() {
            tree.insert(i, 2 * i);
            // Find each new element as we insert it.
            assert_eq!(*(tree.search(&i).unwrap()), i * 2);
        }
        
        // Search for everything again now that we have all of the elements in the tree.
        for i in 0..100 {
            assert_eq!(*(tree.search(&i).unwrap()), i * 2);
        }

        // Repeat the test but insert keys in an arbitrary order.
        let mut tree = BinarySearchTree::new();
        for i in 0..100 {
            // Inserting hashes will give us random-looking, but deterministic, data.
            let key = calculate_hash(&i);
            let data = calculate_hash(&(i * 2));
            tree.insert(key, data);
            assert_eq!(*(tree.search(&key).unwrap()), data);
        }
        for i in 0..100 {
            let key = calculate_hash(&i);
            let data = calculate_hash(&(i * 2));
            assert_eq!(*(tree.search(&key).unwrap()), data);
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
    // Verify that the correct data is returned.
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

        // Repeat the test but insert keys in descending order.
        let mut tree = BinarySearchTree::new();
        for i in (0..100).rev() {
            tree.insert(i, 2 * i);
        }
        for i in 0..100 {
            assert_eq!(tree.remove(&i).unwrap(), i * 2);
            assert!(tree.search(&i).is_none());
        }

        // Repeat the test but insert keys in an arbitrary order.
        let mut tree = BinarySearchTree::new();
        for i in 0..100 {
            let key = calculate_hash(&i);
            let data = calculate_hash(&(i * 2));
            tree.insert(key, data);

        }
        for i in 0..100 {
            let key = calculate_hash(&i);
            let data = calculate_hash(&(i * 2));
            assert_eq!(tree.remove(&key).unwrap(), data);
            assert!(tree.search(&key).is_none());
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

    // Create a tree, insert some elements, and call a function to get elements
    // in order. Verify that the retured vector is in ascending order.
    #[test]
    fn in_order_traversal_no_iterator() {
        let mut tree = BinarySearchTree::new();

        for i in 0..100 {
            tree.insert(i, 2 * i);
        }
        let vector = tree.as_vector();
        let mut j = 0;
        // Make sure the inserted items are all there in the correct order.
        for data in vector {
            assert_eq!(*data, 2 * j);
            j += 1;
        }
        // Make sure we got through all 100 inserted elements.
        assert_eq!(j, 100);

        // Repeat the test but insert keys in descending order.
        let mut tree = BinarySearchTree::new();
        for i in (0..100).rev() {
            tree.insert(i, 2 * i);
        }
        let vector = tree.as_vector();
        let mut j = 0;
        // Make sure the inserted items are all there in the correct order.
        for data in vector {
            assert_eq!(*data, 2 * j);
            j += 1;
        }
        // Make sure we got through all 100 inserted elements.
        assert_eq!(j, 100);

        // Repeat the test but insert keys in an arbitrary order.
        let mut tree = BinarySearchTree::new();
        let mut expected = Vec::new();
        for i in 0..100 {
            let key = calculate_hash(&i);
            let data = calculate_hash(&(i * 2));
            tree.insert(key, data);
            expected.push((key, data));
        }
        // Sort the (key, data) array by key.
        expected.sort_by(|a, b| (a.0).cmp(&b.0));
        let vector = tree.as_vector();
        let mut j = 0;
        // Make sure the inserted items are all there in the correct order.
        for data in vector {
            assert_eq!(*data, expected[j].1);
            j += 1;
        }
        // Make sure we got through all 100 inserted elements.
        assert_eq!(j, 100);
    }

    // Create a tree, insert some elements, get an in-order iterator, and use it
    // to traverse the tree. Verify that iteration occurs in ascending order.
    #[test]
    fn in_order_traversal_with_iterator() {
        let mut tree = BinarySearchTree::new();

        for i in 0..100 {
            tree.insert(i, i);
        }

        let iter = tree.iter_in_order();
        let mut j = 0;
        // Make sure the inserted items are all there in the correct order.
        for i in iter {
            assert_eq!(*i, j);
            j += 1;
        }
        // Make sure we got through all 100 inserted elements.
        assert_eq!(j, 100);

        // Repeat the test but insert in descending order.
        let mut tree = BinarySearchTree::new();
        for i in 0..100 {
            tree.insert(i, i);
        }
        let iter = tree.iter_in_order();
        let mut j = 0;
        // Make sure the inserted items are all there in the correct order.
        for i in iter {
            assert_eq!(*i, j);
            j += 1;
        }
        // Make sure we got through all 100 inserted elements.
        assert_eq!(j, 100);

        // Repeat the test but insert keys in an arbitrary order.
        let mut tree = BinarySearchTree::new();
        let mut expected = Vec::new();
        for i in 0..100 {
            let key = calculate_hash(&i);
            let data = calculate_hash(&(i * 2));
            tree.insert(key, data);
            expected.push((key, data));
        }
        // Sort the (key, data) array by key.
        expected.sort_by(|a, b| (a.0).cmp(&b.0));
        let iter = tree.iter_in_order();
        let mut j = 0;
        // Make sure the inserted items are all there in the correct order.
        for data in iter {
            assert_eq!(*data, expected[j].1);
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
    // Three cases:
    //  1. Deleting a leaf node: Just delete it.
    //  2. Deleting a node with one child: Make the parent of the deleted node
    //      point to the child node.
    //  3. Deleting a node with two children: Replace the deleted node with its
    //      immediate successor (the leftmost node of its right subtree). In this
    //      case the immediate successor must be deleted from its original location,
    //      which will be one of the two simpler cases.
    pub fn remove(&mut self, key: &T) -> Option<U> {
        // If there's no root, there's nothing to delete.
        let i_root = match self.root {
            Some(root) => root,
            None => return None
        };

        // Find the index of the node, the index of its parent, and whether it
        // is the left (or right) child of its parent.
        let (parent, to_delete, is_left_child) = 
            self.find_with_parent(i_root, None, key);

        // If there's no node with the given key, there's nothing to delete.
        let i_delete = match to_delete {
            Some(i) => i,
            None => return None
        };

        let doomed_node = match self.nodes.remove(i_delete)
        {
            Some(node) => node,
            None => {
                panic!("Invalid index used in remove(). This is probably a bug\
                    in BinarySearchtree.");
            }
        };

        if doomed_node.left.is_some() && doomed_node.right.is_some() {
            // 3. Deleting a node with two children: Replace the deleted node
            //      with its successor. In this case the successor must be
            //      deleted from its original location, which will be one of the
            //      two simpler cases.
            // We can ignore the possibility that the node has no true successor
            // because we know the node has a right child, meaning it must have
            // a successor.
            let i_successor = self.successor(i_delete);
            let successor_node = self.nodes[i_successor].as_ref().unwrap();
            let key_successor = &(successor_node.key);
            let (s_parent, _successor, s_is_left) = 
                self.find_with_parent(i_root, None, key_successor);

            // Remove the pointers to the successor...
            // Successor is the doomed node's child (it can only be a right child).
            if doomed_node.right.unwrap() == i_successor {
                // If the doomed_node is the parent of its successor, there's
                // no need to change the child pointer of the successor's
                // parent, because that parent is itself being deleted. The
                // "Splice the successor back in" step below will do all that
                // is needed in this case.
            }
            // Successor has one child.
            else if doomed_node.left.is_some() || doomed_node.right.is_some() {
                    self.remove_single_child_case(i_successor,
                                             &successor_node,
                                             s_parent,
                                             s_is_left);
            }
            // Successor is a leaf. Remove its parent's pointer to it.
            else {
                if let Some(i_s_parent) = s_parent {
                   let parent_node = self.nodes.borrow_mutable(i_s_parent).
                       expect("find_with_parent() should return a valid parent index");
                   if let Some(is_left) = s_is_left {
                       if is_left {
                           parent_node.left = None;
                       }
                       else {
                           parent_node.right = None;
                       }
                   }
                   else {
                       panic!("If find_with_parent() returns a parent, it should \
                            also return a valid \"is left child\" boolean.");
                   }
                }
                else {
                    // If a node has two nodes, and therefore has a right child,
                    // its successor must exist and must be in its right subtree,
                    // meaning its successor must have a parent.
                    panic!("It shouldn't be possible for the successor of a \
                        node with two children to be the root. This is a bug in \
                        BinarySearchTree");
                }
            }
 

            // Splice the successor back in where the doomed node was.
            match parent {
                Some(i_parent) => {
                    let parent_node = self.nodes.borrow_mutable(i_parent);
                    let parent_node = parent_node.
                        expect("i_parent should be valid. There is a bug in BinarySearchTree.");
                    
                    if let Some(is_left) = is_left_child {
                       if is_left {
                           parent_node.left = Some(i_successor);
                       }
                       else {
                           parent_node.right = Some(i_successor);
                       }
                    }
                    else {
                       panic!("If find_with_parent() returns a parent, it should \
                            also return a valid \"is left child\" boolean.");
                    }
                },
                None =>  {
                    self.root = Some(i_successor);
                }
            }
            let successor_node = self.nodes.borrow_mutable(i_successor);
            let successor_node = successor_node.
                expect("i_successor should be valid. There is a bug in BinarySearchTree.");
            match doomed_node.left {
                Some(i_left) => successor_node.left = Some(i_left),
                None => successor_node.left = None
            }
            match doomed_node.right {
                Some(i_right) => successor_node.right = Some(i_right),
                None => successor_node.right = None
            }
        }
        else if doomed_node.left.is_some() || doomed_node.right.is_some() {
            self.remove_single_child_case(i_delete, &doomed_node, parent, is_left_child);
        }
        else {
            // 1. Deleting a leaf node: Just delete it.
            // Remove the parent's reference to the node if the node is not the root.
            if let Some(i_parent) = parent {
                let parent_node = self.nodes.borrow_mutable(i_parent).
                    expect("i_parent should be valid. There is a bug in BinarySearchTree.");
                if is_left_child.expect("This node has a parent. It either is or is \
                                        not a left child.") {
                    parent_node.left = None;
                }
                else {
                    parent_node.right = None;
                }
            }
        }

        // Return the contents of the deleted node.
        Some(doomed_node.data)
    }

    // Helper function for remove():
    //  2. Deleting a node with one (left) child: Make the parent of the
    //  deleted node point to the child node.
    fn remove_single_child_case(
        &mut self,
        i_delete: usize,
        doomed_node: &Node<T, U>,
        parent: Option<usize>,
        is_left_child: Option<bool>)
    {
        if doomed_node.left.is_some() {
            match parent {
                Some(i_parent) => {
                    let parent_node = self.nodes.
                        borrow_mutable(i_parent).
                        expect("i_parent should be valid. There is a bug in BinarySearchTree.");
                    // If the doomed node is its parent's left child.
                    if is_left_child.
                        expect("If the node has a parent, it must be a child") {
                        parent_node.left =
                            Some(self.nodes[i_delete].
                                as_ref().
                                expect("i_delete should be valid. There is a bug in BinarySearchTree").
                                left.
                                unwrap());
                    }
                    else { // If the doomed node is its parent's right child.
                        parent_node.right =
                            Some(self.nodes[i_delete].
                                 as_ref().
                                 expect("i_delete should be valid. There is a bug in BinarySearchTree").
                                 left.
                                 unwrap());
                    }

                },
                None => {
                    // We are deleting the root. Make the child the new root.
                    self.root = doomed_node.left;
                }
            }
        }
        else if doomed_node.right.is_some() {
            match parent {
                Some(i_parent) => {
                    let parent_node = self.nodes.
                        borrow_mutable(i_parent).
                        expect("i_parent should be valid. There is a bug in BinarySearchTree.");
                    // If the doomed node is its parent's left child.
                    if *is_left_child.
                        as_ref().
                        expect("If the node has a parent, it must be a child") {
                        parent_node.left =
                            Some(self.nodes[i_delete].
                                 as_ref().
                                 expect("i_delete should be valid. There is a bug in BinarySearchTree").
                                 right.
                                 unwrap());
                    }
                    else { // If the doomed node is its parent's right child.
                        parent_node.right =
                            Some(self.nodes[i_delete].
                                 as_ref().
                                 expect("i_parent should be valid. There is a bug in BinarySearchTree.").
                                 right.
                                 unwrap());
                    }
                },
                None => {
                    // We are deleting the root. Make the child the new root.
                    self.root = doomed_node.right;
                }
            }
        }
        else {
            panic!("remove_single_child_case() should not be called unless the \
                    doomed node has exactly one child. This is a bug in \
                    BinarySearchTree.");
        }
    }

    // Find the node with the given key and return its index, the index of its
    // parent, and whether the child is the left child of its parent in a
    // (parent, child, is_left_child) tuple. If the node is the root, parent will
    // be None and if the node doesn't exist, both will be None. The is_left_child
    // value will be none if there is no parent.
    // Initially, current should be the index of the root and previous should be
    // None. If the tree is empty (has no root), this function sould not be called.
    fn find_with_parent(&self, current: usize, previous: Option<usize>, key: &T)
        -> (Option<usize>, Option<usize>, Option<bool>)
    {
        let current_node = match &self.nodes[current] {
            Some(node) => node,
            None => {
                panic!("An invalid node index was passed to find_with_parent().\
                This is probably a bug in BinarySearchTree's implementation.");
            }
        };

        if *key < current_node.key {
            match current_node.left {
                // Search the left subtree.
                Some(left_index) =>
                    self.find_with_parent(left_index, Some(current), key),
                // The given key does not exist in the tree.
                None => (None, None, None)
            }
        }
        else if *key > current_node.key {
            match current_node.right {
                // Search the right subtree.
                Some(right_index) =>
                    self.find_with_parent(right_index, Some(current), key),
                // The given key does not exist in the tree.
                None => (None, None, None)
            }
        }
        else {
            // We've found the key.
            match previous {
                Some(i_parent) => {
                    let is_left_child;
                    let parent = self.
                        nodes[i_parent].
                        as_ref().
                        expect("i_parent should be valid. There is a bug in BinarySearchTree.");
                    is_left_child =
                        parent.left.is_some() && parent.left.unwrap() == current;
                    (Some(i_parent), Some(current), Some(is_left_child))
                },
                None => (None, Some(current), None)
            }
        }
    }

    // Return the index of the successor of the node at the given index.
    // Assumes the index argument is a valid index.
    // If the node at index has no successor, it will be returned as its own
    // successor.
    fn successor(&self, index: usize) -> usize {
        let mut current;
        
        // Get the index of the right subtree if it exists. Otherwise, return
        // the given index as its own successor.
        match self.
            nodes[index].
            as_ref().
            expect("invalid index in BinaryTree.Successor()").
            right {

            Some(right) => current = right,
            None => return index
        }
        
        while let Some(left) =
            self.
            nodes[current].
            as_ref().
            expect("invalid index in BinaryTree.Successor()").
            left {

            current = left;
        }

        current
    }

    /// Get an iterator to perform an in-order traversal on the tree, returning
    /// a reference to the data stored in the next node with each iteration.
    pub fn iter_in_order(&self) -> InOrderIterator<T, U> {
        InOrderIterator {
            tree: self,
            stack: Vec::new(),
            current: match self.root {
                Some(root) => Some(root),
                None => None
            },
            go_left: true
        }
    }

    /// Get a Vec of references to the data stored in the tree, in ascending
    /// order.
    // We will accomplish this with an in-order traversal.
    // inorder traversal: left, current, right
    pub fn as_vector(&self) -> Vec<&U> {
        let mut vector = Vec::new();
        let mut current;
        let mut stack = Vec::new();

        current = match self.root {
            Some(root) => Some(root),
            None => None
        };

        while current.is_some() || !stack.is_empty() {
            // Keep moving left and pushing the leftmost node to the stack.
            while let Some(cur) = current {
                stack.push(cur);
                current = match &self.nodes[cur] {
                    Some(node) => match node.left {
                        Some(left) => Some(left),
                        None => None
                    },
                    None => {
                        panic!("Attempted to use invalid index in as_vector().\
                            This is probably a bug in BinarySearchTree.");
                    }
                };
            }

            while current.is_none() && !stack.is_empty() {
                // Process the leftmost node (i.e. add it to the result vector).
                // We can unwrap because we know the stack is not empty.
                let cur = stack.pop().unwrap();
                // We can unwrap here. If cur was invalid, we would've panicked
                // in the previous loop.
                vector.push(&(self.nodes[cur].as_ref().unwrap().data));
                // Go right.
                current = match &self.nodes[cur] {
                    Some(node) => match node.right {
                        Some(right) => Some(right),
                        None => None
                    },
                    None => {
                        panic!("Attempted to use invalid index in as_vector().\
                            This is probably a bug in BinarySearchTree.");
                    }
                };
            }
        }

        vector
    }
}

pub struct InOrderIterator<'a, T, U>
    where T: PartialOrd + Eq
{
    tree: &'a BinarySearchTree<T, U>,
    stack: Vec<usize>,
    current: Option<usize>,
    go_left: bool
}

impl<'a, T, U> Iterator for InOrderIterator<'a, T, U>
    where T: PartialOrd + Eq
{
    type Item = &'a U;

    // We want to do an in-order traversal, returning a single element with
    // each call to next().
    // Based on the traversal performed in BinarySearchTree.as_vector(), we
    // have two stages:
    //  1. Move all the way to the leftmost node of the current subtree.
    //  2. Process the current node (return its data) and move right if possible.
    // After each stage, change the state of a bool to keep track of which
    // stage we're in. Each call to next() must cause the second stage to
    // execute exactly once if there are still nodes to traverse.
    fn next(&mut self) -> Option<Self::Item> {

        let mut to_return = None;

        if self.current.is_some() || !self.stack.is_empty() {
            if self.go_left {
                // Keep moving left and pushing the leftmost node to the stack.
                while let Some(cur) = self.current {
                    self.stack.push(cur);
                    self.current = match &self.tree.nodes[cur] {
                        Some(node) => match node.left {
                            Some(left) => Some(left),
                            None => None
                        },
                        None => {
                            panic!("Attempted to use invalid index in as_vector().\
                                This is probably a bug in BinarySearchTree.");
                        }
                    };
                }
                self.go_left = false;
            }

            if self.current.is_none() && !self.stack.is_empty() {
                // Process the leftmost node (i.e. add it to the result vector).
                // We can unwrap because we know the stack is not empty.
                let cur = self.stack.pop().unwrap();
                // We can unwrap here. If cur was invalid, we would've panicked
                // in the while loop.
                to_return = Some(&(self.tree.nodes[cur].as_ref().unwrap().data));
                // Go right.
                self.current = match &self.tree.nodes[cur] {
                    Some(node) => match node.right {
                        Some(right) => {
                            self.go_left = true;
                            Some(right)
                        },
                        None => None
                    },
                    None => {
                        panic!("Attempted to use invalid index in as_vector().\
                            This is probably a bug in BinarySearchTree.");
                    }
                };
            }
        }
        
        to_return
    }
}
