use std::hash::{Hash, Hasher};
use std::ops;
// We'll use Rust's hasher instead of writing our own for now.
use std::collections::hash_map::DefaultHasher;

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
    fn count_test() {
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

    #[test]
    #[should_panic(expected = "Encountered attempt to look up a value not in the hashset")]
    fn invalid_lookup_empty() {
        let h = Hashset::new();
        h[1];
    }
    #[test]
    #[should_panic(expected = "Encountered attempt to look up a value not in the hashset")]
    fn invalid_lookup_before_grow() {
        let mut h = Hashset::new();
        h.insert(2);
        h[1];
    }
    #[test]
    #[should_panic(expected = "Encountered attempt to look up a value not in the hashset")]
    fn invalid_lookup_after_grow() {
        let mut h = Hashset::new();

        for i in 2..103 {
            h.insert(i);
        }

        h[1];
    }

    #[test]
    #[should_panic(expected = "Encountered attempt to remove a value not in the hashset")]
    fn invalid_remove_empty() {
        let mut h = Hashset::new();
        h.remove(&String::from("This doesn't exist."));
    }

    #[test]
    #[should_panic(expected = "Encountered attempt to remove a value not in the hashset")]
    fn invalid_remove_not_empty() {
        let mut h = Hashset::new();
        h.insert(String::from("Thing"));
        h.remove(&String::from("This doesn't exist."));
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
    /// # Panics
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
    /// # Panics
    /// This function will panic if an attempt is made to remove an item that
    /// is not present in the hashset.
    // "Deletion in an open addressing scheme can get ugly, since removing one
    // element might break a chain of insertions, making some elements
    // inaccessible. We have no alternative but to reinsert all the items in
    // the run following the new hole."
    pub fn remove(&mut self, value: &T) -> T {

        if !self.contains(&value) {
            panic!("Encountered attempt to remove a value not in the hashset");
        }

        let i = self.get_index(value, false).unwrap();
        let ret = self.remove_no_reinsert(i);
        self.count -= 1;

        // Find all items in the run after the removed item (if there are any).
        let mut run = Vec::new();
        let mut j = i;
        while self.vector[(j + 1) % self.capacity].is_some() {
            j = (j + 1) % self.capacity;
            run.push(self.remove_no_reinsert(j));
        }
        // Reinsert all of the items from the run.
        for _ in 0..run.len() {
            self.insert_internal(run.pop().unwrap(), false);
        }


        ret
    }

    // Remove an element by index and return ownership.
    // The calling function must check for a broken run and handle it appropriately.
    fn remove_no_reinsert(&mut self, i: usize) -> T {
        // Push none and swap it with the removed element to preserve the positions
        // of the elements in the vector.
        self.vector.push(None);
        self.vector.swap_remove(i).unwrap()
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
        let mut found_value;

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

/// Allows hashset elements to be accessed with the "[]" syntax.
/// #Panics
/// This will panic if the requested element is not in the set.
// Undocumented panic: May panic if there is a bug in contains() or get_index().
impl<T> ops::Index<T> for Hashset<T>
    where T: Hash + Eq
{
    type Output = T;

    fn index(&self, value: T) -> &Self::Output {
        if !self.contains(&value) {
            panic!("Encountered attempt to look up a value not in the hashset");
        }
        
        match self.get_index(&value, false) {
            Some(index) => {
                match self.vector[index].as_ref() {
                    Some(reference) => {
                        reference
                    },
                    None => {
                        panic!("If get_index(&value, false) succeeds, a valid \
                            index into the vector should be returned.");
                    }
                }
            },
            None => {
                panic!("Hashset<T>.get_index(&value, false) should always succeed \
                    if the hashset contains the given value.");
            }
        }
    }
}

// TODO: Write a key value pair struct that implements the Hash trait, causing
// it to be hashed by the key only, regardless of the value. Use this struct
// with the hashset to implement a hashmap.
// It will probably be necessary to implement the Eq(?) trait as well and have
// it test equality by checking the equality of the keys only.
