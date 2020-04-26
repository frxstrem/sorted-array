use alloc::{boxed::Box, vec::Vec};
use core::{
    borrow::{Borrow, BorrowMut},
    fmt::{self, Debug},
    iter::FromIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::{
    comparator::{Comparator, OrdComparator},
    sorted_slice::{self, SortedSlice},
    weak_borrow::WeakBorrow,
};

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[repr(transparent)]
pub struct SortedVec<T, C: Comparator<T> = OrdComparator> {
    _comparator: PhantomData<fn() -> C>,
    vec: Vec<T>,
}

impl<T, C: Comparator<T>> SortedVec<T, C> {
    pub fn new() -> Self {
        SortedVec {
            _comparator: PhantomData,
            vec: Vec::new(),
        }
    }

    pub fn as_sorted_slice(&self) -> &SortedSlice<T, C> {
        sorted_slice::from_slice_unchecked(&self.vec)
    }

    pub fn as_mut_sorted_slice(&mut self) -> &mut SortedSlice<T, C> {
        sorted_slice::from_mut_slice_unchecked(&mut self.vec)
    }

    pub fn into_boxed_sorted_slice(self) -> Box<SortedSlice<T, C>> {
        sorted_slice::from_boxed_slice_unchecked(self.vec.into_boxed_slice())
    }

    pub fn insert(&mut self, item: T) {
        let insert_at = self
            .vec
            .binary_search_by(|it| C::compare(it, &item))
            .map(|i| i + 1)
            .unwrap_or_else(|i| i);

        self.vec.insert(insert_at, item);
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.vec.remove(index)
    }

    pub fn remove_item<U>(&mut self, item: &U) -> Option<T>
    where
        T: WeakBorrow<U>,
        C: Comparator<U>,
    {
        self.vec
            .binary_search_by(|it| C::compare(it.weak_borrow(), item))
            .ok()
            .map(|index| self.vec.remove(index))
    }
}

impl<T: Clone, C: Comparator<T>> Clone for SortedVec<T, C> {
    fn clone(&self) -> Self {
        SortedVec {
            _comparator: PhantomData,
            vec: self.vec.clone(),
        }
    }
}

impl<T, C: Comparator<T>> Default for SortedVec<T, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug, C: Comparator<T>> Debug for SortedVec<T, C> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.vec, fmt)
    }
}

impl<T, C: Comparator<T>> Deref for SortedVec<T, C> {
    type Target = SortedSlice<T, C>;

    fn deref(&self) -> &SortedSlice<T, C> {
        self.as_sorted_slice()
    }
}

impl<T, C: Comparator<T>> DerefMut for SortedVec<T, C> {
    fn deref_mut(&mut self) -> &mut SortedSlice<T, C> {
        self.as_mut_sorted_slice()
    }
}

impl<T, C: Comparator<T>> Borrow<[T]> for SortedVec<T, C> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, C: Comparator<T>> Borrow<SortedSlice<T, C>> for SortedVec<T, C> {
    fn borrow(&self) -> &SortedSlice<T, C> {
        self.as_sorted_slice()
    }
}

impl<T, C: Comparator<T>> BorrowMut<SortedSlice<T, C>> for SortedVec<T, C> {
    fn borrow_mut(&mut self) -> &mut SortedSlice<T, C> {
        self.as_mut_sorted_slice()
    }
}

impl<T, C: Comparator<T>> AsRef<[T]> for SortedVec<T, C> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, C: Comparator<T>> AsRef<SortedSlice<T, C>> for SortedVec<T, C> {
    fn as_ref(&self) -> &SortedSlice<T, C> {
        self.as_sorted_slice()
    }
}

impl<T, C: Comparator<T>> AsMut<SortedSlice<T, C>> for SortedVec<T, C> {
    fn as_mut(&mut self) -> &mut SortedSlice<T, C> {
        self.as_mut_sorted_slice()
    }
}

impl<T, C: Comparator<T>, Idx> Index<Idx> for SortedVec<T, C>
where
    SortedSlice<T, C>: Index<Idx>,
{
    type Output = <SortedSlice<T, C> as Index<Idx>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.as_sorted_slice()[index]
    }
}

impl<T, C: Comparator<T>, Idx> IndexMut<Idx> for SortedVec<T, C>
where
    SortedSlice<T, C>: IndexMut<Idx>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.as_mut_sorted_slice()[index]
    }
}

impl<T, C: Comparator<T>> IntoIterator for SortedVec<T, C> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, T, C: Comparator<T>> IntoIterator for &'a SortedVec<T, C> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<'a, T, C: Comparator<T>> IntoIterator for &'a mut SortedVec<T, C> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<T, C: Comparator<T>> FromIterator<T> for SortedVec<T, C> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> SortedVec<T, C> {
        sort_vec(FromIterator::from_iter(iter))
    }
}

impl<T, C: Comparator<T>> Extend<T> for SortedVec<T, C> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.vec.extend(iter);
        self.vec.sort_by(C::compare);
    }
}

pub(crate) fn from_vec_unchecked<T, C: Comparator<T>>(vec: Vec<T>) -> SortedVec<T, C> {
    SortedVec {
        _comparator: PhantomData,
        vec,
    }
}

pub fn sort_vec<T, C: Comparator<T>>(mut vec: Vec<T>) -> SortedVec<T, C> {
    vec.sort_by(C::compare);
    from_vec_unchecked(vec)
}
