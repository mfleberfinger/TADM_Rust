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
