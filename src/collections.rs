use std::rc::{Rc, Weak};
use std::hash::{Hash, Hasher};
use std::ops;
// We'll use Rust's hasher instead of writing our own for now.
use std::collections::hash_map::DefaultHasher;

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


#[cfg(test)]
mod heap_tests {
    use super::*;

    // Extracting from an empty heap should return None.
    #[test]
    fn extract_from_empty() {
        let mut max_heap: Heap<i32> = Heap::new(true);
        let mut min_heap: Heap<i32> = Heap::new(false);

        assert!(max_heap.extract().is_none());
        assert!(min_heap.extract().is_none());
    }
    
    // Use heapsort to test that all values are inserted and extracted in the
    // correct order.
    #[test]
    fn heapsort_test() {
        let mut max_heap = Heap::new(true);
        let mut min_heap = Heap::new(false);

        // Sort an arbitrarily ordered list of integers in ascending and
        // descending order by inserting into and removing from a min and max
        // heap.
        let arbitrary_order = vec![0, -3, -1, 3, 2, 1, -2];
        for i in arbitrary_order {
            min_heap.insert(i);
            max_heap.insert(i);
        }
        assert_ascending_order(&mut min_heap);
        assert_descending_order(&mut max_heap);

        // The heaps should now be empty.
        // Sort a list of integers that is already in ascending order.
        for i in -3..4 {
            min_heap.insert(i);
            max_heap.insert(i);
        }
        assert_ascending_order(&mut min_heap);
        assert_descending_order(&mut max_heap);

        // The heaps should now be empty again.
        // Sort a list of integers that is already in descending order.
        for i in (-3..4).rev() {
            min_heap.insert(i);
            max_heap.insert(i);
        }
        assert_ascending_order(&mut min_heap);
        assert_descending_order(&mut max_heap);
    }

    // Extracting everything from the min heap should result in ascending order.
    fn assert_ascending_order(min_heap: &mut Heap<i32>) {
        let expect_message = "Failed to extract all inserted elements from the heap.";
        println!("Min-heap contains: {:?}", min_heap);
        for i in -3..4 {
            let extracted = min_heap.extract().expect(expect_message);
            println!("Expected {}. Extracted {}", i, extracted);
            println!("Min-heap contains: {:?}", min_heap);
            assert_eq!(i, extracted);
        }
    }

    // Extracting everything from the max heap should result in descending order.
    fn assert_descending_order(max_heap: &mut Heap<i32>) {
        let expect_message = "Failed to extract all inserted elements from the heap.";
        println!("Max-heap contains: {:?}", max_heap);
        for i in (-3..4).rev() {
            let extracted = max_heap.extract().expect(expect_message);
            println!("Expected {}. Extracted {}", i, extracted);
            println!("Max-heap contains: {:?}", max_heap);
            assert_eq!(i, extracted);
        }
    }
}

/// A heap. Can be configured as a min-heap or a max-heap when constructed.
#[derive(Debug)]
pub struct Heap<T> {
    is_max_heap: bool,
    vector: Vec<T>
}

