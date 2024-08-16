#![allow(clippy::module_name_repetitions)]

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Copy, Clone)]
enum Directions {
    Left,
    Right,
}

pub struct BinarySearchTreeNode<V, K> {
    id: K,
    value: V,
    one_side_depth: RefCell<i32>,
    parent: Weak<Self>,
    nodes: RefCell<[Option<Rc<Self>>; 2]>,
}

impl<V, K> BinarySearchTreeNode<V, K>
where
    V: Ord + Eq,
    K: Eq + Hash + Copy + Debug,
{
    #[must_use]
    pub fn new(id: K, parent: Weak<Self>, value: V) -> Self {
        Self {
            id,
            value,
            one_side_depth: RefCell::new(0),
            parent,
            nodes: RefCell::new([None, None]),
        }
    }

    #[must_use]
    pub fn nodes(&self) -> impl Deref<Target = [Option<Rc<Self>>; 2]> + '_ {
        Ref::map(self.nodes.borrow(), |x| x)
    }

    #[must_use]
    pub fn id(&self) -> &K {
        &self.id
    }

    #[must_use]
    pub fn value(&self) -> &V {
        &self.value
    }

    #[must_use]
    pub fn parent(&self) -> &Weak<Self> {
        &self.parent
    }
}

/// # Description
/// `BinarySearchTree` is just a `BinaryTree`, but with additional logic implemented into `tree.insert` method.
///
///
/// # Logic explanation
/// * The insert method doesn't provide a possibility of assigning new nodes to specific leafs, instead we're doing it automatically. User only needs to provide an `id` and a `value`.
/// * All new nodes are going to be compared against existed nodes and be assigned by principle *lower on the left, bigger on the right*.
///
/// **Please note** that `value` must be comparable, that means that Rust must have a possibility to compare two `value`s.
///
/// # What problem `BinarySearchTree` is solving
/// We have binary search algorithm for search sorted arrays, but even though binary search has `O(log n)` complexity, it still `O(n)` to insert a new item into a sorted array.
/// We still have to iterate through a whole array(in the worst case scenario of course) to find a place where we should insert the new element.
/// Even if we can find an index via binary search we'd still need to move all indexes to insert new item.
///
/// `BinarySearchTree` has `O(log n)` for both search AND inserting, which makes it superfast at all possible operations(insert, search, delete, edit, maybe something else?).
pub struct BinarySearchTree<V, K> {
    head: Rc<BinarySearchTreeNode<V, K>>,
    tree: HashMap<K, Rc<BinarySearchTreeNode<V, K>>>,
}

