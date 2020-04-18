/// Macro to easily create a sorted array.
///
/// `sorted_vec![...]` is expanded to <code>[Sorted::new]\(vec![...])</code>.
#[macro_export]
macro_rules! sorted_vec {
    [$($tt:tt)*] => {
        $crate::Sorted::new(vec![$($tt)*])
    }
}
