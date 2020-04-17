use super::*;

const TEST_ARRAY: [(i32, i32); 7] = [(1, 3), (3, 1), (2, 3), (2, 2), (2, 1), (3, 2), (5, 3)];

#[test]
fn test_find_by() {
    let array = Sorted::new(TEST_ARRAY);

    let cmp_fn = |n: i32| move |it: &(i32, i32)| Ord::cmp(&it.0, &n);

    assert_eq!(array.find_by(cmp_fn(0)).map(|(a, _)| a), None);
    assert_eq!(array.find_by(cmp_fn(1)).map(|(a, _)| a), Some(&1));
    assert_eq!(array.find_by(cmp_fn(2)).map(|(a, _)| a), Some(&2));
    assert_eq!(array.find_by(cmp_fn(3)).map(|(a, _)| a), Some(&3));
    assert_eq!(array.find_by(cmp_fn(4)).map(|(a, _)| a), None);
    assert_eq!(array.find_by(cmp_fn(5)).map(|(a, _)| a), Some(&5));
    assert_eq!(array.find_by(cmp_fn(6)).map(|(a, _)| a), None);
}

#[test]
fn test_find_range_by() {
    let array = Sorted::new(TEST_ARRAY);

    let cmp_fn = |n: i32| move |it: &(i32, i32)| Ord::cmp(&it.0, &n);

    assert_eq!(array.find_range_by(cmp_fn(0)), &[]);
    assert_eq!(array.find_range_by(cmp_fn(1)), &[(1, 3)]);
    assert_eq!(array.find_range_by(cmp_fn(2)), &[(2, 1), (2, 2), (2, 3)]);
    assert_eq!(array.find_range_by(cmp_fn(3)), &[(3, 1), (3, 2)]);
    assert_eq!(array.find_range_by(cmp_fn(4)), &[]);
    assert_eq!(array.find_range_by(cmp_fn(5)), &[(5, 3)]);
    assert_eq!(array.find_range_by(cmp_fn(6)), &[]);
}

#[test]
fn test_find_by_key() {
    let array = Sorted::new(TEST_ARRAY);

    let key_fn = |item: &(i32, i32)| item.0;

    assert_eq!(array.find_by_key(&0, key_fn).map(|(a, _)| a), None);
    assert_eq!(array.find_by_key(&1, key_fn).map(|(a, _)| a), Some(&1));
    assert_eq!(array.find_by_key(&2, key_fn).map(|(a, _)| a), Some(&2));
    assert_eq!(array.find_by_key(&3, key_fn).map(|(a, _)| a), Some(&3));
    assert_eq!(array.find_by_key(&4, key_fn).map(|(a, _)| a), None);
    assert_eq!(array.find_by_key(&5, key_fn).map(|(a, _)| a), Some(&5));
    assert_eq!(array.find_by_key(&6, key_fn).map(|(a, _)| a), None);
}

#[test]
fn test_find_range_by_key() {
    let array = Sorted::new(TEST_ARRAY);

    let key_fn = |item: &(i32, i32)| item.0;

    assert_eq!(array.find_range_by_key(&0, key_fn), &[]);
    assert_eq!(array.find_range_by_key(&1, key_fn), &[(1, 3)]);
    assert_eq!(
        array.find_range_by_key(&2, key_fn),
        &[(2, 1), (2, 2), (2, 3)]
    );
    assert_eq!(array.find_range_by_key(&3, key_fn), &[(3, 1), (3, 2)]);
    assert_eq!(array.find_range_by_key(&4, key_fn), &[]);
    assert_eq!(array.find_range_by_key(&5, key_fn), &[(5, 3)]);
    assert_eq!(array.find_range_by_key(&6, key_fn), &[]);
}

#[test]
fn test_insert() {
    let mut array = Sorted::new(Vec::new());

    array.insert(1);
    array.insert(4);
    array.insert(2);
    assert_eq!(array.as_slice(), &[1, 2, 4]);

    array.insert(5);
    array.insert(3);
    assert_eq!(array.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_extend() {
    let mut array = Sorted::new(vec![1, 7, 3]);
    assert_eq!(array.as_slice(), &[1, 3, 7]);

    array.extend(vec![4, 6]);
    assert_eq!(array.as_slice(), &[1, 3, 4, 6, 7]);

    array.extend(vec![5, 2]);
    assert_eq!(array.as_slice(), &[1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_from_iterator() {
    let array: Sorted<Vec<_>, _> = vec![1, 7, 3, 2, 5].into_iter().collect();
    assert_eq!(array.as_slice(), &[1, 2, 3, 5, 7]);
}

#[test]
fn test_iter() {
    let array = Sorted::new(vec![1, 7, 3, 2, 5]);
    assert_eq!(array.iter().collect::<Vec<_>>(), vec![&1, &2, &3, &5, &7]);
}