impl<V, K> BinarySearchTree<V, K>
where
    V: Ord + Eq,
    K: Eq + Hash + Copy + Debug,
{
    #[must_use]
    pub fn from_head(head_id: K, head_value: V) -> Self {
        let mut tree = HashMap::new();
        let head = Rc::new(BinarySearchTreeNode {
            id: head_id,
            value: head_value,
            one_side_depth: RefCell::new(0),
            parent: Weak::new(),
            nodes: RefCell::new([None, None]),
        });

        tree.insert(head.id, Rc::clone(&head));
        Self { head, tree }
    }

    #[must_use]
    pub fn head(&self) -> &Rc<BinarySearchTreeNode<V, K>> {
        &self.head
    }

    #[must_use]
    pub fn get(&self, node_id: &K) -> Option<&Rc<BinarySearchTreeNode<V, K>>> {
        self.tree.get(node_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    // We don't need to handle possible panic from `self.tree.get(&parent_id).unwrap()` as we check for None `child_node` before assigning its id to `parent_id`
    #[allow(clippy::missing_panics_doc)]
    pub fn insert(&mut self, id: K, value: V) {
        let mut parent_id = self.head().id;

        let (direction, parent) = loop {
            let parent = self.tree.get(&parent_id).unwrap();
            // If a value of a new node is equal or less than a value of a parent, then we're going to insert it on the left(0 index), otherwise on the right(1 index)
            let direction = if value > parent.value {
                Directions::Right
            } else {
                Directions::Left
            };

            let parent_nodes = parent.nodes.borrow();
            let child_node = parent_nodes[direction as usize].as_ref();

            match child_node {
                None => break (direction, parent),
                Some(child_node) => {
                    parent_id = child_node.id;
                    continue;
                }
            }
        };

        let node = Rc::new(BinarySearchTreeNode::new(id, Rc::downgrade(parent), value));
        parent.nodes.borrow_mut()[direction as usize] = Some(Rc::clone(&node));
        self.update_depth(&node, direction);
        self.tree.insert(id, node);
    }

    // TODO: Need to forget about balancing 3 items and just recursively update depth of all parent nodes after insertion
    //  If new depth is >= 2 of <= -2, then rotate

    /// We store/update depth to perform balancing if one side depth is deeper than 1 child.
    /// That means if a node has 2 children on one side and 0 children on another side, then we should balance this node.
    ///
    /// Because we balance 3 nodes only, we call them:
    /// 1. `first_level_node` - just inserted node, or the latest child in a chain.
    /// 2. `second_level_node` - parent node of `third_level_node`, or middle node in our 3 items chain
    /// 3. `third_level_node` - parent of `second_level_node`, or the first node in out 3 item chain
    fn update_depth(
        &mut self,
        inserted_node: &Rc<BinarySearchTreeNode<V, K>>,
        insert_direction: Directions,
    ) {
        // TODO: Change additional_depth to dynamic, instead of static value
        //  I should get additional depth by a child, i.e. if current child is on the left from parent, then we should bump parent depth by -1, otherwise by 1
        //  That means that it doesn't matter if inserted node has been inserted on the left or right. We should determine new depth while we travers from bottom to top of our tree.
        //  There are some additional notes in the notebook
        let additional_depth = match insert_direction {
            Directions::Left => -1,
            Directions::Right => 1,
        };

        let mut parent = inserted_node.parent().upgrade();

        while let Some(parent_node) = parent {
            let new_depth = *parent_node.one_side_depth.borrow() + additional_depth;
            *parent_node.one_side_depth.borrow_mut() = new_depth;

            if new_depth >= 2 || new_depth <= -2 {
                self.balance(&parent_node, insert_direction);
                break;
            }

            parent = parent_node.parent().upgrade();
        }
    }

    /// We balance nodes via rotation. Let's say we have nodes `3 -> 2 -> 1`. 3 points to 2(which is located on the left side) and 2 points to 1(which is located on the left side).
    /// To balance them, we can rotate them, i.e. make middle element a parent of the chain and place other items accordingly.
    /// After rotation, we should get this result `3 <- 2 -> 1`, i.e. 2 now is parent, and it points to 3(on the left) and 1(on the right).
    ///
    /// Because we balance 3 nodes only, we call them:
    /// 1. `first_level_node` - just inserted node, or the latest child in a chain.
    /// 2. `second_level_node` - parent node of `third_level_node`, or middle node in our 3 items chain
    /// 3. `third_level_node` - parent of `second_level_node`, or the first node in out 3 item chain
    fn balance(
        &mut self,
        node_to_balance: &Rc<BinarySearchTreeNode<V, K>>,
        insert_direction: Directions,
    ) {
        let opposite_direction = match insert_direction {
            Directions::Left => Directions::Right,
            Directions::Right => Directions::Left,
        };

        let mut nodes = node_to_balance.nodes.borrow_mut();
        let next_after_node_to_balance =
            Rc::clone(nodes[insert_direction as usize].as_ref().unwrap());

        nodes[insert_direction as usize] =
            next_after_node_to_balance.nodes.borrow_mut()[opposite_direction as usize].take();
        next_after_node_to_balance.nodes.borrow_mut()[opposite_direction as usize] =
            Some(Rc::clone(node_to_balance));

        match node_to_balance.parent().upgrade() {
            // Our three elements are the only elements in a tree
            None => {
                self.head = next_after_node_to_balance;
            }
            Some(parent_of_three) => {
                parent_of_three.nodes.borrow_mut()[insert_direction as usize] =
                    Some(next_after_node_to_balance);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn should_assign_nodes_properly() {
        let mut tree = BinarySearchTree::from_head("head_id", 4);

        tree.insert("third", 3);
        tree.insert("eighth", 8);
        tree.insert("sixth", 6);
        tree.insert("eleventh", 11);
        tree.insert("twenty", 20);

        // Checking nodes on sides from head, should be 4 on the left and 8 on the right
        let head_nodes = tree.head().nodes.borrow();
        let head_left = head_nodes[0].as_ref().unwrap();
        let head_right = head_nodes[1].as_ref().unwrap();
        assert_eq!(3, head_left.value);
        assert_eq!(8, head_right.value);

        // Checking nodes on sides from 4, should be 3 on the left and None on the right
        let third_nodes = head_nodes[0].as_ref().unwrap().nodes.borrow();
        assert!(third_nodes.iter().all(Option::is_none));

        // Checking nodes on sides from 8, should be 6 on the left and 11 on the right
        let eighth_nodes = head_nodes[1].as_ref().unwrap().nodes.borrow();
        let eighth_nodes_left = eighth_nodes[0].as_ref().unwrap();
        let eighth_nodes_right = eighth_nodes[1].as_ref().unwrap();
        assert_eq!(6, eighth_nodes_left.value);
        assert_eq!(11, eighth_nodes_right.value);

        // Checking nodes on sides from 8, should be None on the left and 20 on the right
        let eleventh_nodes = eighth_nodes[1].as_ref().unwrap().nodes.borrow();
        let eleventh_nodes_left = &eleventh_nodes[0];
        let eleventh_nodes_right = eleventh_nodes[1].as_ref().unwrap();
        assert!(eleventh_nodes_left.is_none());
        assert_eq!(20, eleventh_nodes_right.value);
    }

    /* #[test]
    fn should_balance_tree() {
        let mut tree = BinarySearchTree::from_head("sixty", 60);

        tree.insert("fifty", 50);
        tree.insert("forty", 40);
        tree.insert("thirty", 30);
        tree.insert("twenty", 20);

        tree.insert("ten", 10);
        tree.insert("nine", 9);

        tree.insert("seventy", 70);
        tree.insert("eighty", 80);
        tree.insert("ninety", 90);
        tree.insert("hundred", 100);

        tree.insert("sixty_five", 65);
        tree.insert("sixty_six", 66);
        tree.insert("sixty_seven", 67);

        // Checking that head node is correct after balancing
        let head = tree.head();
         assert_eq!(&50, head.value());

        // Checking child nodes of head, should be 30 on the left and 70 on the right
        let nodes = head.nodes();
        let thirty_node = nodes[0].as_ref().unwrap();
        let seventy_node = nodes[1].as_ref().unwrap();
        assert_eq!(&30, thirty_node.value());
        assert_eq!(&70, seventy_node.value());

        // Checking child nodes of 30, should be 10 on the left and 40 on the right
        let nodes = thirty_node.nodes();
        let ten_node = nodes[0].as_ref().unwrap();
        let forty_node = nodes[1].as_ref().unwrap();
        assert_eq!(&10, ten_node.value());
        assert_eq!(&40, forty_node.value());

        // Checking child nodes of 40, should be empty on both sides
        assert!(forty_node.nodes().iter().all(Option::is_none));

        // Checking child nodes of 10, should be 9 on the left and 20 on the right
        let nodes = ten_node.nodes();
        let nine_node = nodes[0].as_ref().unwrap();
        let twenty_node = nodes[1].as_ref().unwrap();
        assert_eq!(&9, nine_node.value());
        assert_eq!(&20, twenty_node.value());

        // Checking child nodes of 9, should be empty on both sides
        assert!(nine_node.nodes().iter().all(Option::is_none));
        // Checking child nodes of 20, should be empty on both sides
        assert!(twenty_node.nodes().iter().all(Option::is_none));

        // Checking child nodes of 70, should be 65 on the left and 80 on the right
        let nodes = seventy_node.nodes();
        let sixty_five_node = nodes[0].as_ref().unwrap();
        let eighty_node = nodes[1].as_ref().unwrap();
        assert_eq!(&65, sixty_five_node.value());
        assert_eq!(&80, eighty_node.value());

        // Checking child nodes of 60, should be 60 on the left and 66 on the right
        let nodes = sixty_five_node.nodes();
        let sixty_node = nodes[0].as_ref().unwrap();
        let sixty_six_node = nodes[1].as_ref().unwrap();
        assert_eq!(&60, sixty_node.value());
        assert_eq!(&66, sixty_six_node.value());

        // Checking child nodes of 60, should be empty on both sides
        assert!(sixty_node.nodes().iter().all(Option::is_none));

        // Checking child nodes of 60, should be empty on the left and 67 on the right
        let nodes = sixty_six_node.nodes();
        let sixty_seven_node = nodes[1].as_ref().unwrap();
        assert!(nodes[0].is_none());
        assert_eq!(&67, sixty_seven_node.value());

        // Checking child nodes of 60, should be empty on the left and 10 on the right
        let nodes = eighty_node.nodes();
        let hundred_node = nodes[1].as_ref().unwrap();
        assert!(nodes[0].is_none());
        assert_eq!(&10, hundred_node.value());

        // Checking child nodes of 100, should be empty on both sides
        assert!(hundred_node.nodes().iter().all(Option::is_none));
    }*/
}
