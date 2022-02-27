use std::cmp::Ord;
use crate::heap::Heap;
use crate::hashset::Hashset;

#[cfg(test)]
mod test_helpers {
    // Returns a vector of strings.
    pub fn get_strings() -> Vec<String> {
        vec![
            String::from("spork"),
            String::from("pork"),
            String::from("fork"),
            String::from("spoon"),
            String::from("knife"),
            String::from("serial"),
            String::from("cereal"),
            String::from("senile"),
            String::from("serotonin"),
            String::from("animal"),
            String::from("a"),
            String::from("xylophone"),
            String::from("mississippi"),
            String::from("zebra"),
            String::from("giraffe"),
            String::from(""),                           // Empty string
            String::from("abcdefghijklmnopqrstuvwxyz")  // Now I know my ABCs.
        ]
    }

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
mod square_root_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Cannot calculate the square root of a negative number.")]
    fn negative() {
        square_root(-1.0, 0.0);
    }

    #[test]
    fn integers_greater_than_one() {
        for i in 0..100000 {
            let x = f64::from(i);
            // We do a direct equality comparison of floats here but a similar
            // assertion does succeed with the standard square root function:
            // assert_eq!((x.powf(2.0)).sqrt(), x);
            assert_eq!(square_root(x.powf(2.0), 0.0), x);
        }
    }

    /* Runs for a very long time. May never terminate.
    #[test]
    fn greater_than_one_with_decimal_points() {
        for i in 100..110 {
            for j in 0..1000 {
                let x = f64::from(i) + (1.0 / f64::from(j));
                // We do a direct equality comparison of floats here but a similar
                // assertion does succeed with the standard square root function:
                // assert_eq!((x.powf(2.0)).sqrt(), x);
                assert_eq!(square_root(x.powf(2.0), 0.0), x);
            }
        }
    }
    */

    /* Runs for a very long time. May never terminate.
    #[test]
    fn between_zero_and_one() {
        //let epsilon = 0.1;
        for i in 1..1000 {
            let x = 1.0 / f64::from(i);
            //assert!((square_root(x.powf(2.0), epsilon) - x).abs() <= epsilon);
            assert_eq!(square_root(x.powf(2.0), 0.0), x);
            //assert_eq!((x.powf(2.0)).sqrt(), x);
        }
    }
    */
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

#[cfg(test)]
mod quicksort_tests {
    use super::*;

    #[test]
    fn sort() {
        // Ascending.
        let mut v = vec![5, 4, 3, 1, 11, 10];
        quicksort(&mut v, false);
        assert_eq!(v.len(), 6);
        test_helpers::assert_sorted(v.iter(), true);

        // Descending.
        let mut v = vec![5, 4, 3, 1, 11, 10];
        quicksort(&mut v, true);
        assert_eq!(v.len(), 6);
        test_helpers::assert_sorted(v.iter(), false);
    }

    #[test]
    fn sort_empty() {
        // Ascending.
        let mut v: Vec<i32> = Vec::new();
        quicksort(&mut v, false);
        assert_eq!(v.len(), 0);
        test_helpers::assert_sorted(v.iter(), true);

        // Descending.
        let mut v: Vec<i32> = Vec::new();
        quicksort(&mut v, true);
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
        quicksort(&mut v, true);
        assert_eq!(v.len(), len);
        test_helpers::assert_sorted(v.iter(), false);
    }
}

#[cfg(test)]
mod distribution_sort_tests {
    use super::*;

    #[test]
    fn sort() {
        let mut v = test_helpers::get_strings();
        let len = v.len();
        v = distribution_sort(v);
        assert_eq!(v.len(), len);
        test_helpers::assert_sorted(v.iter(), true);
    }

    #[test]
    fn sort_empty() {
        let mut v: Vec<String> = Vec::new();
        v = distribution_sort(v);
        assert_eq!(v.len(), 0);
        test_helpers::assert_sorted(v.iter(), true);
    }

    #[test]
    #[should_panic(expected = "This function does not support duplicated strings.")]
    fn duplicates() {
        let v = vec![
            String::from("string"),
            String::from("and"),
            String::from("string"),
            String::from("another string")];
        distribution_sort(v);
    }

