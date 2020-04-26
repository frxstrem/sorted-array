#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{
        Deref, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
};

use crate::{comparator::*, utils::*, weak_borrow::*};

#[repr(transparent)]
pub struct SortedSlice<T, C: Comparator<T> = OrdComparator> {
    _comparator: PhantomData<fn() -> C>,
    slice: [T],
}

impl<T, C: Comparator<T>> SortedSlice<T, C> {
    pub fn as_slice(&self) -> &[T] {
        &self.slice
    }

    pub fn contains<U>(&self, item: &U) -> bool
    where
        T: WeakBorrow<U>,
        C: Comparator<U>,
    {
        self.find(item).is_some()
    }

    pub fn find<U>(&self, item: &U) -> Option<&T>
    where
        T: WeakBorrow<U>,
        C: Comparator<U>,
    {
        self.find_by(|it| C::compare(it.weak_borrow(), item))
    }

    pub fn find_by<F>(&self, f: F) -> Option<&T>
    where
        F: FnMut(&T) -> Ordering,
    {
        self.slice
            .binary_search_by(f)
            .ok()
            .map(|index| &self[index])
    }

    pub fn find_by_key<K, F>(&self, key: &K, mut f: F) -> Option<&T>
    where
        F: FnMut(&T) -> K,
        C: Comparator<K>,
    {
        self.find_by(|it| C::compare(&f(it), key))
    }

    pub fn find_range<U>(&self, item: &U) -> &SortedSlice<T, C>
    where
        T: WeakBorrow<U>,
        C: Comparator<U>,
    {
        self.find_range_by(|it| C::compare(it.weak_borrow(), item))
    }

    pub fn find_range_by<F>(&self, f: F) -> &SortedSlice<T, C>
    where
        F: FnMut(&T) -> Ordering,
    {
        let (low, high) = binary_search_range(&self.slice, f);
        &self[low..high]
    }

    pub fn find_range_by_key<K, F>(&self, key: &K, mut f: F) -> &SortedSlice<T, C>
    where
        F: FnMut(&T) -> K,
        C: Comparator<K>,
    {
        self.find_range_by(|it| C::compare(&f(it), key))
    }

    pub fn find_mut_range<U>(&mut self, item: &U) -> &mut SortedSlice<T, C>
    where
        T: WeakBorrow<U>,
        C: Comparator<U>,
    {
        self.find_mut_range_by(|it| C::compare(it.weak_borrow(), item))
    }

    pub fn find_mut_range_by<F>(&mut self, f: F) -> &mut SortedSlice<T, C>
    where
        F: FnMut(&T) -> Ordering,
    {
        let (low, high) = binary_search_range(&self.slice, f);
        &mut self[low..high]
    }

    pub fn find_mut_range_by_key<K, F>(&mut self, key: &K, mut f: F) -> &mut SortedSlice<T, C>
    where
        F: FnMut(&T) -> K,
        C: Comparator<K>,
    {
        self.find_mut_range_by(|it| C::compare(&f(it), key))
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: Clone, C: Comparator<T>> Clone for Box<SortedSlice<T, C>> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T: Debug, C: Comparator<T>> Debug for SortedSlice<T, C> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.slice, fmt)
    }
}

impl<T, C: Comparator<T>> Deref for SortedSlice<T, C> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.slice
    }
}

impl<T, C: Comparator<T>> Borrow<[T]> for SortedSlice<T, C> {
    fn borrow(&self) -> &[T] {
        &self.slice
    }
}

impl<T, C: Comparator<T>> AsRef<[T]> for SortedSlice<T, C> {
    fn as_ref(&self) -> &[T] {
        &self.slice
    }
}

impl<T, C: Comparator<T>> Index<usize> for SortedSlice<T, C> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.slice[index]
    }
}

impl<T, C: Comparator<T>> Index<Range<usize>> for SortedSlice<T, C> {
    type Output = SortedSlice<T, C>;

    fn index(&self, range: Range<usize>) -> &SortedSlice<T, C> {
        from_slice_unchecked(&self.slice[range])
    }
}

impl<T, C: Comparator<T>> IndexMut<Range<usize>> for SortedSlice<T, C> {
    fn index_mut(&mut self, range: Range<usize>) -> &mut SortedSlice<T, C> {
        from_mut_slice_unchecked(&mut self.slice[range])
    }
}

impl<T, C: Comparator<T>> Index<RangeFrom<usize>> for SortedSlice<T, C> {
    type Output = SortedSlice<T, C>;

    fn index(&self, range: RangeFrom<usize>) -> &SortedSlice<T, C> {
        from_slice_unchecked(&self.slice[range])
    }
}

