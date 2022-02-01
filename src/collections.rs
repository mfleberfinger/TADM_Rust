use std::cell:RefCell;

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

#[Cfg(Test)]
mod linked_list_tests() {
    use super::*;

    #[test]
    fn insertion_test() {
        let mut l = LinkedList<i32>::new();
        l.insert(10);
        assert_eq(l.data, 10);
    }

    // The linked list should use an iterator to transparently return data.
    // The user will not know about nodes.
    #[test]
    fn iteration_test() {
        let mut l = LinkedList<i32>::new();

        for i in 0..10 {
            l.insert(i);
        }

        let mut i = 0;
        for data in l {
            assert_eq(data, i);
            i += 1;
        }
    }
}

/// A linked list. Hides the low level details from the user.
pub struct LinkedList<T> {
    head: Option<Box<LinkedList<T>>>
}

struct Node<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>
}

impl<T> LinkedList<T> {
    /// Returns a new empty list.
    pub fn new() -> LinkedList<T> {
        
    }

    /// Inserts a new node containing data at the beginning of the list.
    pub fn insert(&mut self, data: T) {

    }
}
