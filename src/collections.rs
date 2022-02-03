
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
        assert_eq!(l.iter().next(), 10);
    }

    #[test]
    fn iteration_test() {
        let mut l = LinkedList::<i32>::new();

        for i in 0..10 {
            l.insert(i);
        }

        // The linked list should use an iterator to transparently return a
        // reference to the data field.
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
    head: Option<Box<Node<T>>>,
    // The current node of the linked list. Used by the iterator.
    current: Option<Box<Node<T>>>
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            // The list is not empty.
            Some(headNode) => {
                match self.current {
                    // We have a current node.
                    Some(currentNode) => {
                        match currentNode.next {
                            // There is a next node. Advance and return its value.
                            Some(nextNode) => {
                                self.current = Some(nextNode);
                                Some(self.current.expect("This shouldn't fail.").data)
                            },
                            // We've reached the end of the list.
                            None => {
                                None
                            }
                        }
                    },
                    // The list is not empty but we have no current node, set
                    // current to head and return current.data.
                    None => {
                        self.current = Some(headNode);
                        Some(self.current.expect("This shouldn't fail.").data)
                    }
                }
            },
            // The list is empty. Return None.
            None => None
        }
        // If current.next is none (i.e. we've reached the end of the list), return None.
        // Else, advance to the next node and return self.current.data.
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
