use crate::graph::{Graph, Node};
use crate::Queue;
use std::collections::HashMap;
use std::rc::Rc;

/// # Description
/// Breadth first algorithm works via search by "layers". "layers" in this context means that the "head" is 1st level node, all nodes to which head points are 2nd level nodes,
/// all nodes to which nodes from 2nd level point are 3rd level nodes and so on.
///
/// This algorithm ensures:
/// 1. We're going to find a path from the head to a needed node if it exists.
/// 2. If the path exists, then we're going to find the shortest one.
///
/// Queue is used here to make sure that we'll search a layer by a layer, instead of all nodes without any order.
/// Note that Queue is using LinkedList, so it won't contribute to capacity.
///
/// # Complexity
/// This algorithm has `O(n * e)` complexity, where `n` is a number of nodes and `e` is a number of edges(connections between nodes).
pub fn breadth_first_search<K, T, P>(head: K, graph: &Graph<T, K>, predicate: P) -> Option<&Rc<Node<T, K>>>
where
    P: Fn(&T) -> bool,
{
    let mut checked_nodes = HashMap::with_capacity(graph.len());
    let head_node = graph.get(&head)?;
    let mut queue = Queue::from(head_node.nodes.as_ref()?);

    while let Some(queue_item) = queue.take() {
        // Different nodes may point to a same node, so to avoid extra check of already checked nodes - we log them and skip them
        // It also prevents infinity loop in case if we have 2 nodes which points to each other
        if checked_nodes.contains_key(&queue_item.id) {
            continue;
        }

        if predicate(&queue_item.item) {
            return Some(queue_item);
        }

        checked_nodes.insert(&queue_item.id, true);

        if let Some(nodes) = &queue_item.nodes {
            queue.append(nodes)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::algorithms::breadth_first_search::{breadth_first_search, Node};
    use crate::graph::Graph;

    struct Item(bool);

    #[test]
    fn should_find_shortest() {
        let mut graph = Graph::new();

        let head_id = 1;

        let eight = Rc::new(Node {
            id: 8,
            item: Item(false),
            nodes: None,
        });
        let seven = Rc::new(Node {
            id: 7,
            item: Item(true),
            nodes: None,
        });
        let six = Rc::new(Node {
            id: 6,
            item: Item(false),
            nodes: Some(vec![Rc::clone(&eight)]),
        });
        let five = Rc::new(Node {
            id: 5,
            item: Item(false),
            nodes: None,
        });
        let four = Rc::new(Node {
            id: 4,
            item: Item(false),
            nodes: None,
        });
        let three = Rc::new(Node {
            id: 3,
            item: Item(false),
            nodes: Some(vec![Rc::clone(&six), Rc::clone(&seven), Rc::clone(&five)]),
        });
        let two = Rc::new(Node {
            id: 2,
            item: Item(false),
            nodes: Some(vec![Rc::clone(&four), Rc::clone(&five)]),
        });
        let one = Rc::new(Node {
            id: head_id,
            item: Item(false),
            nodes: Some(vec![Rc::clone(&two), Rc::clone(&three)]),
        });

        graph.insert(eight);
        graph.insert(seven);
        graph.insert(six);
        graph.insert(five);
        graph.insert(four);
        graph.insert(three);
        graph.insert(two);
        graph.insert(one);

        assert_eq!(
            7,
            breadth_first_search(head_id, &graph, |x| x.0).unwrap().id
        )
    }

    #[test]
    fn should_not_find_anything() {
        let mut graph = Graph::new();

        let head_id = 1;

        let three = Rc::new(Node {
            id: 3,
            item: Item(false),
            nodes: None,
        });
        let two = Rc::new(Node {
            id: 2,
            item: Item(false),
            nodes: None,
        });
        let one = Rc::new(Node {
            id: head_id,
            item: Item(false),
            nodes: Some(vec![Rc::clone(&two), Rc::clone(&three)]),
        });

        graph.insert(one);
        graph.insert(three);
        graph.insert(two);

        assert!(breadth_first_search(1, &graph, |x| x.0).is_none());
    }
}
