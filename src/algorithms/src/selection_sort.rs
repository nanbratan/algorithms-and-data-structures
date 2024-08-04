pub enum Order {
    Desc,
    Asc,
}

/// # Description
/// Uses selection sort
///
/// # Complexity
/// O(n^2)
pub fn selection_sort_by_key<T, K, F>(list: &mut Vec<T>, order: Order, mut f: F)
where
    K: PartialOrd,
    F: FnMut(&T) -> &K + Copy,
{
    for current in 0..list.len() {
        let mut smallest = current;

        for next in current + 1..list.len() {
            match order {
                Order::Desc if f(&list[smallest]) < f(&list[next]) => smallest = next,
                Order::Asc if f(&list[smallest]) > f(&list[next]) => smallest = next,
                _ => continue,
            }
        }

        list.swap(current, smallest);
    }
}

/// # Description
/// Uses selection sort
///
/// # Complexity
/// O(n^2)
pub fn selection_sort<T>(list: &mut Vec<T>, order: Order)
where
    T: PartialOrd,
{
    selection_sort_by_key(list, order, |x| x);
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::{selection_sort, Order};

    #[derive(Debug)]
    struct Book {
        pages: u32,
    }
    impl PartialEq<Self> for Book {
        fn eq(&self, other: &Self) -> bool {
            &self.pages == &other.pages
        }
    }
    impl PartialOrd for Book {
        fn partial_cmp(&self, other: &Book) -> Option<Ordering> {
            if &self.pages > &other.pages {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }

    fn get_simple_list() -> Vec<i32> {
        vec![
            1, 2554, 6, 3, 8, 2, 12, 656, 42, 21, 15, 4, 61, 32, 96, 124, 0,
        ]
    }
    fn get_struct_list() -> Vec<Book> {
        vec![
            Book { pages: 12 },
            Book { pages: 51515 },
            Book { pages: 25 },
            Book { pages: 14 },
            Book { pages: 222 },
            Book { pages: 424 },
            Book { pages: 1 },
        ]
    }

    #[test]
    fn should_sort_simple_list_asc() {
        // given
        let mut list = get_simple_list();

        // when
        selection_sort(&mut list, Order::Asc);

        // then
        assert_eq!(
            list,
            vec![0, 1, 2, 3, 4, 6, 8, 12, 15, 21, 32, 42, 61, 96, 124, 656, 2554]
        );
    }
    #[test]
    fn should_sort_simple_list_desc() {
        // given
        let mut list = get_simple_list();

        // when
        selection_sort(&mut list, Order::Desc);

        // then
        assert_eq!(
            list,
            vec![2554, 656, 124, 96, 61, 42, 32, 21, 15, 12, 8, 6, 4, 3, 2, 1, 0]
        );
    }

    #[test]
    fn should_sort_simple_list_by_key_asc() {
        let mut list = get_struct_list();

        // when
        selection_sort(&mut list, Order::Asc);

        // then
        assert_eq!(
            list,
            vec![
                Book { pages: 1 },
                Book { pages: 12 },
                Book { pages: 14 },
                Book { pages: 25 },
                Book { pages: 222 },
                Book { pages: 424 },
                Book { pages: 51515 }
            ]
        );
    }
    #[test]
    fn should_sort_simple_list_by_key_desc() {
        let mut list = get_struct_list();

        // when
        selection_sort(&mut list, Order::Desc);

        // then
        assert_eq!(
            list,
            vec![
                Book { pages: 51515 },
                Book { pages: 424 },
                Book { pages: 222 },
                Book { pages: 25 },
                Book { pages: 14 },
                Book { pages: 12 },
                Book { pages: 1 }
            ]
        );
    }
}