    #[test]
    #[should_panic(expected = "Input strings may only contain lowercase characters a through z.")]
    fn unsupported_characters() {
        let v = vec![
            String::from("string"),
            String::from("string?")];
        distribution_sort(v);
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

/// Returns the square root of the argument, x.
/// The x argument must be positive.
/// # Panics
/// This function will panic if x is negative.
/// # Bug
/// This function will run for a very long time (or maybe forever) for certain
/// inputs. It doesn't seem to have any problems running for integer inputs.
/// For example, it returns the correct results for all integers from 1 to 100,000.
/// This probably has something to do with floating point precision.
// We will essentially perform a binary search on the numbers between 0 and x
// for a number m such that m * m = x.
pub fn square_root(x: f64, epsilon: f64) -> f64 {
    if x < 0.0 {
        // We're not very imaginative.
       panic!("Cannot calculate the square root of a negative number.");
    }

    if x == 0.0 {
        // The square root of 0 is 0.
       return 0.0;
    }

    let mut l;
    let mut h;
    let mut m;

    if x >= 1.0 {
        // x is at least 1. The square root of x is between 1 and x.
        l = 1.0;
        h = x;
        m = (l + h) / 2.0;
    }
    else {
        // x is between 0 and 1. The square root of x is between x and 1.
        l = x;
        h = 1.0;
        m = (l + h) / 2.0;
    }

    while (m * m < x - epsilon) || (m * m > x + epsilon) {
        if m * m < x - epsilon {
            l = m;
        }
        else {
            h = m;
        }
        m = (l + h) / 2.0;
    }
    m
}

/*
fn square_root_internal(x: f64, l: f64, h: f64) -> f64 {
    let m = (l + h) / 2.0;

    if m * m < x {
        square_root_internal(x, m, h)
    }
    else if m * m > x {
        square_root_internal(x, l, m)
    }
    else {
        m
    }
}
*/

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
/// This function consumes the given vector and returns a new vector.
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

/// Sorts the given vector in ascending or descending order.
pub fn quicksort<T>(vector: &mut Vec<T>, sort_descending: bool)
    where T: PartialOrd
{
    quicksort_internal(&mut vector[..], sort_descending);
}

fn quicksort_internal<T>(slice: &mut [T], sort_descending: bool)
    where T: PartialOrd
{
    if slice.len() > 1 {
        let mut pivot = slice.len() / 2;
        // Move everything to the correct side of the pivot.
        partition(slice, sort_descending, &mut pivot);
        // Sort everything to the left of the pivot.
        quicksort_internal(&mut slice[..pivot], sort_descending);
        // Sort everything to the right of the pivot, if the pivot is not the last element.
        if pivot + 2 < slice.len() {
            quicksort_internal(&mut slice[(pivot + 1)..], sort_descending);
        }
    }
}

// Move all elements that come after the pivot in sorted order to the right of
// the pivot and move all elements that come before the pivot to the left of
// the pivot. The pivot itself will move as needed to accomplish this within the
// bounds of the slice.
// Mutate the pivot parameter to keep track of the pivot element's index.
fn partition<T>(slice: &mut [T], sort_descending: bool, pivot: &mut usize)
    where T: PartialOrd
{
    let mut j = 0;

    for i in 0..slice.len() {
        if (slice[i] > slice[*pivot] && sort_descending)
            || (slice[i] < slice[*pivot] && !sort_descending) {

            slice.swap(i, j);

            // If we moved the pivot, change the pivot index to point to it again.
            if j == *pivot {
                *pivot = i;
            }

            j += 1;
        }
    }
    
    // All elements that should be to the left of the pivot are now to its left.
    // Swap the pivot with the first element that belongs on its right.
    slice.swap(j, *pivot);
    *pivot = j;
}


/// Given a vector of strings, sort that vector in ascending order by bucketing.
/// This function consumes the given vector and returns a new vector.
/// # Panics
/// This function will panic if there are duplicate strings in the vector. For
/// example, ["this", "is", "a", "vector"] will be sorted but
/// ["this", "this", "is", "a", "vector"] will cause a panic.
/// 
/// This function will panic if any string in the vector contains anything other
/// than lowercase alphabetical characters (a-z).
pub fn distribution_sort(vector: Vec<String>) -> Vec<String> {
    let mut set = Hashset::new();
    for s in &vector {
        // Panic if unsupported characters found in vector. 
        for c in s.chars() {
            if c < 'a' || c > 'z' {
                panic!("Input strings may only contain lowercase characters a through z.");
            }
        }
        
        // Panic if duplicates found in vector.
        if set.contains(&s) {
            panic!("This function does not support duplicated strings.");
        }
        else {
            set.insert(s);
        }
    }

    distribution_sort_internal(vector, 0)
}

fn distribution_sort_internal(mut vector: Vec<String>, i: usize) -> Vec<String> {

    // Create the buckets.
    let mut buckets: Vec<Vec<String>> = Vec::new();

    // Create a bucket for each letter and one more for blank/none.
    let number_of_buckets = 27;
    for _ in 0..number_of_buckets {
        buckets.push(Vec::new());
    }

    // Bucket by the ith letter.
    for s in vector.drain(..) {
        // Convert the ith character of this string to an integer between 0 and 25.
        let index = match s.chars().nth(i) {
            // We know we can unwrap because the calling function should only
            // allow the letters a to z to be passed to this function.
            Some(c) => c.to_digit(36).unwrap() - 9,
            // If this string is shorter than i letters, it comes before any
            // string with i or more letters in the same bucket.
            None => 0
        } as usize;
        // Put the string in the proper bucket.
        buckets[index].push(s);
    }

    // Sort each bucket.
    for j in 0..buckets.len() {
        if buckets[j].len() > 1 {
            // Need to give distribution_sort_internal ownership of the bucket,
            // then put the bucket back where it was when we get ownership back.
            // Maybe distribution_sort_internal could be rewritten to use a borrow
            // but if mergesort is any indication, that is not straightforward.
            let bucket = distribution_sort_internal(buckets.swap_remove(j), i + 1);
            buckets.push(bucket);
            buckets.swap(j, number_of_buckets - 1);
        }
    }

    // Combine the sorted buckets.
    let mut sorted = Vec::new();
    for b in buckets {
        for s in b {
            sorted.push(s);
        }
    }

    sorted
}
