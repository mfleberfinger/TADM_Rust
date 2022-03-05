use std::ops;

#[cfg(test)]
mod arena_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn remove_from_empty() {
        let mut arena: Arena<i32> = Arena::new();
        arena.remove(0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_when_empty() {
        let arena: Arena<i32> = Arena::new();
        arena[0];
    }

    #[test]
    fn insert_and_remove() {
        let mut arena = Arena::new();
        arena.insert(0);
        assert_eq!(arena.remove(0), Some(0));
        assert!(arena.remove(0).is_none());

        let mut indices = Vec::new();
        for i in 0..100 {
            indices.push(arena.insert(i));
        }
        let mut j = 0;
        for i in indices {
            assert_eq!(arena.remove(i), Some(j));
            j += 1;
        }
    }

    #[test]
    fn insert_and_index() {
        let mut arena = Arena::new();
        arena.insert(0);
        assert_eq!(arena[0], Some(0));

        let mut indices = Vec::new();
        for i in 0..100 {
            indices.push(arena.insert(i));
        }
        let mut j = 0;
        for i in indices {
            assert_eq!(arena[i], Some(j));
            j += 1;
        }
    }

    #[test]
    #[should_panic(expected = "Index out of bounds. Cannot borrow.")]
    fn borrow_mutable_out_of_bounds() {
        let mut arena: Arena<i32> = Arena::new();
        arena.borrow_mutable(0);
    }

    #[test]
    fn mutate_stored_data() {
        let mut arena = Arena::new();
        let index = arena.insert(String::from("Some data"));
        let mutable = arena.borrow_mutable(0);
        *(mutable.unwrap()) = String::from("Some other data");
        assert_eq!(arena[index].as_ref().unwrap(), "Some other data");
    }
}

/// An "arena allocator." Provides convenient storage space to hold the nodes in
/// linked data structures (e.g. graphs, trees) without running afoul of Rust's
/// borrowing rules.
/// Can be indexed into using the [] operator.
pub struct Arena<T> {
    // Indices that already exist in the vector but do not currently hold data.
    available: Vec<usize>,
    // Storage space maintained for the user of this struct.
    storage: Vec<Option<T>>
}

impl<T> Arena<T> {
    /// Create an empty Arena.
    pub fn new() -> Arena<T> {
        Arena {
            available: Vec::new(),
            storage: Vec::new()
        }
    }

    /// Add an element to the arena and return its index, which will be used to
    /// reference that element going foward.
    pub fn insert(&mut self, data: T) -> usize {
        let index;
        // If we already have an empty slot, use it. Otherwise, add a new one.
        if let Some(i) = self.available.pop() {
            let _ = self.storage[i].insert(data);
            index = i;
        }
        else {
            self.storage.push(Some(data));
            index = self.storage.len() - 1;
        }
        index
    }

    /// Remove and return the item at a given index or None if that index is
    /// allocated but empty.
    /// # Panics
    /// Panic if the index was never allocated (index out of bounds).
    pub fn remove(&mut self, index: usize) -> Option<T> {
        // Mark the emptied slot as available.
        if let Some(item) = self.storage[index].take() {
            self.available.push(index);
            Some(item)
        }
        else {
            None
        }

    }

    /// Returns a mutable reference to the value at the given index.
    /// Returns None if the index was allocated but does not currently hold
    /// anything.
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn borrow_mutable(&mut self, index: usize) -> Option<&mut T> {
        match self.storage.get_mut(index) {
            Some(reference) => reference.as_mut(),
            None => { panic!("Index out of bounds. Cannot borrow.") }
        }
    }
}

/// Returns the stored value at the given index. Returns None if the index was
/// allocated but does not currently hold anything.
/// # Panics
/// Panic if the index was never allocated (index out of bounds).
impl<T> ops::Index<usize> for Arena<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.storage[index]
    }
}
