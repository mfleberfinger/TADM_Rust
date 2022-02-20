use std::rc::{Rc, Weak};

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

#[cfg(test)]
mod linked_list_tests {
    use super::*;

    #[test]
    fn insertion_test() {
        let mut l = LinkedList::<i32>::new();
        l.insert(10);
        assert_eq!(l.next().expect("Iterator should return the value in the list."),
            10);
    }

    // The linked list should use an iterator to transparently return data.
    // The calling code will not know about Nodes.
    #[test]
    fn iteration_test() {
        let mut l = LinkedList::<i32>::new();

        for i in 0..10 {
            l.insert(i);
        }

        assert_eq!((&mut l).count(), 10);

        l.reset();

        // Insertion into the list occurs at the head, so the values inserted
        // above will be returned in reverse order.
        let mut i = 9;
        for data in l {
            println!("Next list item: {}; Expecting: {}", data, i);
            assert_eq!(data, i);
            i -= 1;
        }
    }
}

/// A linked list. Hides the low level details from the user.
/// Only holds types that implement the Copy trait.
// To make this easier, for now, the listed list will only support types that
// implement Copy.
// TODO: Learn how to properly handle linked data structures before attempting
// graphs or trees*. This implementation is crap.
// TODO: Implement a function to remove items from the list.
// * A note on graphs and trees: It may be reasonable to make a purely Vec<T>
// based adjacency list, eliminating the need for pointers.
pub struct LinkedList<T>
    where T: Copy
{
    // The head of the linked list.
    head: Option<Rc<Node<T>>>,
    // The current node of the linked list. Used by the iterator.
    current: Option<Weak<Node<T>>>
}

struct Node<T>
    where T: Copy
{
    data: T,
    next: Option<Rc<Node<T>>>
}

impl<T> Iterator for LinkedList<T>
    where T: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {

        // If head is None, the list is empty. Return None.
        if self.head.is_none() {
            None
        }
        // If head is Some but current is none, set current to head and return
        // the data in the current (and head) node.
        else if self.current.is_none() {
            self.current = Some(Rc::downgrade(&(self.head.as_ref().unwrap())));
            Some((*self.current.as_ref().unwrap().upgrade().unwrap()).data)
        }
        // If current is not none but its weak reference is, something went wrong.
        else if self.current.as_ref().unwrap().upgrade().is_none() {
            panic!("The weak reference in a LinkedList's current node is invalid.\
                   This is probably a bug in LinkedList<T>'s implementation.");
        }
        // We know current is not none if we reach this, so we can use unwrap().
        // If current.next is none (i.e. we've reached the end of the list), return None.
        else if self.current.as_ref().unwrap().upgrade().unwrap().next.is_none() {
            None
        }
        // Else, advance to the next node and return self.current.data.
        else {
            // We know there's something to unwrap here because the branches
            // above this one have ensured it.
            self.current =
                Some(
                    Rc::downgrade(
                        &((*(self.current.as_ref().unwrap().upgrade().unwrap())).next.as_ref().unwrap())
                    )
                );
            // We know there's something to unwrap() because we just put it there.
            Some((*self.current.as_ref().unwrap().upgrade().unwrap()).data)
        }
    }
}

impl<T> LinkedList<T>
    where T: Copy
{
    /// Returns a new empty list.
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            current: None
        }
    }

    /// Resets the list's internal pointer.
    /// Allows the list to be iterated over from the start.
    pub fn reset(&mut self) {
        self.current = None
    }

    /// Inserts a new item at the head of the list.
    pub fn insert(&mut self, data: T) {
        let mut n: Node<T> = Node {
            data,
            next: None
        };

        match &self.head {
            Some(h) => {
                // Put a reference to the head node in n.next.
                n.next = Some(Rc::clone(&h));
                // Make n the head node.
                self.head = Some(Rc::new(n));
            },
            None => { 
                self.head = Some(Rc::new(n));
            }
        }
    }
}


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
    /// Returns a new empty stack.
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
    // TODO: This should return an Option<T>, there should be a way to
    // get the number of items on the stack, or both.
    pub fn pop(&mut self) -> T {
        if let Some(item) = self.vector.pop() {
            item
        }
        else {
            panic!("Cannot pop from an empty stack.");
        }
    }
}
