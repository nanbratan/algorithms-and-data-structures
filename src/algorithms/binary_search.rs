#![allow(clippy::module_name_repetitions)]

use crate::binary_search_tree::{BinarySearchTree, BinarySearchTreeNode};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

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
pub fn binary_search<T>(list: &[T], element: &T) -> Option<usize>
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
                high = mid - 1;
            }
            Ordering::Greater => {
                low = mid + 1;
            }
        }
    }
}
pub fn binary_search_for_tree<V, K>(
    tree: &BinarySearchTree<V, K>,
    desired_value: &V,
) -> Option<Rc<BinarySearchTreeNode<V, K>>>
where
    V: Eq + Ord,
    K: Hash + Eq + Copy + Debug,
{
    let mut current_node = Rc::clone(tree.head());

    loop {
        if current_node.value() == desired_value {
            break Some(current_node);
        }

        // If a value of the `current_node` is lower or equal that the `desired_value`, then we're going to search lower items(on the left), otherwise we're going to search bigger items(on the right)
        let direction = usize::from(current_node.value() <= desired_value);
        // I'm getting current node from the tree here as without it here is an error that we can't re-assign `current_node` while it is still borrowed.
        // Would like to get rid of tree.get() call here, but right now I don't know how
        let nodes = tree.get(current_node.id())?.nodes();

        match nodes[direction].as_ref() {
            None => break None,
            Some(next_node) => {
                current_node = Rc::clone(next_node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{binary_search, binary_search_for_tree};
    use crate::binary_search_tree::BinarySearchTree;
    use crate::tree::TreeNode;

    fn get_list() -> Vec<i32> {
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32,
        ]
    }
    fn get_binary_tree<'a>() -> BinarySearchTree<i32, &'a str> {
        let mut tree = BinarySearchTree::from_head("head_id", 5);

        tree.insert("fourth", 4);
        tree.insert("third", 3);
        tree.insert("eighth", 8);
        tree.insert("sixth", 6);
        tree.insert("eleventh", 11);
        tree.insert("twenty", 20);

        tree
    }

    #[test]
    fn should_find_item_in_vector() {
        assert_eq!(binary_search::<i32>(&get_list(), &28), Some(28));
    }
    #[test]
    fn should_return_none_if_not_exist_in_vector() {
        assert_eq!(binary_search::<i32>(&get_list(), &45), None);
    }

    #[test]
    fn should_find_item_in_binary_tree() {
        // given
        let tree = get_binary_tree();

        // when
        let found_node = binary_search_for_tree(&tree, &20);

        // then
        assert_eq!(found_node.unwrap().id(), &"twenty");
    }
    #[test]
    fn should_return_none_if_not_exist_in_binary_tree() {
        // given
        let tree = get_binary_tree();

        // when
        let found_node = binary_search_for_tree(&tree, &9999);

        // then
        assert!(found_node.is_none());
    }
}
