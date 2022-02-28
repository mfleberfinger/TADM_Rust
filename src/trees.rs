#[cfg(test)]
mod BinaryTree_Tests {
    use super::*;

}


struct Node<T> {
    // Data stored in this node.
    data: T,
    // Indices of the left and right child in the "arena" holding this tree.
    left: Option<usize>,
    right: Option<usize>
}

/// A simple binary tree. Notice that this is not necessarily a binary search tree.
pub struct BinaryTree<T> {
    arena: Vec<Option<Node<T>>>,
    root: usize
}
