use std::cmp::Ord;
use crate::heap::Heap;

#[cfg(test)]
mod test_helpers {
    // Asserts that the given iterator is sorted in ascending order if the
    // ascending argument is true. Otherwise, asserts that the iterator is in
    // descending order.
    // This function assumes that the iterator has not already been iterated.
    pub fn assert_sorted<T, U>(mut iterator: T, ascending: bool)
        where
        T: Iterator<Item = U>,
        U: PartialOrd
    {
        // If the iterator is empty or only contains one element, it's trivially
        // true that the iterator is sorted.
        if let Some(first) = iterator.next() {
            let mut prev = first;
            for i in iterator {
                assert!((ascending && i >= prev) || (!ascending && i <= prev));
                prev = i;
            }
        }
    }

    // Testing the test.
    #[test]
    fn assert_sorted_pass_test() {
        let v: Vec<i32> = Vec::new();
        assert_sorted(v.iter(), true);
        assert_sorted(v.iter(), false);
        let v = vec![1,2,3];
        assert_sorted(v.iter(), true);
        let v = vec![3,2,1];
        assert_sorted(v.iter(), false);
    }
    // Testing the test.
    #[test]
    #[should_panic]
    fn assert_sorted_fail_test_asc() {
        let v = vec![3,2,1];
        assert_sorted(v.iter(), true);
    }
    // Testing the test.
    #[test]
    #[should_panic]
    fn assert_sorted_fail_test_desc() {
        let v = vec![1,2,3];
        assert_sorted(v.iter(), false);
    }
}

#[cfg(test)]
mod disjoint_tests {
    use super::*;

    #[test]
    fn are_disjoint() {
        let v1 = vec!(1, 2, 3, 4, 5, 5);
        let v2 = vec!(6, 7, 8, 9, 10, 10);

        assert!(disjoint(&v1, &v2));
    }

    #[test]
    fn are_not_disjoint() {
        let v1 = vec!(1, 2, 3, 4, 4, 5);
        let v2 = vec!(5, 6, 7, 8, 9, 10);

        assert!(!disjoint(&v1, &v2));
    }

    #[test]
    fn one_empty() {
        let v1 = vec!(1, 2, 3, 4, 4, 5);
        let v2 = Vec::new();

        assert!(disjoint(&v1, &v2));
    }

    #[test]
    fn both_empty() {
        let v1: Vec<i32> = Vec::new();
        let v2: Vec<i32> = Vec::new();

        // The empty set is disjoint with itself. For two sets not to be disjoint,
        // those sets must contain a common element. The empty set contains no
        // elements, meaning it has no elements in common with itself.
        assert!(disjoint(&v1, &v2));
    }

}

#[cfg(test)]
mod binary_search_tests {
    use super::*;

    #[test]
    fn contains() {
        // Even length.
        let v = vec![0, 1, 2, 3, 4, 5];
        let i = binary_search(&v, &4).expect("Should return an index.");
        assert_eq!(i, 4);

        // Odd length.
        let v = vec![0, 1, 2, 3, 4];
        let i = binary_search(&v, &4).expect("Should return an index.");
        assert_eq!(i, 4);
    }

    #[test]
    fn contains_multiple() {
        let v = vec![0, 1, 2, 3, 4, 4, 4, 4, 5];
        let i = binary_search(&v, &4).expect("Should return an index.");

        // Any index to a 4 is acceptable.
        assert!(i <= 7 && i >= 4);
    }

    #[test]
    fn does_not_contain_high() {
        // Even length.
        let v = vec![0, 1, 2, 3, 4, 4, 4, 4, 5, 6];
        assert!(binary_search(&v, &700).is_none());

        // Odd length.
        let v = vec![0, 1, 2, 3, 4, 4, 4, 4, 5];
        assert!(binary_search(&v, &700).is_none());
    }

