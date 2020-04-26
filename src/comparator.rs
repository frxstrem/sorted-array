use core::cmp::Ordering;

pub trait Comparator<T: ?Sized> {
    fn compare(x: &T, y: &T) -> Ordering;
}

pub struct OrdComparator;

impl<T: Ord + ?Sized> Comparator<T> for OrdComparator {
    fn compare(x: &T, y: &T) -> Ordering {
        Ord::cmp(x, y)
    }
}
