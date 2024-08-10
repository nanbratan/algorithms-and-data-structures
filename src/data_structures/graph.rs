// TODO: Remove line below when figure out better files structure to avoid functions with the same name as modules
#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

///
/// Graph traits
///

pub trait GraphNode
where
    Self::Id: Hash + Eq,
{
    type Id;
    type Value;

    fn id(&self) -> &Self::Id;
    fn value(&self) -> &Self::Value;
    fn nodes(&self) -> &Option<Vec<Rc<Self>>>;
}

pub trait Graph<Node, Key>
where
    Node: GraphNode,
{
    fn insert(&mut self, node: Rc<Node>);
    fn get(&self, node_id: &Key) -> Option<&Rc<Node>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

///
/// Basic graph implementation
///

#[derive(Debug)]
pub struct BasicGraphNode<T, K> {
    id: K,
    value: T,
    nodes: Option<Vec<Rc<BasicGraphNode<T, K>>>>,
}

impl<T, K> BasicGraphNode<T, K> {
    #[must_use]
    pub fn new(id: K, value: T, nodes: Option<Vec<Rc<BasicGraphNode<T, K>>>>) -> Self {
        Self { id, value, nodes }
    }
}

impl<T, K> GraphNode for BasicGraphNode<T, K>
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

pub struct BasicGraph<T, K = i32>(HashMap<K, Rc<BasicGraphNode<T, K>>>);

impl<T, K> BasicGraph<T, K> {
    #[must_use]
    pub fn new() -> Self {
        BasicGraph(HashMap::new())
    }
}

impl<T, K> Graph<BasicGraphNode<T, K>, K> for BasicGraph<T, K>
where
    K: Eq + Hash + Copy,
{
    fn insert(&mut self, node: Rc<BasicGraphNode<T, K>>) {
        self.0.insert(node.id, node);
    }
    #[must_use]
    fn get(&self, node_id: &K) -> Option<&Rc<BasicGraphNode<T, K>>> {
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

impl<T, K> Default for BasicGraph<T, K>
where
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