impl<T> Heap<T>
    where T: PartialOrd
{
    pub fn new(is_max_heap: bool) -> Heap<T> {
        Heap {
            is_max_heap,
            vector: Vec::new()
        }
    }

    /// Inserts an item into the heap.
    // Puts item at the end of the vector (the bottom-most, rightmost leaf of
    // the heap). If item dominates its parent, moves item up recursively.
    pub fn insert(&mut self, item: T) {
        self.vector.push(item);
        self.bubble_up(self.vector.len() - 1);
    }

    // Recursively moves the item at index i up the heap until it is dominated
    // by its parent.
    fn bubble_up(&mut self, i: usize) {
        // If the current node is already the root, we're done.
        if i > 0 {
            let p = Heap::<T>::parent(i);
            
            if self.dominates(i, p) {
                self.vector.swap(i, p);
                self.bubble_up(p);
            }
        }
    }


    /// Removes the minimum or maximum value, depending on whether this is a
    /// min or max heap, and returns it. Returns None if the heap is empty.
    // Gets the first element in the vector and removes it. If there is
    // more than one element in the vector, the last element is moved to
    // the beginning of the vector.
    pub fn extract(&mut self) -> Option<T> {
        if self.vector.is_empty() {
            None
        }
        else {
            let dominant = self.vector.swap_remove(0);
            self.bubble_down(0);

            Some(dominant)
        }
    }

    // Starting from index i, recursively swap the current value with its most
    // dominant child. In the case where the value at i dominates both of its
    // children, we're done (the recursion bottoms out).
    fn bubble_down(&mut self, i: usize) {
        // If there is nothing in the heap we're done.
        // If the current node has no children, we're done.
        if Heap::<T>::left_child(i) < self.vector.len() {
            // Get the index of the most dominant element among the current
            // element and its (at most) two children.
            let mut i_dom = i;
            i_dom = self.get_dominant(Heap::<T>::left_child(i), i_dom);
            if Heap::<T>::right_child(i) < self.vector.len() {
                i_dom = self.get_dominant(Heap::<T>::right_child(i), i_dom);
            }

            // If the current item dominates its children, we're done.
            if i_dom != i {
                self.vector.swap(i, i_dom);
                self.bubble_down(i_dom);
            }
        }
    }

    // Returns the index of the left child of the element at index i.
    // Does not check whether the child exists.
    fn left_child(i: usize) -> usize {
        (2 * i) + 1
    }

    // Returns the index of the right child of the element at index i.
    // Does not check whether the child exists.
    fn right_child(i: usize) -> usize {
        (2 * i) + 2
    }

    // Returns the index of the parent of the element at index i.
    fn parent(i: usize) -> usize {
        if i < 1 {
            panic!("The root cannot have a parent.");
        }
        else {
            (i - 1) / 2
        }
    }

    // Returns true if i dominates j. False otherwise.
    // Assumes i and j are valid indices into self.vector.
    fn dominates(&self, i: usize, j: usize) -> bool {
        (self.is_max_heap && self.vector[i] > self.vector[j])
            || (!self.is_max_heap && self.vector[i] < self.vector[j])
    }

    // Compares the elements at indices i and j and returns the index of the
    // dominant element, based on whether this is a max or min heap.
    // Assumes that i and j are valid indices into self.vector.
    fn get_dominant(&self, i: usize, j: usize) -> usize {
        if self.dominates(i, j) {
            i
        }
        else {
            j
        }
    }
}


#[cfg(test)]
mod hashset_tests {
    use super::*;

    // Tests insertion of a duplicate value (should not be allowed).
    #[test]
    #[should_panic(expected = "Cannot insert a duplicate value into a hashset.")]
    fn insert_duplicate() {
        let mut h = Hashset::new();
        h.insert(1);
        h.insert(1);
    }

    // Tests insertion and lookup.
    #[test]
    fn insert_and_lookup() {
        let mut h = Hashset::new();
        // Test with one element in the set.
        h.insert(0);
        assert_eq!(h[0], 0);

        for i in 1..100 {
            h.insert(i);
        }

        // Test with 100 elements in the set.
        for i in 0..100 {
            assert_eq!(h[i], i);
        }
    }

    // Tests insertion and the contains() function.
    #[test]
    fn insert_and_contains() {
        let mut h = Hashset::new();
        // Test with 1 element in the set.
        h.insert(0);
        assert!(h.contains(&0));
        assert!(!h.contains(&1));

        for i in 1..100 {
            h.insert(i);
        }

        // Test with 100 elements in the set.
        for i in 1..100 {
            assert!(h.contains(&i));
            println!("-1 * {} = {}", i, (-1 * i));
            assert!(!h.contains(&(-1 * i)));
        }
    }

    // Tests insertion and deletion.
    #[test]
    fn insert_and_remove() {
        let mut h = Hashset::new();
        // Test with 1 element in the set.
        h.insert(0);
        assert!(h.contains(&0));
        h.remove(&0);
        assert!(!h.contains(&0));

        for i in 0..100 {
            h.insert(i);
        }

        // Remove the last element inserted.
        h.remove(&99);
        assert!(!h.contains(&99));
        
        // Remove everything else.
        for i in 0..99 {
            h.remove(&i);
            assert!(!h.contains(&i));
        }
    }

