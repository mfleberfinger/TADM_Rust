use std::cmp::Ord;

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

/// Returns false if any element in v1 equals any element in v2. Otherwise,
/// returns true.
/// The arguments need not be sets (they can contain duplicated elements).
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
// while still being able to indicate that calculating end resulted in a negative
// number.
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
