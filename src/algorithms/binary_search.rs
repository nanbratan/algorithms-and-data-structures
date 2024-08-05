use std::cmp::Ordering;

/// # Description
/// This algorithm uses binary search.
///
/// # Complexity
/// O(log n)
///
/// # Explanation
/// This algorithm works **ONLY** with sorted lists.
///
/// It takes 0 index as a `low` position and latest index as a `high` position, then it follows next steps in a loop:
/// - if `low` is `high`, then it means we reached the end of our vector, so there is no desired element in our list, return `None`
/// - Calculating a middle element index by `(low + high) / 2` and compares it to the desired element
/// - if middle element is desired element, then return `Some(mid)`
/// - else if middle element is bigger than the desired one, then we shift `high` to `mid - 1`(we don't need to keep `mid` index as we already know that it is wrong). Or in other words we take a vector slice on the left from the middle element as the desired element is lower that current middle one.
/// - else if middle element is lower than the desired one, then we shift `low` to `mid + 1`(we don't need to keep `mid` index as we already know that it is wrong). Or in other words we take a vector slice on the right from the middle element as the desired element is bigger that current middle one.
pub fn binary_search<T>(list: &[T], element: T) -> Option<usize>
where
    T: Eq + Ord,
{
    let mut low = 0;
    let mut high = list.len() - 1;

    loop {
        let mid = (low + high) / 2;

        if low == high {
            break None;
        }

        match element.cmp(&list[mid]) {
            Ordering::Equal => break Some(mid),
            Ordering::Less => {
                high = mid - 1
            }
            Ordering::Greater => {
                low = mid + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    fn get_list() -> Vec<i32> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]
    }

    #[test]
    fn should_find_item() {
        assert_eq!(binary_search::<i32>(&get_list(), 28), Some(28));
    }
    #[test]
    fn should_return_none_if_not_exist() {
        assert_eq!(binary_search::<i32>(&get_list(), 45), None);
    }
}