    // Tests that the remove() function returns the value.
    #[test]
    fn insert_and_reclaim_ownership() {
        let mut h = Hashset::new();

        h.insert(String::from("This is a test."));
        let s = h.remove(&String::from("This is a test."));

        assert_eq!(s, "This is a test.");
    }

    // "Deletion in an open addressing scheme can get ugly, since removing one
    // element might break a chain of insertions, making some elements
    // inaccessible. We have no alternative but to reinsert all the items in
    // the run following the new hole."
    // Insert a number of elements, then delete them one by one, accessing all
    // elements after each deletion and asserting that they are found.
    #[test]
    fn remove_and_lookup() {
        let mut h = Hashset::new();
        
        // Test with 10 elements.
        for i in 0..10 {
            h.insert(i);
        }
        for i in 0..10 {
            h.remove(&i);
            for j in (i + 1)..10 {
                assert_eq!(h[j], j);
            }
        }

        // Test with 100 elements.
        for i in 0..100 {
            h.insert(i);
        }
        for i in 0..100 {
            h.remove(&i);
            for j in (i + 1)..10 {
                assert_eq!(h[j], j);
            }
        }
    }

    // Make sure count() always returns the number of elements in the set.
    // Start with 0, do some inserts, checking the count each time, then delete
    // everything, checking the count each time.
    #[test]
    fn insert_and_count() {
        let mut h = Hashset::new();

        for i in 0..100 {
            assert_eq!(h.count(), i);
            h.insert(i);
        }

        assert_eq!(h.count(), 100);

        for i in (0..100).rev() {
            h.remove(&i);
            assert_eq!(h.count(), i);
        }
    }

}

/// A simple hashset.
/// Allows insertion, retrieval, and deletion of elements.
// TODO: Start by using the built in  hash functions. Maybe implement a simple
// hash function later if desired.
pub struct Hashset<T>
    where T: Hash + Eq
{
    vector: Vec<Option<T>>,
    // Even though the vector keeps track of its length and capacity, we need
    // to keep track of the part of the vector we have initialized to None.
    capacity: usize,
    // We also need to keep track of the number of actual elements we have (not Nones).
    count: usize
}

