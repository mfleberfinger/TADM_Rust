/* TODO: Rust doesn't allow dynamic sizing of its standard arrays.
 * Apparently, implementing a dynamic array requires "advanced" Rust
 * knowledge, including unsafe code (https://doc.rust-lang.org/nomicon/vec/vec.html).
 * This doesn't seem like a good place to start.
#[cfg(test)]
mod dynamic_array_tests {

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
