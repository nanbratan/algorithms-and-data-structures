use crate::graph::{Graph, GraphNode};
use crate::Queue;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

/// # Description
/// Breadth first algorithm works via search by "layers". "layers" in this context means that the "head" is 1st level node, all nodes to which head points are 2nd level nodes,
/// all nodes to which nodes from 2nd level point are 3rd level nodes and so on.
///
/// This algorithm ensures:
/// 1. We're going to find a path from the head to a needed node if it exists.
/// 2. If the path exists, then we're going to find the shortest one.
///
/// `Queue` is used here to make sure that we'll search a layer by a layer, instead of all nodes without any order.
/// Note that `Queue` is using `LinkedList`, so it won't contribute to capacity.
///
/// # Complexity
/// This algorithm has `O(n * e)` complexity, where `n` is a number of nodes and `e` is a number of edges(connections between nodes).
pub fn breadth_first_search<K, G, N, T, P>(
    start_node_id: K,
    graph: &G,
    predicate: P,
) -> Option<&Rc<N>>
where
    T: Debug,
    G: Graph<N, K>,
// Debug trait only for Drop trait visualisation, it should be removed if visualisation is not needed
    N: GraphNode<Value=T> + Debug,
    K: Eq + Hash + Copy + Debug,
    P: Fn(&T) -> bool,
{
    let mut checked_nodes = HashMap::with_capacity(graph.len());
    let head_node = graph.get(&start_node_id)?;
    let mut queue = Queue::from(head_node.nodes().as_ref()?);

    while let Some(queue_item) = queue.take() {
        // Different nodes may point to a same node, so to avoid extra check of already checked nodes - we log them and skip them
        // It also prevents infinity loop in case if we have 2 nodes which points to each other
        if checked_nodes.contains_key(queue_item.id()) {
            continue;
        }

        if predicate(queue_item.value()) {
            return Some(queue_item);
        }

        checked_nodes.insert(queue_item.id(), true);

        if let Some(nodes) = &queue_item.nodes() {
            queue.append(nodes);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::algorithms::breadth_first_search::breadth_first_search;
    use crate::graph::{BasicGraph, BasicGraphNode, Graph, GraphNode};

    #[derive(Debug)]
    struct Item(bool);

    #[test]
    fn should_find_shortest() {
        let mut graph = BasicGraph::new();

        // TODO: Need to re-write `graph.insert()`/`BasicGraphNode::new()` as I did it with Tree/TreeNode
        let eight = Rc::new(BasicGraphNode::new(8, Item(false), None));
        let seven = Rc::new(BasicGraphNode::new(7, Item(true), None));
        let six = Rc::new(BasicGraphNode::new(
            6,
            Item(false),
            Some(vec![Rc::clone(&eight)]),
        ));
        let five = Rc::new(BasicGraphNode::new(5, Item(false), None));
        let four = Rc::new(BasicGraphNode::new(4, Item(false), None));
        let three = Rc::new(BasicGraphNode::new(
            3,
            Item(false),
            Some(vec![Rc::clone(&six), Rc::clone(&seven), Rc::clone(&five)]),
        ));
        let two = Rc::new(BasicGraphNode::new(
            2,
            Item(false),
            Some(vec![Rc::clone(&four), Rc::clone(&five)]),
        ));
        let one = Rc::new(BasicGraphNode::new(
            1,
            Item(false),
            Some(vec![Rc::clone(&two), Rc::clone(&three)]),
        ));

        graph.insert(eight);
        graph.insert(seven);
        graph.insert(six);
        graph.insert(five);
        graph.insert(four);
        graph.insert(three);
        graph.insert(two);
        graph.insert(one);

        assert_eq!(&7, breadth_first_search(1, &graph, |x| x.0).unwrap().id())
    }

    #[test]
    fn should_not_find_anything() {
        let mut graph = BasicGraph::new();

        let three = Rc::new(BasicGraphNode::new(3, Item(false), None));
        let two = Rc::new(BasicGraphNode::new(2, Item(false), None));
        let one = Rc::new(BasicGraphNode::new(
            1,
            Item(false),
            Some(vec![Rc::clone(&two), Rc::clone(&three)]),
        ));

        graph.insert(one);
        graph.insert(three);
        graph.insert(two);

        assert!(breadth_first_search(1, &graph, |x| x.0).is_none());
    }
}