    #[test]
    fn does_not_contain_low() {
        // Even length.
        let v = vec![0, 1, 2, 3, 4, 4, 4, 4, 5, 6];
        assert!(binary_search(&v, &(-1)).is_none());

        // Odd length.
        let v = vec![0, 1, 2, 3, 4, 4, 4, 4, 5];
        assert!(binary_search(&v, &(-1)).is_none());
    }

    #[test]
    fn empty() {
        let v = Vec::new();
        assert!(binary_search(&v, &4).is_none());
    }
}

#[cfg(test)]
mod heapsort_tests {
    use super::*;

    #[test]
    fn sort() {
        // Ascending.
        let mut v = vec![5, 4, 3, 1, 11, 10];
        heapsort(&mut v, false);
        assert_eq!(v.len(), 6);
        test_helpers::assert_sorted(v.iter(), true);

        // Descending.
        let mut v = vec![5, 4, 3, 1, 11, 10];
        heapsort(&mut v, true);
        assert_eq!(v.len(), 6);
        test_helpers::assert_sorted(v.iter(), false);
    }

    #[test]
    fn sort_empty() {
        // Ascending.
        let mut v: Vec<i32> = Vec::new();
        heapsort(&mut v, false);
        assert_eq!(v.len(), 0);
        test_helpers::assert_sorted(v.iter(), true);

        // Descending.
        let mut v: Vec<i32> = Vec::new();
        heapsort(&mut v, true);
        assert_eq!(v.len(), 0);
        test_helpers::assert_sorted(v.iter(), false);
    }

    #[test]
    fn sort_big() {
        let mut v = Vec::new();
        let len = 100000;

        // Create a big vector that isn't already sorted.
        for i in 0..len {
            v.push(i % 100);
        }
        // sort descending.
        heapsort(&mut v, true);
        assert_eq!(v.len(), len);
        test_helpers::assert_sorted(v.iter(), false);
    }
}

#[cfg(test)]
mod mergesort_tests {
    use super::*;

    #[test]
    fn sort() {
        // Ascending.
        let mut v = vec![5, 4, 3, 1, 11, 10];
        v = mergesort(v, false);
        assert_eq!(v.len(), 6);
        test_helpers::assert_sorted(v.iter(), true);

        // Descending.
        let mut v = vec![5, 4, 3, 1, 11, 10];
        v = mergesort(v, true);
        assert_eq!(v.len(), 6);
        test_helpers::assert_sorted(v.iter(), false);
    }

    #[test]
    fn sort_empty() {
        // Ascending.
        let mut v: Vec<i32> = Vec::new();
        v = mergesort(v, false);
        assert_eq!(v.len(), 0);
        test_helpers::assert_sorted(v.iter(), true);

        // Descending.
        let mut v: Vec<i32> = Vec::new();
        v = mergesort(v, true);
        assert_eq!(v.len(), 0);
        test_helpers::assert_sorted(v.iter(), false);
    }

    #[test]
    fn sort_big() {
        let mut v = Vec::new();
        let len = 100000;

        // Create a big vector that isn't already sorted.
        for i in 0..len {
            v.push(i % 100);
        }
        // sort descending.
        v = mergesort(v, true);
        assert_eq!(v.len(), len);
        test_helpers::assert_sorted(v.iter(), false);
    }
}

/// Returns false if any element in v1 equals any element in v2. Otherwise,
/// returns true.
/// The arguments need not be sets (they can contain duplicated elements).
/// The items stored in v1 and v2 must implement the clone trait.
// For practice, this is implemented with sorting and searching instead of a hashset.
//  The O(nlogn) runtime of sorting will be our bottleneck, so sort the
//  smaller vector, then scan through the larger vector, running binary search
//  on the smaller one to determine whether each element of the smaller
//  vector is present in the larger vector.
// Several sorting algorithms will be implemented in this crate but, for now
//  (forever?), this particular function will use the built in sort.
pub fn disjoint<T>(v1: &Vec<T>, v2: &Vec<T>) -> bool
    where T: Eq + Ord + Clone
{
    let big;
    let mut small;

    if v1.len() <= v2.len() {
        big = v2;
        small = v1.clone();
    }
    else {
        big = v1;
        small = v2.clone();
    }
    small.sort();

    let mut disjoint = true;
    let mut i = 0;
    while i < big.len() && disjoint {
        disjoint = binary_search(&small, &big[i]).is_none();
        i += 1;
    }

    disjoint
}

