#![allow(clippy::module_name_repetitions)]

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct Edge<K> {
    weight: i32,
    node: Rc<WeightedGraphNode<K>>,
}

impl<K> Edge<K> {
    #[must_use]
    pub fn weight(&self) -> i32 {
        self.weight
    }

    #[must_use]
    pub fn node(&self) -> &Rc<WeightedGraphNode<K>> {
        &self.node
    }
}

pub struct WeightedGraphNode<K> {
    id: K,
    nodes: RefCell<Vec<Edge<K>>>,
}

impl<K> WeightedGraphNode<K>
where
    K: Ord + Hash + Copy + Eq,
{
    #[must_use]
    pub fn new(id: K) -> Self {
        Self {
            id,
            nodes: RefCell::new(vec![]),
        }
    }

    #[must_use]
    pub fn id(&self) -> K {
        self.id
    }

    #[must_use]
    pub fn nodes(&self) -> Ref<Vec<Edge<K>>> {
        Ref::map(self.nodes.borrow(), |x| x)
    }
}

pub struct WeightedGraph<K = i32>(HashMap<K, Rc<WeightedGraphNode<K>>>);

impl<K> WeightedGraph<K>
where
    K: Ord + Hash + Copy + Eq,
{
    #[must_use]
    pub fn new() -> Self {
        WeightedGraph(HashMap::new())
    }

    pub fn insert(&mut self, id: K) {
        let node = Rc::new(WeightedGraphNode::new(id));

        self.0.insert(node.id, node);
    }
    ///
    /// # Panics
    ///
    /// If `from_node_id` or `to_node_id` does not exist in a `WeightedGraph`, then this method will panic at either of them.
    pub fn connect(&mut self, from_node_id: K, to_node_id: K, edge_weight: i32) {
        let from_node = self
            .get(&from_node_id)
            .expect("Passed \"from_node_id\" does not exist");
        let to_node = self
            .get(&to_node_id)
            .expect("Passed \"to_node_id\" does not exist");

        from_node.nodes.borrow_mut().push(Edge {
            weight: edge_weight,
            node: Rc::clone(to_node),
        });
    }

    #[must_use]
    pub fn get(&self, node_id: &K) -> Option<&Rc<WeightedGraphNode<K>>> {
        self.0.get(node_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<K> Default for WeightedGraph<K>
where
    K: Ord + Hash + Copy + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}
