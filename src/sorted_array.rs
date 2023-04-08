use core::{
    borrow::{Borrow, BorrowMut},
    fmt::{self, Debug},
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::{
    comparator::{Comparator, OrdComparator},
    sorted_slice::{self, SortedSlice},
};

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[repr(transparent)]
pub struct SortedArray<T, const N: usize, C: Comparator<T> = OrdComparator> {
    _comparator: PhantomData<fn() -> C>,
    array: [T; N],
}

impl<T, const N: usize, C: Comparator<T>> SortedArray<T, N, C> {
    pub fn as_sorted_slice(&self) -> &SortedSlice<T, C> {
        sorted_slice::from_slice_unchecked(&self.array)
    }

    pub fn as_mut_sorted_slice(&mut self) -> &mut SortedSlice<T, C> {
        sorted_slice::from_mut_slice_unchecked(&mut self.array)
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn into_boxed_sorted_slice(self) -> alloc::boxed::Box<SortedSlice<T, C>> {
        sorted_slice::from_boxed_slice_unchecked(alloc::boxed::Box::new(self.array))
    }
}

impl<T: Copy, const N: usize, C: Comparator<T>> Copy for SortedArray<T, N, C> {}

impl<T: Clone, const N: usize, C: Comparator<T>> Clone for SortedArray<T, N, C> {
    fn clone(&self) -> Self {
        SortedArray {
            _comparator: PhantomData,
            array: self.array.clone(),
        }
    }
}

impl<T: Default, const N: usize, C: Comparator<T>> Default for SortedArray<T, N, C> {
    fn default() -> Self {
        SortedArray {
            _comparator: PhantomData,
            array: [(); N].map(|_| T::default()),
        }
    }
}

impl<T: Debug, const N: usize, C: Comparator<T>> Debug for SortedArray<T, N, C> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.array, fmt)
    }
}

impl<T, const N: usize, C: Comparator<T>> Deref for SortedArray<T, N, C> {
    type Target = SortedSlice<T, C>;

    fn deref(&self) -> &SortedSlice<T, C> {
        self.as_sorted_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>> DerefMut for SortedArray<T, N, C> {
    fn deref_mut(&mut self) -> &mut SortedSlice<T, C> {
        self.as_mut_sorted_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>> Borrow<[T]> for SortedArray<T, N, C> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>> Borrow<SortedSlice<T, C>> for SortedArray<T, N, C> {
    fn borrow(&self) -> &SortedSlice<T, C> {
        self.as_sorted_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>> BorrowMut<SortedSlice<T, C>> for SortedArray<T, N, C> {
    fn borrow_mut(&mut self) -> &mut SortedSlice<T, C> {
        self.as_mut_sorted_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>> AsRef<[T]> for SortedArray<T, N, C> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>> AsRef<SortedSlice<T, C>> for SortedArray<T, N, C> {
    fn as_ref(&self) -> &SortedSlice<T, C> {
        self.as_sorted_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>> AsMut<SortedSlice<T, C>> for SortedArray<T, N, C> {
    fn as_mut(&mut self) -> &mut SortedSlice<T, C> {
        self.as_mut_sorted_slice()
    }
}

impl<T, const N: usize, C: Comparator<T>, Idx> Index<Idx> for SortedArray<T, N, C>
where
    SortedSlice<T, C>: Index<Idx>,
{
    type Output = <SortedSlice<T, C> as Index<Idx>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.as_sorted_slice()[index]
    }
}

impl<T, const N: usize, C: Comparator<T>, Idx> IndexMut<Idx> for SortedArray<T, N, C>
where
    SortedSlice<T, C>: IndexMut<Idx>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.as_mut_sorted_slice()[index]
    }
}

impl<T, const N: usize, C: Comparator<T>> IntoIterator for SortedArray<T, N, C> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

impl<'a, T, const N: usize, C: Comparator<T>> IntoIterator for &'a SortedArray<T, N, C> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.iter()
    }
}

impl<'a, T, const N: usize, C: Comparator<T>> IntoIterator for &'a mut SortedArray<T, N, C> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.iter()
    }
}

pub(crate) fn from_array_unchecked<T, const N: usize, C: Comparator<T>>(
    array: [T; N],
) -> SortedArray<T, N, C> {
    SortedArray {
        _comparator: PhantomData,
        array,
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn sort_array<T, const N: usize, C: Comparator<T>>(mut array: [T; N]) -> SortedArray<T, N, C> {
    array.sort_by(C::compare);
    from_array_unchecked(array)
}

pub fn sort_array_unstable<T, const N: usize, C: Comparator<T>>(
    mut array: [T; N],
) -> SortedArray<T, N, C> {
    array.sort_unstable_by(C::compare);
    from_array_unchecked(array)
}
