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
pub fn binary_search<T>(list: &Vec<T>, element: &T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    // This is only for visualisation
    let mut iterations = 0;
    let mut low = 0;
    let mut high = list.len() - 1;

    loop {
        if low == high {
            break None;
        }

        iterations += 1;
        println!("iterations: {iterations}");

        let mid = (low + high) / 2;

        match &list[mid] {
            guess if guess > element => {
                high = mid - 1;
                continue;
            }
            guess if guess < element => {
                low = mid + 1;
                continue;
            }
            _ => {
                break Some(mid);
            }
        }
    }
}