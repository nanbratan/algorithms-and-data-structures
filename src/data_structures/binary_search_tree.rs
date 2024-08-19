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

impl Directions {
    fn get_opposite(direction: Directions) -> Directions {
        match direction {
            Directions::Left => Directions::Right,
            Directions::Right => Directions::Left,
        }
    }

    fn get_depth(direction: Directions) -> i32 {
        match direction {
            Directions::Left => -1,
            Directions::Right => 1,
        }
    }
}

pub struct BinarySearchTreeNode<V, K> {
    id: K,
    value: V,
    one_side_depth: RefCell<i32>,
    parent: RefCell<Weak<Self>>,
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
            parent: RefCell::new(parent),
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
    pub fn parent(&self) -> impl Deref<Target = Weak<Self>> + '_ {
        Ref::map(self.parent.borrow(), |x| x)
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
pub struct AVLTree<V, K> {
    head: Rc<BinarySearchTreeNode<V, K>>,
    tree: HashMap<K, Rc<BinarySearchTreeNode<V, K>>>,
}

impl<V, K> AVLTree<V, K>
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
            parent: RefCell::new(Weak::new()),
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
        self.tree.insert(id, Rc::clone(&node));
        self.update_depth(&node);
    }

    fn get_directions(
        parent: &Rc<BinarySearchTreeNode<V, K>>,
        child: &Rc<BinarySearchTreeNode<V, K>>,
    ) -> Directions {
        if let Some(left) = parent.nodes.borrow()[Directions::Left as usize].as_ref() {
            if left.id == child.id {
                return Directions::Left;
            }
        }

        Directions::Right
    }

    /// We store/update depth to perform balancing if one side depth is deeper than 1 child.
    /// That means if a node has 2 children on one side and 0 children on another side, then we should balance this node.
    ///
    /// Because we balance 3 nodes only, we call them:
    /// 1. `first_level_node` - just inserted node, or the latest child in a chain.
    /// 2. `second_level_node` - parent node of `third_level_node`, or middle node in our 3 items chain
    /// 3. `third_level_node` - parent of `second_level_node`, or the first node in out 3 item chain
    fn update_depth(&mut self, inserted_node: &Rc<BinarySearchTreeNode<V, K>>) {
        // We need to keep track of a current node(parent) and its child.
        // We need to keep track of them to calculate edge direction(left or right) and additional depth(-1 or 1)
        let mut child = Rc::clone(inserted_node);
        let mut parent = child.parent().upgrade();

        // We travers through full branch from bottom to top to set new depth for all affected nodes.
        // Head doesn't have parent, so when we reach head - this loop will end.
        while let Some(parent_node) = parent {
            let direction = AVLTree::get_directions(&parent_node, &child);
            let additional_depth = match direction {
                Directions::Left => -1,
                Directions::Right => 1,
            };

            let child_depth = *child.one_side_depth.borrow();
            let new_parent_depth = *parent_node.one_side_depth.borrow() + additional_depth;
            *parent_node.one_side_depth.borrow_mut() = new_parent_depth;

            let is_simple_rotation = new_parent_depth >= 2 && child_depth > 0
                || new_parent_depth <= -2 && child_depth < 0;
            // If 3 items are not aligned in a single line, then we need to align them first via double rotation(do left-right/right-left rotation and then left/right rotation).
            let is_double_rotation = new_parent_depth >= 2 && child_depth < 0
                || new_parent_depth <= -2 && child_depth > 0;

            if is_simple_rotation {
                self.simple_rotation(&parent_node, direction);
                break;
            }

            if is_double_rotation {
                self.double_rotation(&parent_node, direction);
                // TODO: simple_rotation should be inside double_rotation as it is a part of it(we do left-right/right-left rotation and then left/right rotation)
                //  But, dues to similar borrowing - they conflict with each other. They work fine when separate though, so I may keep it this way, but it'd be much better to merge them.
                self.simple_rotation(&parent_node, direction);
                break;
            }

            parent = parent_node.parent().upgrade();
            child = parent_node;
        }
    }

    /// We balance nodes via rotation. Let's say we have nodes `3 -> 2 -> 1`. 3 points to 2(which is located on the left side) and 2 points to 1(which is located on the left side).
    /// To balance them, we can rotate them, i.e. make middle element a parent of the chain and place other items accordingly.
    /// After rotation, we should get this result `3 <- 2 -> 1`, i.e. 2 now is parent, and it points to 3(on the left) and 1(on the right).
    //
    //       Before       After
    //         A
    //        /             B
    //       B             / \
    //      /             C   A
    //     C
    //
    /// Because we balance 3 nodes only, we call them:
    /// 1. `first_level_node` - just inserted node, or the latest child in a chain.
    /// 2. `second_level_node` - parent node of `third_level_node`, or middle node in our 3 items chain
    /// 3. `third_level_node` - parent of `second_level_node`, or the first node in out 3 item chain
    fn simple_rotation(
        &mut self,
        first_level_node: &Rc<BinarySearchTreeNode<V, K>>,
        balance_direction: Directions,
    ) {
        let opposite_direction = Directions::get_opposite(balance_direction);

        let mut nodes_of_first_level = first_level_node.nodes.borrow_mut();
        let second_level_node = Rc::clone(
            nodes_of_first_level[balance_direction as usize]
                .as_ref()
                .unwrap(),
        );

        // If second child(e.g. B from docs) has a child on the opposite side(e.g. we balance nodes on the left side and second node has a child on the right side)
        // Then we should move this child from second node to first node.
        let second_level_node_opposite_child =
            second_level_node.nodes.borrow_mut()[opposite_direction as usize].take();

        if let Some(second_level_node_opposite_child) = second_level_node_opposite_child {
            *second_level_node_opposite_child.parent.borrow_mut() = Rc::downgrade(first_level_node);
            nodes_of_first_level[balance_direction as usize] =
                Some(second_level_node_opposite_child);
        } else {
            nodes_of_first_level[balance_direction as usize] = None;
        }

        // Moving first_level_node to second_level_node children
        second_level_node.nodes.borrow_mut()[opposite_direction as usize] =
            Some(Rc::clone(first_level_node));

        *second_level_node.one_side_depth.borrow_mut() = 0;
        *first_level_node.one_side_depth.borrow_mut() = 0;

        let second_level_node_weak_link = Rc::downgrade(&second_level_node);

        match first_level_node.parent().upgrade() {
            // Our three elements are the only elements in a tree
            None => {
                *second_level_node.parent.borrow_mut() = Weak::new();
                self.head = second_level_node;
            }
            Some(parent_of_three) => {
                let insert_direction_for_parent_of_three =
                    AVLTree::get_directions(&parent_of_three, first_level_node);

                *second_level_node.parent.borrow_mut() = Rc::downgrade(&parent_of_three);
                parent_of_three.nodes.borrow_mut()[insert_direction_for_parent_of_three as usize] =
                    Some(second_level_node);
            }
        }

        // Making second_level_node a parent of first_level_node
        *first_level_node.parent.borrow_mut() = second_level_node_weak_link;
    }

    /// If there is a node with depth -2/2, but its child has the opposite depth(for example our node has depth 2, but it's children has -1), then we need to align nodes into one line
    /// and only then make a single rotation
    /// To balance them, we can rotate them, i.e. make middle element a parent of the chain and place other items accordingly.
    /// After rotation, we should get this result `3 <- 2 -> 1`, i.e. 2 now is parent, and it points to 3(on the left) and 1(on the right).
    ///
    //       Before       After        The final result(when we run single rotation after double rotation)
    //         A             A                 C
    //        /             /                 / \
    //       B             C                 B   A
    //        \           /
    //         C         B
    fn double_rotation(
        &self,
        first_level_node: &Rc<BinarySearchTreeNode<V, K>>,
        balance_direction: Directions,
    ) {
        let opposite_direction = Directions::get_opposite(balance_direction);

        let mut nodes_of_first_level = first_level_node.nodes.borrow_mut();
        let second_level_node = Rc::clone(
            nodes_of_first_level[balance_direction as usize]
                .as_ref()
                .unwrap(),
        );

        let mut nodes_of_second_level = second_level_node.nodes.borrow_mut();
        let third_level_node = Rc::clone(
            nodes_of_second_level[opposite_direction as usize]
                .as_ref()
                .unwrap(),
        );

        *first_level_node.one_side_depth.borrow_mut() =
            Directions::get_depth(balance_direction) * 2;
        *second_level_node.one_side_depth.borrow_mut() = Directions::get_depth(balance_direction);
        *third_level_node.one_side_depth.borrow_mut() = 0;

        let third_level_node_same_line_child =
            third_level_node.nodes.borrow_mut()[balance_direction as usize].take();

        // If third child(e.g. C from docs) has a child on the opposite side from where it exists
        // For example, C exists on the right from B, if C has a child on the left, then we put this child on the right from B(where C exists at the moment)
        if let Some(third_level_node_same_line_child) = third_level_node_same_line_child {
            *third_level_node_same_line_child.parent.borrow_mut() =
                Rc::downgrade(&second_level_node);
            nodes_of_second_level[opposite_direction as usize] =
                Some(third_level_node_same_line_child);
        } else {
            nodes_of_second_level[opposite_direction as usize] = None;
        }

        // Putting second node in children of third node(putting B in children of C)
        third_level_node.nodes.borrow_mut()[balance_direction as usize] =
            Some(Rc::clone(&second_level_node));
        *second_level_node.parent.borrow_mut() = Rc::downgrade(&third_level_node);

        // Put third node(C from docs) in a place of second node(in a place of B)
        nodes_of_first_level[balance_direction as usize] = Some(Rc::clone(&third_level_node));
        *third_level_node.parent.borrow_mut() = Rc::downgrade(first_level_node);
    }
}

