use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Deref;

/// A wrapper around an array-like type that is guaranteed to always be sorted.
#[repr(transparent)]
pub struct Sorted<A: ?Sized, T: Ord> {
    _phantom: PhantomData<fn() -> T>,
    array: A,
}

impl<A: BorrowMut<[T]>, T: Ord> Sorted<A, T> {
    /// Create a new sorted array.
    ///
    /// The underlying array `array` will be sorted according to the ordering of `T`.
    pub fn new(mut array: A) -> Sorted<A, T> {
        array.borrow_mut().sort();
        Sorted {
            array,
            _phantom: PhantomData,
        }
    }
}

impl<A: Borrow<[T]>, T: Ord> Sorted<A, T> {
    /// Create a new sorted array from already sorted data.
    ///
    /// The underlying array `array` will not be sorted. If the array is not already sorted, [`Sorted::new`]
    /// should be called instead.
    pub fn new_assume_sorted(array: A) -> Sorted<A, T> {
        Sorted {
            array,
            _phantom: PhantomData,
        }
    }
}

impl<A: BorrowMut<[T]> + ?Sized, T: Ord> Sorted<A, T> {
    /// Get a sorted array of a mutable slice of the underlying data.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_slice_mut(&mut self) -> &mut Sorted<[T], T> {
        let slice: &mut [T] = self.array.borrow_mut();
        unsafe { &mut *(slice as *mut [T] as *mut Sorted<[T], T>) }
    }
}

impl<A: BorrowMut<[T]> + Extend<T> + ?Sized, T: Ord> Sorted<A, T> {
    /// Insert an item into the array, maintaining the sorted order.
    pub fn insert(&mut self, item: T) {
        self.extend(std::iter::once(item));
    }
}

impl<A: Borrow<[T]> + ?Sized, T: Ord> Sorted<A, T> {
    /// Get a sorted array of an immutable slice of the underlying data.
    pub fn to_slice(&self) -> &Sorted<[T], T> {
        let slice: &[T] = self.array.borrow();
        unsafe { &*(slice as *const [T] as *const Sorted<[T], T>) }
    }

    /// Get a slice from the underlying array.
    pub fn as_slice(&self) -> &[T] {
        self.array.borrow()
    }

    /// Iterate through the underlying array.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Search through the array with a comparator function.
    ///
    /// The comparator function should implement an order consistent with the ordering of `T`,
    /// returning an order code that indicates whether its argument is `Less`, `Equal` or `Greater`
    /// the desired target.
    ///
    /// If the value is found then `Some` is returned, containing a reference to the matching
    /// element. If there are multiple matches, then any one of the matches could be returned. If the
    /// value is not found then `None` is returned.
    pub fn find_by<F>(&self, f: F) -> Option<&T>
    where
        F: FnMut(&T) -> Ordering,
    {
        self.as_slice()
            .binary_search_by(f)
            .ok()
            .map(|i| &self.array.borrow()[i])
    }

    /// Search through the array with a key extraction function.
    ///
    /// The key should implement an order consistent with the ordering of `T`, specifically:
    ///
    /// * If `x == y` then `key_of(x) == key_of(y)`.
    /// * If `key_of(x) < key_of(y)`, then `x < y`.
    ///
    /// If the value is found then `Some` is returned, containing a reference to the matching
    /// element. If there are multiple matches, then any one of the matches could be returned. If the
    /// value is not found then `None` is returned.
    pub fn find_by_key<K, F>(&self, key: &K, mut key_of: F) -> Option<&T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.find_by(|item| Ord::cmp(&key_of(item), key))
    }

    /// Search through the array for a range of items with a comparator function.
    ///
    /// The comparator function should implement an order consistent with the ordering of `T`,
    /// returning an order code that indicates whether its argument is `Less`, `Equal` or `Greater`
    /// the desired target.
    ///
    /// The returned slice of items that are considered `Equal` by the comparison function is
    /// returned.
    pub fn find_range_by<F>(&self, mut f: F) -> &[T]
    where
        F: FnMut(&T) -> Ordering,
    {
        let lo = self
            .as_slice()
            .binary_search_by(|item| f(item).then(Ordering::Greater))
            .unwrap_or_else(|i| i);
        let hi = self
            .as_slice()
            .binary_search_by(|item| f(item).then(Ordering::Less))
            .unwrap_or_else(|i| i);
        &self.as_slice()[lo..hi]
    }

    /// Search through the array for a range of items with a key extraction function.
    ///
    /// The key should implement an order consistent with the ordering of `T`, specifically:
    ///
    /// * If `x == y` then `key_of(x) == key_of(y)`.
    /// * If `key_of(x) < key_of(y)`, then `x < y`.
    ///
    /// The returned slice of items that are considered `Equal` by comparison of the keys is
    /// returned.
    pub fn find_range_by_key<K, F>(&self, key: &K, mut key_of: F) -> &[T]
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.find_range_by(|item| Ord::cmp(&key_of(item), key))
    }
}

impl<A: Clone, T: Ord> Clone for Sorted<A, T> {
    fn clone(&self) -> Self {
        Sorted {
            _phantom: PhantomData,
            array: self.array.clone(),
        }
    }
}

impl<A: Debug, T: Ord> Debug for Sorted<A, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Sorted").field(&self.array).finish()
    }
}

impl<A: Borrow<[T]> + ?Sized, T: Ord> Deref for Sorted<A, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'a, A: Borrow<[T]> + ?Sized, T: Ord> IntoIterator for &'a Sorted<A, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.iter()
    }
}

impl<'a, A: Borrow<[T]> + ?Sized, T: Ord> IntoIterator for &'a mut Sorted<A, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.iter()
    }
}

impl<A: BorrowMut<[T]> + Extend<T> + ?Sized, T: Ord> Extend<T> for Sorted<A, T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.array.extend(iter);
        self.array.borrow_mut().sort();
    }
}

impl<A: BorrowMut<[T]> + FromIterator<T>, T: Ord> FromIterator<T> for Sorted<A, T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let array = A::from_iter(iter);
        Sorted::new(array)
    }
}
