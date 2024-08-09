#![allow(clippy::module_name_repetitions)]

use crate::graph::{Graph, GraphNode};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;


/// The difference between a Graph and a Tree is that a Tree can't have cycles, i.e. nodes of a tree can't point to each other in both ways(children can't point to parents).
/// So with trees we can safely have parent_id field
pub trait TreeNode: GraphNode
where
    Self::ParentId: Hash + Eq,
{
    type ParentId;

    fn parent_id(&self) -> &Option<Self::ParentId>;
}

pub trait Tree<Node, K>: Graph<Node, K>
where
    Node: TreeNode,
{}

#[derive(Debug)]
pub struct BasicTreeNode<T, K> {
    id: K,
    parent_id: Option<K>,
    value: T,
    nodes: Option<Vec<Rc<BasicTreeNode<T, K>>>>,
}

impl<T, K> BasicTreeNode<T, K> {
    #[must_use]
    pub fn new(id: K, parent_id: Option<K>, value: T, nodes: Option<Vec<Rc<BasicTreeNode<T, K>>>>) -> Self {
        Self { id, parent_id, value, nodes }
    }
}

impl<T, K> GraphNode for BasicTreeNode<T, K>
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
    fn nodes(&self) -> &Option<Vec<Rc<Self>>> {
        &self.nodes
    }
}

impl<T, K> TreeNode for BasicTreeNode<T, K>
where
    K: Hash + Eq,
{
    type ParentId = K;

    #[must_use]
    fn parent_id(&self) -> &Option<Self::ParentId> {
        &self.parent_id
    }
}

pub struct BasicTree<T, K = i32>(HashMap<K, Rc<BasicTreeNode<T, K>>>);

impl<T, K> BasicTree<T, K> {
    #[must_use]
    pub fn new() -> Self {
        BasicTree(HashMap::new())
    }
}

impl<T, K> Graph<BasicTreeNode<T, K>, K> for BasicTree<T, K>
where
    K: Eq + Hash + Copy,
{
    fn insert(&mut self, node: Rc<BasicTreeNode<T, K>>) {
        self.0.insert(node.id, node);
    }
    #[must_use]
    fn get(&self, node_id: &K) -> Option<&Rc<BasicTreeNode<T, K>>> {
        self.0.get(node_id)
    }
    #[must_use]
    fn len(&self) -> usize {
        self.0.len()
    }
    #[must_use]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T, K> Default for BasicTree<T, K>
where
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
