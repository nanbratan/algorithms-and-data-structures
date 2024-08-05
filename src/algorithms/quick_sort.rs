pub fn quick_sort(slice: &mut [i32]) {
    if slice.len() < 2 {
        return;
    }

    let pivot_index = partitioning(slice);

    // We can skip pivot elements as we know that elements on the left from it are less than pivot and elements on the right are bigger
    quick_sort(&mut slice[..pivot_index]);
    quick_sort(&mut slice[pivot_index + 1..]);
}

/// The goal of this function is find a pivot and move all items which are less(going to call them `low` below) than pivot on the left and all items which are keep in place all other items
///
/// How it's done:
/// - First, we take a middle element and move it to the end
///     - We need to move it to the end to make sure that we're going to check all elements except the pivot
/// - Then we iterate over rest elements and move `low` items to the left and keep other elements in place.
///     We don't need to care about other(bigger) elements as they're going to turn on the right anyway(all `low` elements are going to be on the left anyway)
///     - In iterator we have `left` and `right` indexes
///         - `left` is an index of first bigger item from the left. It means that if iterator is over, then `left` is the next index after latest lower item.
///         - `right` is an index of an item we're currently checking for being lower or bigger.
/// - When iterator is over we need to swap latest element with `left`, to "return" the pivot in place. Here's why:
///     - the latest element is our pivot, because we swapped it to the end to make sure that all elements are checked.
///     - `left` is next after latest lowest element in a slice(or in other words it is first biggest element from the left).
/// 
/// After "swap" we now have a pivot element with all lower elements on the left and all bigger element on the right.
fn partitioning(slice: &mut [i32]) -> usize {
    let pivot_index = slice.len() / 2;
    let pivot = slice[pivot_index];

    slice.swap(pivot_index, slice.len() - 1);

    let mut left = 0;

    for right in 0..slice.len() - 1 {
        if slice[right] <= pivot {
            slice.swap(left, right);

            left += 1;
        }
    }

    slice.swap(left, slice.len() - 1);

    left
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn should_sort_list() {
        let mut arr = vec![1, 7, 2, 0, 8, 5];

        quick_sort(&mut arr);

        assert_eq!(arr, vec![0, 1, 2, 5, 7, 8]);
    }
    #[test]
    fn should_sort_list2() {
        let mut arr = vec![1, 7677, 6, 2, 5, 0, 12, 51, 2, 88, 124, 0, 2, 88, 124, 0];

        quick_sort(&mut arr);

        assert_eq!(
            arr,
            vec![0, 0, 0, 1, 2, 2, 2, 5, 6, 12, 51, 88, 88, 124, 124, 7677]
        );
    }
}