/// If x exists in v, returns the index of x wrapped in Some().
/// Otherwise, returns None.
/// v must be sorted.
pub fn binary_search<T>(v: &Vec<T>, x: &T) -> Option<usize>
    where T: Eq + Ord
{
    binary_search_internal(v, x, 0, if v.len() > 0 { Some(v.len() - 1) } else { None })
}

// We use an Option<usize> to allow us to index into the largest possible Vec
// while still being able to indicate that calculating end would result in a
// negative number.
// In an abstract sense, we could run into the same problem with usize::MAX + 1
// but we probably can't actually have a vector that large (because it would
// require all of the computer's addressable memory and then some).
fn binary_search_internal<T>(
    v: &Vec<T>,
    x: &T,
    start: usize,
    end_option: Option<usize>)
    -> Option<usize>
    where T: Eq + Ord
{
    let end = match end_option {
        Some(i) => i,
        None => return None
    };

    if end < start {
        return None;
    }

    let middle = (start + end) / 2;

    if v[middle] == *x {
        Some(middle)
    }
    else if *x < v[middle] {
        binary_search_internal(v, x, start, if middle > 0 { Some(middle - 1) } else { None })
    }
    else {
        binary_search_internal(v, x, middle + 1, Some(end))
    }
}

/// Sorts the given vector in ascending or descending order.
pub fn heapsort<T>(v: &mut Vec<T>, sort_descending: bool)
    where T: PartialOrd
{
    // Use a max-heap for descending sort and a min-heap for ascending.
    let mut h = Heap::new(sort_descending);
    
    // Empty the vector into the heap.
    for _ in 0..v.len() {
        h.insert(v.pop().unwrap());
    }
    
    // Empty the heap back into the vector.
    // Like magic, the elements are now sorted!
    let mut element = h.extract();
    while element.is_some() {
        v.push(element.unwrap());
        element = h.extract();
    }
}

/// Sorts the given vector in ascending or descending order.
pub fn mergesort<T>(mut v: Vec<T>, sort_descending: bool) -> Vec<T>
    where T: PartialOrd
{
    if v.len() > 1 {
        let v1: Vec<T> = v.drain(0..(v.len() / 2)).collect();
        let v2: Vec<T> = v.drain(0..v.len()).collect();
        merge(
            mergesort(v1, sort_descending),
            mergesort(v2, sort_descending),
            sort_descending
            )
    }
    else {
        v
    }
}

fn merge<T>(mut v1: Vec<T>, mut v2: Vec<T>, sort_descending: bool) -> Vec<T>
    where T: PartialOrd
{
    let mut merged = Vec::new();

    // We will need to work backwards through v1 and v2 so we can efficiently
    // move values out of of them without changing the order.
    while !v1.is_empty() || !v2.is_empty() {
        if v1.is_empty() {
            merged.push(v2.pop().unwrap());
        }
        else if v2.is_empty() {
            merged.push(v1.pop().unwrap());
        }
        else if v1[v1.len() - 1] > v2[v2.len() - 1] {
            if sort_descending {
                merged.push(v2.pop().unwrap());
            }
            else {
                merged.push(v1.pop().unwrap());
            }
        }
        else {
            if sort_descending {
                merged.push(v1.pop().unwrap());
            }
            else {
                merged.push(v2.pop().unwrap());
            }
        }
    }

    // We reversed the order of the vectors while merging, so we need to reverse
    // it again. It's not clear if there is an efficient way to avoid reversing
    // the vector in the first place.
    merged.reverse();
    merged
}
