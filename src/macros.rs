#[macro_export]
macro_rules! sorted_vec {
    ($($tt:tt)*) => {
        $crate::Sorted::new(vec![$($tt)*])
    }
}
