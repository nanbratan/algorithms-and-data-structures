/// # Description
/// Insertion sort has O(n^2) time complexity, which makes it very slow sorting algorithm.
/// However, its constant time is usually faster than merge sort. So it makes sense to use insertion sort for small input
/// and other(merge sort/quick sort) for big input.
#[allow(dead_code)]
pub fn insertion_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i - 1;

        while j != usize::MAX && arr[j] > key {
            arr[j + 1] = arr[j];
            j = j.wrapping_sub(1);
        }

        arr[j.wrapping_add(1)] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;

    #[test]
    fn should_sort_array() {
        let mut array: [i32; 8] = [3, 41, 52, 26, 38, 57, 9, 49];

        insertion_sort(&mut array);

        assert_eq!(array, [3, 9, 26, 38, 41, 49, 52, 57]);
    }
}
