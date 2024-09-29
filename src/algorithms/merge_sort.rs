// TODO: Description
#[allow(dead_code)]
pub fn merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    if arr.len() <= 1 {
        return;
    }

    let mut first_half = arr.iter().copied().take(arr.len() / 2).collect::<Vec<_>>();
    let mut second_half = arr.iter().copied().skip(arr.len() / 2).collect::<Vec<_>>();

    merge_sort(&mut first_half);
    merge_sort(&mut second_half);

    let mut first_half_index = 0;
    let mut second_half_index = 0;

    while first_half_index < first_half.len() || second_half_index < second_half.len() {
        let insertion_index = first_half_index + second_half_index;

        match (
            first_half.get(first_half_index),
            second_half.get(second_half_index),
        ) {
            (Some(first), Some(second)) => {
                if first < second {
                    arr[insertion_index] = *first;
                    first_half_index += 1;
                } else {
                    arr[insertion_index] = *second;
                    second_half_index += 1;
                }
            }
            (Some(first), None) => {
                arr[insertion_index] = *first;
                first_half_index += 1;
            }
            (None, Some(second)) => {
                arr[insertion_index] = *second;
                second_half_index += 1;
            }
            _ => break,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn should_sort_array() {
        let mut array: [i32; 8] = [3, 41, 52, 26, 38, 57, 9, 49];

        merge_sort(&mut array);

        assert_eq!(array, [3, 9, 26, 38, 41, 49, 52, 57]);
    }
}
