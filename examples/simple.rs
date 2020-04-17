use sorted_array::{sorted_vec, Sorted};

fn main() {
    // Create a new sorted array from data.
    let mut sorted_array: Sorted<Vec<(i32, i32)>, (i32, i32)> =
        sorted_vec![(4, 2), (1, 0), (2, 2), (6, 3), (4, 1)];

    // Check that the array is now sorted.
    assert_eq!(
        sorted_array.as_slice(),
        &[(1, 0), (2, 2), (4, 1), (4, 2), (6, 3)]
    );

    // Insert a new item into the sorted array
    sorted_array.insert((3, 3));
    assert_eq!(
        sorted_array.as_slice(),
        &[(1, 0), (2, 2), (3, 3), (4, 1), (4, 2), (6, 3)]
    );

    // Find range of items whose first item is "4"
    assert_eq!(
        sorted_array.find_range_by_key(&4, |item| item.0),
        &[(4, 1), (4, 2)]
    );
}
