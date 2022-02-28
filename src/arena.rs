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
        let mut arena: Arena<i32> = Arena::new();
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
            self.storage[i].insert(data);
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
