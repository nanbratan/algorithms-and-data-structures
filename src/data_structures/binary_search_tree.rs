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
    // TODO: write tests for BinarySearchTree to ensure that we insert nodes in correct order.
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
