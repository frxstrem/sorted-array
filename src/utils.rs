use core::cmp::Ordering;

pub fn binary_search_range<T, F>(slice: &[T], mut f: F) -> (usize, usize)
where
    F: FnMut(&T) -> Ordering,
{
    let low = slice
        .binary_search_by(|it| f(it).then(Ordering::Greater))
        .unwrap_or_else(|i| i);
    let high = slice
        .binary_search_by(|it| f(it).then(Ordering::Less))
        .unwrap_or_else(|i| i);

    (low, high)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search_range() {
        let array = [1, 2, 3, 3, 3, 4, 4, 6];

        let cases = [
            (0, (0, 0)),
            (1, (0, 1)),
            (2, (1, 2)),
            (3, (2, 5)),
            (4, (5, 7)),
            (5, (7, 7)),
            (6, (7, 8)),
            (7, (8, 8)),
        ];

        for (input, expected) in &cases {
            assert_eq!(
                binary_search_range(&array, |it| Ord::cmp(it, input)),
                *expected
            );
        }
    }
}