#[cfg(test)]
mod tests {
    use super::AVLTree;

    #[test]
    fn should_assign_nodes_properly() {
        let mut tree = AVLTree::from_head("head_id", 4);

        tree.insert("third", 3);
        tree.insert("eighth", 8);
        tree.insert("sixth", 6);
        tree.insert("eleventh", 11);
        tree.insert("twenty", 20);

        // Checking that head node is correct after balancing
        let head = tree.head();
        assert_eq!(8, head.value);

        // Checking nodes on sides from head, should be 4 on the left and 8 on the right
        let head_nodes = head.nodes.borrow();
        let four_node = head_nodes[0].as_ref().unwrap();
        let eleven_node = head_nodes[1].as_ref().unwrap();
        assert_eq!(4, four_node.value);
        assert_eq!(11, eleven_node.value);

        // Checking nodes on sides from 4, should be 3 on the left and 6 on the right
        let four_nodes = four_node.nodes.borrow();
        let three_node = four_nodes[0].as_ref().unwrap();
        let six_node = four_nodes[1].as_ref().unwrap();
        assert_eq!(3, three_node.value);
        assert_eq!(6, six_node.value);

        let third_nodes = three_node.nodes.borrow();
        assert!(third_nodes.iter().all(Option::is_none));
        let six_nodes = six_node.nodes.borrow();
        assert!(six_nodes.iter().all(Option::is_none));

        // Checking nodes on sides from 8, should be 6 on the left and 11 on the right
        let eleven_nodes = eleven_node.nodes.borrow();
        let twenty_node = eleven_nodes[1].as_ref().unwrap();
        assert!(eleven_nodes[0].is_none());
        assert_eq!(20, twenty_node.value);

        // Checking nodes on sides from 8, should be None on the left and 20 on the right
        let twenty_nodes = twenty_node.nodes.borrow();
        assert!(twenty_nodes.iter().all(Option::is_none));
    }

    #[test]
    fn should_balance_tree_3() {
        let mut tree = AVLTree::from_head("sixty", 60);

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
        let ninety_node = nodes[1].as_ref().unwrap();
        assert_eq!(&65, sixty_five_node.value());
        assert_eq!(&90, ninety_node.value());

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
        let nodes = ninety_node.nodes();
        let eighty_node = nodes[0].as_ref().unwrap();
        let hundred_node = nodes[1].as_ref().unwrap();

        assert_eq!(&80, eighty_node.value());
        assert_eq!(&100, hundred_node.value());

        // Checking child nodes of 100, should be empty on both sides
        assert!(eighty_node.nodes().iter().all(Option::is_none));
        // Checking child nodes of 80, should be empty on both sides
        assert!(hundred_node.nodes().iter().all(Option::is_none));
    }
}
