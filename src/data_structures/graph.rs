use std::collections::HashMap;
use std::rc::Rc;

pub struct Node<T, K> {
    pub id: K,
    pub item: T,
    pub nodes: Option<Vec<Rc<Node<T, K>>>>,
}

pub struct Graph<T, K = i32>(HashMap<K, Rc<Node<T, K>>>);

impl<T, K> Graph<T, K> {
    pub fn new() -> Self {
        Graph(HashMap::new())
    }

    pub fn insert(&mut self, node: Rc::new(Node<T, K>)) {
        self.0.insert(*node.id, node);
    }

    pub fn get(&self, node_id: &i32) -> Option<&Rc<Node<T, K>>> {
        self.0.get(node_id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

