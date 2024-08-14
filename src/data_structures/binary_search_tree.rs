use crate::tree::{Tree, TreeNode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::{Rc, Weak};

struct BinarySearchTreeNode<V, K> {
    id: K,
    value: V,
    parent: Option<Weak<Self>>,
    nodes: RefCell<[Option<Rc<Self>>; 2]>,
}

impl<V, K> BinarySearchTreeNode<V, K> {
    #[must_use]
    pub fn new(id: K, parent: Weak<Self>, value: V) -> Self {
        Self {
            id,
            value,
            parent: Some(parent),
            nodes: RefCell::new([None, None]),
        }
    }
}

impl<V, K> TreeNode<V, K> for BinarySearchTreeNode<V, K>
where
    V: Ord + Eq,
    K: Eq + Hash + Copy + Debug,
{
    #[must_use]
    fn id(&self) -> &K {
        &self.id
    }

    #[must_use]
    fn value(&self) -> &V {
        &self.value
    }

    #[must_use]
    fn parent(&self) -> &Option<Weak<Self>> {
        &self.parent
    }

    #[must_use]
    fn nodes(&self) -> Vec<Rc<Self>> {
        self.nodes
            .borrow()
            .iter()
            .filter(|&x| x.is_some())
            .map(|x| Rc::clone(x.as_ref().unwrap()))
            .collect::<Vec<Rc<Self>>>()
    }
}

// TODO: Write docs with explanation what's the difference between a Tree and a BinarySearchTree
// TODO: Write binary search algorithm for BinarySearchTree
struct BinarySearchTree<V, K> {
    head: Rc<BinarySearchTreeNode<V, K>>,
    tree: HashMap<K, Rc<BinarySearchTreeNode<V, K>>>,
}

impl<V, K> BinarySearchTree<V, K>
where
    K: Copy + Eq + Hash,
{
    #[must_use]
    pub fn from_head(head_id: K, head_value: V) -> Self {
        let mut tree = HashMap::new();
        let head = Rc::new(BinarySearchTreeNode {
            id: head_id,
            parent: None,
            value: head_value,
            nodes: RefCell::new([None, None]),
        });

        tree.insert(head.id, Rc::clone(&head));

        Self { head, tree }
    }
}

impl<V, K> Tree<BinarySearchTreeNode<V, K>, V, K> for BinarySearchTree<V, K>
where
    V: Ord + Eq,
    K: Eq + Hash + Copy + Debug,
{
    #[must_use]
    fn head(&self) -> &Rc<BinarySearchTreeNode<V, K>> {
        &self.head
    }

    #[must_use]
    fn get(&self, node_id: &K) -> Option<&Rc<BinarySearchTreeNode<V, K>>> {
        self.tree.get(node_id)
    }

    #[must_use]
    fn len(&self) -> usize {
        self.tree.len()
    }

    #[must_use]
    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}

impl<V, K> BinarySearchTree<V, K>
where
    V: Ord + Eq,
    K: Eq + Hash + Copy + Debug,
{
    pub fn insert(&mut self, id: K, value: V) {
        let mut parent_id = self.head().id;

        let (direction, parent) = loop {
            let parent = self.tree.get(&parent_id).unwrap();
            // If a value of a new node is equal or less than a value of a parent, then we're going to insert it on the left(0 index), otherwise on the right(1 index)
            let direction = usize::from(parent.value <= value);
            let parent_nodes = parent.nodes.borrow();
            let child_node = parent_nodes[direction].as_ref();

            match child_node {
                None => break (direction, parent),
                Some(child_node) => {
                    parent_id = child_node.id;
                    continue;
                }
            }
        };

        let node = Rc::new(BinarySearchTreeNode::new(id, Rc::downgrade(parent), value));
        parent.nodes.borrow_mut()[direction] = Some(Rc::clone(&node));
        self.tree.insert(id, node);
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn should_assign_nodes_properly() {
        let mut tree = BinarySearchTree::from_head("head_id", 5);

        tree.insert("fourth", 4);
        tree.insert("third", 3);
        tree.insert("eighth", 8);
        tree.insert("sixth", 6);
        tree.insert("eleventh", 11);
        tree.insert("twenty", 20);

        // Checking nodes on sides from head, should be 4 on the left and 8 on the right
        let head_nodes = tree.head.nodes.borrow();
        let head_left = head_nodes[0].as_ref().unwrap();
        let head_right = head_nodes[1].as_ref().unwrap();
        assert_eq!(4, head_left.value);
        assert_eq!(8, head_right.value);

        // Checking nodes on sides from 4, should be 3 on the left and None on the right
        let fourth_nodes = head_nodes[0].as_ref().unwrap().nodes.borrow();
        let fourth_nodes_left = fourth_nodes[0].as_ref().unwrap();
        let fourth_nodes_right = &fourth_nodes[1];
        assert_eq!(3, fourth_nodes_left.value);
        assert!(fourth_nodes_right.is_none());

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
}
