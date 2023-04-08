This crate provides [`SortedSlice`], [`SortedArray`] and [`SortedVec`], which are sorted variants of `[T]`, `[T; N]` and `Vec<T>`.

These types have a restricted set of operations to guarantee that they are always sorted. For instance,
it is not possible to get mutable references to the underlying slices or items within, as that could
allow altering the array so that the items are no longer in order.

The types also have some additional functions that rely on the fact that the array is sorted to be
more efficient that the standard slice functions.