impl<T> Hashset<T>
    where T: Hash + Eq
{
    /// Creates a new empty hashset.
    pub fn new() -> Hashset<T> {
        let capacity = 10;
        let mut h = Hashset::<T> {
            vector: Vec::with_capacity(capacity),
            capacity: capacity,
            count: 0
        };

        for _ in 0..h.capacity {
            h.vector.push(None);
        }

        h
    }

    /// Inserts a new value into the hashset.
    /// #Panics
    /// This function will panic if an attempt is made to insert a value that
    /// already exists in the hashset.
    pub fn insert(&mut self, value: T) {
        self.insert_internal(value, true);
    }

    fn insert_internal(&mut self, value: T, increment_count: bool) {

        // Increase our capacity if the vector is more than 3/4 full.
        // When the vector is 3/4 full, we expect to probe 4 times on average
        // before finding an available slot for the new item.
        // These numbers come from the "Performance of Open Addressing" section
        // of the document found at:
        //      https://courses.csail.mit.edu/6.006/spring11/rec/rec07.pdf
        if ((self.count + 1) as f64) > (self.capacity as f64) * 0.75 {
            self.grow();
        }

        let next_index = self.get_index(&value, true);
        match next_index {
            Some(i) => {
                self.vector[i] = Some(value);
                // We don't want to increment the count if we're reinserting
                // after increasing capacity.
                if increment_count {
                    self.count += 1;
                }
            }
            None => {
                // If we're out of space, grow the hashset and try inserting again.
                self.grow();
                self.insert_internal(value, true);
            }
        }
    }

    /// Returns true if the hashset contains the given value.
    /// Otherwise, returns false.
    pub fn contains(&self, value: &T) -> bool {
        self.get_index(&value, false).is_some()
    }

    /// Removes a value from the hashset and returns it.
    // "Deletion in an open addressing scheme can get ugly, since removing one
    // element might break a chain of insertions, making some elements
    // inaccessible. We have no alternative but to reinsert all the items in
    // the run following the new hole."
    pub fn remove(&mut self, value: &T) -> T {
        // Push none and swap it with the removed element to preserve the positions
        // of the elements in the vector.
        self.vector.push(None);
        self.vector.swap_remove(0).unwrap()
    }

    /// Returns the total number of elements in the set.
    pub fn count(&self) -> usize {
        self.count
    }

    // Calculates and returns the hash of to_hash.
    fn calculate_hash(to_hash: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        to_hash.hash(&mut hasher);
        hasher.finish()
    }

    // Returns the index in the vector that the given value hashes to.
    // If there is a collision, this is the index from which sequential probing
    // would begin.
    fn get_first_index(&self, value: &T) -> usize {
        // As long as this code doesn't end up on a 128-bit processor, it should
        // be okay to convert usize into u64.
        // It should also be okay to convert the u64 resulting from this
        // remainder operation back into a usize because it can be no larger
        // than the original usize.
        (Hashset::<T>::calculate_hash(value) % (self.capacity as u64)) as usize
    }

    // If the is_insert parameter is false:
    //      Get the index in the vector where a value resides, or None if the
    //      value is not found.
    // If the is_insert parameter is true:
    //      Get the index in the vector where we should insert the given value,
    //      or none if we are out of space. Panic if the given value is already
    //      in the vector.
    fn get_index(&self, value: &T, is_insert: bool) -> Option<usize> {
        let mut found_value = false;

        // Get the index the value hashes to.
        let mut i = self.get_first_index(value);
        // We don't need to make sure i is less than our capacity here
        // because get_first_index() should return an index we already
        // initialized.
        found_value = self.vector[i].is_some()
            && self.vector[i].as_ref().unwrap() == value;
        // j tracks how many times we've iterated. While doing our sequential probe,
        // we may wrap around to the start of the vector, resetting i to 0 at
        // most once.
        let mut j = 0;
        while !found_value && j + 1 < self.capacity &&
            self.vector[(i + 1) % self.capacity].is_some() {
            i = (i + 1) % self.capacity;
            j += 1;
            found_value = self.vector[i].as_ref().unwrap() == value;
        }

        if is_insert {
            if found_value {
                panic!("Cannot insert a duplicate value into a hashset.");
            }
            else if j + 1 >= self.capacity {
                None
            }
            else {
                // Return the index of the first empty space.
                Some((i + 1) % self.capacity)
            }
        }
        else {
            if !found_value {
                None
            }
            else {
                // Return the index where we found the value.
                Some(i)
            }
        }
    }

    // Double the size of the initialized vector and reinsert everything.
    fn grow(&mut self) {
        // Remove all of the elements from the vector, drop the Nones, and put
        // the Somes in a local vector.
        let mut data: Vec<_> = self.vector.drain(..).filter(|x| x.is_some()).collect();
        self.capacity = self.capacity * 2;
        // Resize the vector and fill the new space with None.
        // The "|| None" is a closure that returns None.
         self.vector.resize_with(self.capacity, || None);

        
        // Reinsert all of the data.
        for i in (0..data.len()).rev() {
            // We can unwrap because we guaranteed that data only contains Somes
            // when we created it.
            self.insert_internal(data.swap_remove(i).unwrap(), false);
        }
    }

}

// Allows hashset elements to be accessed with the "[]" syntax.
impl<T> ops::Index<T> for Hashset<T>
    where T: Hash + Eq
{
    type Output = T;

    fn index(&self, value: T) -> &Self::Output {
        // TODO: We're ignoring the value parameter right now so we can just run
        // the failing tests but we'll use the hash of the value parameter in
        // the real implementation.
        &self.vector[0].as_ref().expect("No reason to expect this but we're expecting a failure anyway.")
    }
}

// TODO: Write a key value pair struct that implements the Hash trait, causing
// it to be hashed by the key only, regardless of the value. Use this struct
// with the hashset to implement a hashmap.
// It will probably be necessary to implement the Eq(?) trait as well and have
// it test equality by checking the equality of the keys only.