impl<T, C: Comparator<T>> IndexMut<RangeFrom<usize>> for SortedSlice<T, C> {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut SortedSlice<T, C> {
        from_mut_slice_unchecked(&mut self.slice[range])
    }
}

impl<T, C: Comparator<T>> Index<RangeTo<usize>> for SortedSlice<T, C> {
    type Output = SortedSlice<T, C>;

    fn index(&self, range: RangeTo<usize>) -> &SortedSlice<T, C> {
        from_slice_unchecked(&self.slice[range])
    }
}

impl<T, C: Comparator<T>> IndexMut<RangeTo<usize>> for SortedSlice<T, C> {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut SortedSlice<T, C> {
        from_mut_slice_unchecked(&mut self.slice[range])
    }
}

impl<T, C: Comparator<T>> Index<RangeFull> for SortedSlice<T, C> {
    type Output = SortedSlice<T, C>;

    fn index(&self, range: RangeFull) -> &SortedSlice<T, C> {
        from_slice_unchecked(&self.slice[range])
    }
}

impl<T, C: Comparator<T>> IndexMut<RangeFull> for SortedSlice<T, C> {
    fn index_mut(&mut self, range: RangeFull) -> &mut SortedSlice<T, C> {
        from_mut_slice_unchecked(&mut self.slice[range])
    }
}

impl<T, C: Comparator<T>> Index<RangeInclusive<usize>> for SortedSlice<T, C> {
    type Output = SortedSlice<T, C>;

    fn index(&self, range: RangeInclusive<usize>) -> &SortedSlice<T, C> {
        from_slice_unchecked(&self.slice[range])
    }
}

impl<T, C: Comparator<T>> IndexMut<RangeInclusive<usize>> for SortedSlice<T, C> {
    fn index_mut(&mut self, range: RangeInclusive<usize>) -> &mut SortedSlice<T, C> {
        from_mut_slice_unchecked(&mut self.slice[range])
    }
}

impl<T, C: Comparator<T>> Index<RangeToInclusive<usize>> for SortedSlice<T, C> {
    type Output = SortedSlice<T, C>;

    fn index(&self, range: RangeToInclusive<usize>) -> &SortedSlice<T, C> {
        from_slice_unchecked(&self.slice[range])
    }
}

impl<T, C: Comparator<T>> IndexMut<RangeToInclusive<usize>> for SortedSlice<T, C> {
    fn index_mut(&mut self, range: RangeToInclusive<usize>) -> &mut SortedSlice<T, C> {
        from_mut_slice_unchecked(&mut self.slice[range])
    }
}

impl<'a, T, C: Comparator<T>> IntoIterator for &'a SortedSlice<T, C> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.iter()
    }
}

impl<'a, T, C: Comparator<T>> IntoIterator for &'a mut SortedSlice<T, C> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.iter()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T, C: Comparator<T>> core::iter::FromIterator<T> for Box<SortedSlice<T, C>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Box<SortedSlice<T, C>> {
        sort_boxed_slice(core::iter::FromIterator::from_iter(iter))
    }
}

pub(crate) fn from_slice_unchecked<T, C: Comparator<T>>(slice: &[T]) -> &SortedSlice<T, C> {
    unsafe { &*(slice as *const [T] as *const SortedSlice<T, C>) }
}

pub(crate) fn from_mut_slice_unchecked<T, C: Comparator<T>>(
    slice: &mut [T],
) -> &mut SortedSlice<T, C> {
    unsafe { &mut *(slice as *mut [T] as *mut SortedSlice<T, C>) }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub(crate) fn from_boxed_slice_unchecked<T, C: Comparator<T>>(
    slice: Box<[T]>,
) -> Box<SortedSlice<T, C>> {
    unsafe { Box::from_raw(Box::into_raw(slice) as *mut SortedSlice<T, C>) }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn sort_mut_slice<T, C: Comparator<T>>(slice: &mut [T]) -> &mut SortedSlice<T, C> {
    slice.sort_by(C::compare);
    from_mut_slice_unchecked(slice)
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn sort_boxed_slice<T, C: Comparator<T>>(mut slice: Box<[T]>) -> Box<SortedSlice<T, C>> {
    slice.sort_by(C::compare);
    from_boxed_slice_unchecked(slice)
}

pub fn sort_mut_slice_unstable<T, C: Comparator<T>>(slice: &mut [T]) -> &mut SortedSlice<T, C> {
    slice.sort_unstable_by(C::compare);
    from_mut_slice_unchecked(slice)
}
