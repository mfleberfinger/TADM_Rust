//use std::cell::RefCell;
//use std::rc::{Rc, Weak};

/* TODO: Rust doesn't allow dynamic sizing of its standard arrays.
 * Apparently, implementing a dynamic array requires "advanced" Rust,
 * including unsafe code (https://doc.rust-lang.org/nomicon/vec/vec.html).
 * This doesn't seem like a good place to start.
#[cfg(test)]
mod dynamic_array_tests {
    use super::*;

    // Make sure a dynamic array accepts data and grows as needed.
    // Make sure the size and count are updated correctly when items are added.
    #[test]
    fn add_items() {
    }

    // Make sure that attempting to index into unused but allocated space
    // results in a panic.
    #[test]
    #[should_panic(expected = "Attempted to access unitialized dynamic array index.")]
    fn bounds_check() {
    }
}


pub struct DynamicArray<T> {
    size: usize,
    items: Box<[T]>
}

impl<T> DynamicArray<T> {
    pub fn new() -> DynamicArray<T> {
        DynamicArray {
            size: 1,
            items: Box::new([T; 1])
        }
    }

    pub fn append(item: T) {
    }
}
*/

/* TODO: Apparently, linked lists are  not a beginner friendly data structure
 * in Rust either. Come back to it later.
#[cfg(test)]
mod linked_list_tests {
    use super::*;

    #[test]
    fn insertion_test() {
        let mut l = LinkedList::<i32>::new();
        l.insert(10);
        assert_eq!(l.iter().next(), 10);
    }

    #[test]
    fn iteration_test() {
        let mut l = LinkedList::<i32>::new();

        for i in 0..10 {
            l.insert(i);
        }

        // The linked list should use an iterator to transparently return data.
        // The calling code will not know about Nodes.
        let mut i = 0;
        for data in l {
            assert_eq!(data, i);
            i += 1;
        }
    }
}

/// A linked list. Hides the low level details from the user.
pub struct LinkedList<T> {
    // The head of the linked list.
    // Owns the first node of the list.
    head: Option<RefCell<Rc<Node<T>>>>,
    // The current node of the linked list. Used by the iterator.
    // Current should be a weak reference. It should not own anything.
    current: RefCell<Weak<Node<T>>>
}

struct Node<T> {
    data: T,
    // Each node owns its child.
    next: Option<RefCell<Rc<Node<T>>>>
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = None;

        // If head is None, the list is empty. Return None.
        if self.head.is_none() {
            ret = None;
        }
        // If head is Some but current is none, set current to head and return self.current.data.
        else if self.current.is_none() {
        }
        // If current.next is none (i.e. we've reached the end of the list), return None.
        else if self.current.next.is_none() {
        }
        // Else, advance to the next node and return self.current.data.
        else {
        }

        ret
    }
}

impl<T> LinkedList<T> {
    /// Returns a new empty list.
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            current: None
        }
    }

    /// Inserts a new node containing data at the beginning of the list.
    pub fn insert(&mut self, data: T) {

    }
}
*/

#[cfg(test)]
mod vec_stack_tests {
    use super::*;

    // Push some data to the stack. Pop it all off the stack. Verify that all
    // of the data is returned in the correct order.
    #[test]
    fn push_and_pop() {
        let mut stack = VecStack::new();

        // Fill the stack.
        for i in 0..10 {
            stack.push(i);
        }

        // Empty the stack and make assertions.
        for i in (0..10).rev() {
            assert_eq!(i, stack.pop());
        }
    }

    // Pop from an empty stack. This is an error on the part of the caller and
    // should result in a panic.
    #[test]
    #[should_panic(expected = "Cannot pop from an empty stack.")]
    fn pop_from_empty() {
        let mut stack:VecStack<i32> = VecStack::new();
        stack.pop();
    }
}

/// A vector-based stack implementation.
/// Data stored on the stack is owned by the stack. Ownership of data returned
/// by pop() will be given to the caller of pop().
// Implementing a queue this way probably wouldn't work but this should be easy.
// Considering that vec<T> implements its own push and pop() this is trivial...
pub struct VecStack<T> {
    // The vector that stores our data.
    vector: Vec<T>,
}

impl<T> VecStack<T> {
    pub fn new() -> VecStack<T> {
        VecStack {
            vector: Vec::new()
        }
    }

    /// Puts an item on the top of the stack.
    pub fn push(&mut self, item: T) {
        self.vector.push(item);
    }

    /// Removes and returns the item on the top of the stack.
    pub fn pop(&mut self) -> T {
        if let Some(item) = self.vector.pop() {
            item
        }
        else {
            panic!("Cannot pop from an empty stack.");
        }
    }
}
