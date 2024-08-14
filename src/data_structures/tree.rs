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
pub trait TreeNode<V, K> {
    fn id(&self) -> &K;
    fn value(&self) -> &V;
    fn parent(&self) -> &Option<Weak<Self>>;
    // TODO: This is probably not the best idea to create and return a new vector every time we call .nodes() method, but in BinarySearchTree I want to have nodes field as a fixed size array with 2 elements.
    //  The issue is - I want to have fixed size array([Rc<Self>; 2]) in BinarySearchTree, but dynamically sized vector(Vec<Rc<Self>>) in Tree(because a Node in a Tree may have any amount of children).
    //  However, I also want to specify that BinarySearchTreeNode implements TreeNode trait as I want to use BinarySearchTree wherever I can use Tree(BinarySearchTree is a Tree, but with extra logic).
    //  So, I don't really understand for now which type should .nodes() method return
    //      1. &Vec<_> doesn't work as I'd need to create a Vec<_> in BinarySearchTreeNode's .nodes() method and return a reference to it.
    //          This is not possible as I can't return a reference to a local variable(because local variable is going to be dropped when scope is over and the reference to it as well).
    //      2. &[_] is not possible as well because BinarySearchTreeNode stores nodes as Option, so we would need to create a new vector anyway to unwrap elements.
    fn nodes(&self) -> Vec<Rc<Self>>;
}

pub trait Tree<Node, V, K>
where
    Node: TreeNode<V, K>,
{
    fn head(&self) -> &Rc<Node>;
    /*fn insert(&mut self, id: K, value: V);*/
    fn get(&self, node_id: &K) -> Option<&Rc<Node>>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub struct BasicTreeNode<V, K> {
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

impl<V, K> TreeNode<V, K> for BasicTreeNode<V, K>
where
    K: Hash + Eq,
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
        self.nodes.borrow().clone()
    }
}

pub struct BasicTree<V, K = i32> {
    head: Rc<BasicTreeNode<V, K>>,
    tree: HashMap<K, Rc<BasicTreeNode<V, K>>>,
}

impl<V, K> BasicTree<V, K>
where
    K: Eq + Hash + Copy + Debug,
{
    #[must_use]
    pub fn from_head(head_id: K, head_value: V) -> Self {
        let mut tree = HashMap::new();
        let head = Rc::new(BasicTreeNode {
            id: head_id,
            parent: None,
            value: head_value,
            nodes: RefCell::new(vec![]),
        });

        tree.insert(head.id, Rc::clone(&head));

        Self { head, tree }
    }
    ///
    ///
    /// # Arguments
    ///
    /// * `id`: id for new leaf
    /// * `parent_id`: parent id for new leaf
    /// * `value`: value for new leaf
    ///
    /// returns: `()`
    ///
    /// # Panics
    ///
    /// Panics if provided `parent_id` does not exist.
    pub fn insert(&mut self, id: K, parent_id: K, value: V) {
        let parent = self.get(&parent_id);

        match parent {
            None => {
                panic!("Can't insert a new leaf, parent with id \"{parent_id:?}\" doesn't exist")
            }
            Some(parent) => {
                let node = Rc::new(BasicTreeNode::new(id, Rc::downgrade(parent), value));

                parent.nodes.borrow_mut().push(Rc::clone(&node));
                self.tree.insert(id, node);
            }
        }
    }
}

impl<V, K> Tree<BasicTreeNode<V, K>, V, K> for BasicTree<V, K>
where
    K: Eq + Hash + Copy + Debug,
{
    #[must_use]
    fn head(&self) -> &Rc<BasicTreeNode<V, K>> {
        &self.head
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
