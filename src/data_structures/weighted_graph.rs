#![allow(clippy::module_name_repetitions)]

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct Edge<T, K> {
    weight: i32,
    node: Rc<WeightedGraphNode<T, K>>,
}

impl<T, K> Edge<T, K> {
    #[must_use]
    pub fn weight(&self) -> i32 {
        self.weight
    }

    #[must_use]
    pub fn node(&self) -> &Rc<WeightedGraphNode<T, K>> {
        &self.node
    }
}

pub struct WeightedGraphNode<T, K> {
    id: K,
    value: T,
    nodes: RefCell<Vec<Edge<T, K>>>,
}

impl<T, K> WeightedGraphNode<T, K>
where
    K: Copy,
{
    #[must_use]
    pub fn new(id: K, value: T) -> Self {
        Self {
            id,
            value,
            nodes: RefCell::new(vec![]),
        }
    }

    #[must_use]
    pub fn id(&self) -> K {
        self.id
    }

    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    #[must_use]
    pub fn nodes(&self) -> Ref<Vec<Edge<T, K>>> {
        Ref::map(self.nodes.borrow(), |x| x)
    }
}

pub struct WeightedGraph<T, K = i32>(HashMap<K, Rc<WeightedGraphNode<T, K>>>);

impl<T, K> WeightedGraph<T, K>
where
    K: Hash + Eq + Copy,
{
    #[must_use]
    pub fn new() -> Self {
        WeightedGraph(HashMap::new())
    }

    pub fn insert(&mut self, id: K, value: T) {
        let node = Rc::new(WeightedGraphNode::new(id, value));

        self.0.insert(node.id, node);
    }

    pub fn connect(&mut self, from_node_id: K, to_node_id: K, edge_weight: i32) {
        let from_node = self.get(&from_node_id).unwrap();
        let to_node = self.get(&to_node_id).unwrap();

        from_node.nodes.borrow_mut().push(Edge {
            weight: edge_weight,
            node: Rc::clone(to_node),
        });
    }

    #[must_use]
    pub fn get(&self, node_id: &K) -> Option<&Rc<WeightedGraphNode<T, K>>> {
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

impl<T, K> Default for WeightedGraph<T, K>
where
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
