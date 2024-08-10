#![allow(clippy::module_name_repetitions)]

/// TODO: I'm going to remove Graph/GraphNode traits from Tree/TreeNode as `GraphNode` has immutable nodes, but `TreeNode`'s nodes require to be mutable(`RefCell`) in order to add new children to a tree.
///     I've tried to make `GraphNode`'s nodes `RefCell`, but it doesn't seem super straight forward how to do it for `breadth_first_search` algorithm, integration with `Queue` is breaking when I'm trying.
///     So for now I'm not going to use `Graph`/`GraphNode` traits here as I want to implement `Tree` first, then try to move `Graph` to mutable nodes and only then use `Graph`/`GraphNode` traits here.
///     I'm not doing it all at one as it seems confusing and time consuming, so I'm going to splitting tasks.
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::{Rc, Weak};


/// The difference between a Graph and a Tree is that a Tree can't have cycles, i.e. nodes of a tree can't point to each other in both ways(children can't point to parents).
/// So with trees we can safely have `parent_id` field
pub trait TreeNode
where
    Self::Id: Hash + Eq,
{
    type Id;
    type Value;

    fn id(&self) -> &Self::Id;
    fn value(&self) -> &Self::Value;
    fn parent(&self) -> &Option<Weak<Self>>;
    fn nodes(&self) -> &RefCell<Vec<Rc<Self>>>;
}

pub trait Tree<Node, Key, V>
where
    Node: TreeNode<Value=V>,
{
    fn head(&self) -> &Rc<Node>;
    fn insert(&mut self, id: Key, parent_id: Key, value: V);
    fn get(&self, node_id: &Key) -> Option<&Rc<Node>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub struct BasicTreeNode<V, K>
{
    id: K,
    parent: Option<Weak<Self>>,
    value: V,
    nodes: RefCell<Vec<Rc<Self>>>,
}

impl<V, K> BasicTreeNode<V, K> {
    #[must_use]
    pub fn new(id: K, parent: Weak<Self>, value: V) -> Self {
        Self {
            id,
            parent: Some(parent),
            value,
            nodes: RefCell::new(vec![]),
        }
    }
}

impl<T, K> TreeNode for BasicTreeNode<T, K>
where
    K: Hash + Eq,
{
    type Id = K;
    type Value = T;

    #[must_use]
    fn id(&self) -> &Self::Id {
        &self.id
    }
    #[must_use]
    fn value(&self) -> &Self::Value {
        &self.value
    }
    #[must_use]
    fn parent(&self) -> &Option<Weak<Self>> {
        &self.parent
    }
    #[must_use]
    fn nodes(&self) -> &RefCell<Vec<Rc<Self>>> {
        &self.nodes
    }
}

pub struct BasicTree<V, K = i32> {
    head: Rc<BasicTreeNode<V, K>>,
    tree: HashMap<K, Rc<BasicTreeNode<V, K>>>,
}

impl<T, K> BasicTree<T, K>
where
    K: Eq + Hash + Copy,
{
    #[must_use]
    pub fn from_head(
        head_id: K,
        head_value: T,
    ) -> Self {
        let head = Rc::new(BasicTreeNode {
            id: head_id,
            parent: None,
            value: head_value,
            nodes: RefCell::new(vec![]),
        });
        let mut tree = HashMap::new();

        tree.insert(head.id, Rc::clone(&head));

        Self { head, tree }
    }
}

impl<V, K> Tree<BasicTreeNode<V, K>, K, V> for BasicTree<V, K>
where
    K: Eq + Hash + Copy + Debug,
{
    #[must_use]
    fn head(&self) -> &Rc<BasicTreeNode<V, K>> {
        &self.head
    }
    fn insert(&mut self, id: K, parent_id: K, value: V) {
        let parent = self.get(&parent_id);

        match parent {
            None => panic!("Can't insert a new leaf, parent with id \"{parent_id:?}\" doesn't exist"),
            Some(parent) => {
                let node = Rc::new(BasicTreeNode::new(id, Rc::downgrade(parent), value));

                parent.nodes.borrow_mut().push(Rc::clone(&node));
                self.tree.insert(id, node);
            }
        }
    }
    #[must_use]
    fn get(&self, node_id: &K) -> Option<&Rc<BasicTreeNode<V, K>>> {
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

