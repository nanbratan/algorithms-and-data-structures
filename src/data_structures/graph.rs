use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
#[derive(Debug)]
pub struct Node<T, K> {
    pub id: K,
    pub item: T,
    pub nodes: Option<Vec<Rc<Node<T, K>>>>,
}

pub struct Graph<T, K = i32>(HashMap<K, Rc<Node<T, K>>>);

impl<T, K> Graph<T, K>
where
    K: Eq + Hash + Copy,
{
    #[must_use]

    pub fn new() -> Self {
        Graph(HashMap::new())
    }

    pub fn insert(&mut self, node: Rc<Node<T, K>>) {
        self.0.insert(node.id, node);
    }

    #[must_use]
    pub fn get(&self, node_id: &K) -> Option<&Rc<Node<T, K>>> {
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

impl<T, K> Default for Graph<T, K>
where
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}