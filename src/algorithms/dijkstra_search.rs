use crate::weighted_graph::{WeightedGraph, WeightedGraphNode};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

// TODO: The book mentioned that it's better to use "Priority Queue" data structure for that.
//  I have some ideas what that might be, but it's better to learn "Priority Queue" and get back here than guessing.
//  Also it seems Rust has std::collections::BinaryHeap which is a "Priority Queue", but I'd like to figure out by myself how to implement it and then use existed solution.
fn get_lowest<K>(cost: &HashMap<K, i32>, finish: &K) -> Option<K>
where
    K: Ord + Hash + Copy + Eq,
{
    let lowest = cost
        .iter()
        .reduce(|acc, item| if item.1 < acc.1 { item } else { acc })?;

    if lowest.0 == finish {
        return None;
    }

    Some(*lowest.0)
}

fn calculate_cost<K>(
    node: &Rc<WeightedGraphNode<K>>,
    cost: &mut HashMap<K, i32>,
    parents: &mut HashMap<K, K>,
) where
    K: Ord + Hash + Copy + Eq,
{
    let current_node_cost = *cost.get(&node.id()).unwrap_or(&0);

    for child in node.nodes().iter() {
        let new_cost_to_child = current_node_cost + child.weight();

        match cost.entry(child.node().id()) {
            Entry::Occupied(current_min_cost_to_child) => {
                if &new_cost_to_child < current_min_cost_to_child.get() {
                    *current_min_cost_to_child.into_mut() = new_cost_to_child;
                    parents
                        .entry(child.node().id())
                        .and_modify(|entry| *entry = node.id());
                }
            }
            Entry::Vacant(current_min_cost_to_child) => {
                current_min_cost_to_child.insert(new_cost_to_child);
                parents.insert(child.node().id(), node.id());
            }
        }
    }
}

fn build_chain<K>(finish: K, parents: &HashMap<K, K>) -> Vec<K>
where
    K: Ord + Hash + Copy + Eq,
{
    let mut chain = vec![finish];
    let mut next_parent = parents.get(&finish);

    while let Some(parent) = next_parent {
        chain.push(*parent);
        next_parent = parents.get(parent);
    }

    chain.reverse();
    chain
}

/// # Description
///
/// Dijkstra search algorithm is an approach to find the shortest path from A to B in a graph.
/// The difference between BFS and Dijkstra search is - Dijkstra search is working with weighted graphs, whereas BFS is working with unweighted graphs.
/// That's it, besides that they are similar as they both have `O(n)` complexity in general as we have to went through all nodes.
///
/// Realisation details:
/// 1. Find a node with the lowest cost(a weight to get to the node). In the beginning we need to calculate s cost from start node to its children.
/// 2. Then We take the cheapest node(a node with the lowest cost) and calculate cost to its children(the same way as we did with the start node).
/// 3. If new cost from the current node to a child is lower than existing cost(e.g. there was another path to the child, but more expensive), then we update the child's cost and its parent.
/// 3. When cost to children is calculated - we drop a node from `cost` HashMap as we don't need it anymore, we found cost to its children already.
/// 4. Repeat 1-3 steps till the lowest node is the `finish` node. That means we reached the end of our graph and visited all nodes.
/// 5. Build a chain from the start to the finish using `parents` HashMap.
pub fn dijkstra_search<K>(graph: &WeightedGraph<K>, start: K, finish: K) -> Vec<K>
where
    K: Ord + Hash + Copy + Eq,
{
    let mut cost: HashMap<K, i32> = HashMap::new();
    let mut parents = HashMap::new();

    // Here we need to get cost to start's children
    calculate_cost(graph.get(&start).unwrap(), &mut cost, &mut parents);

    // Then we get the cheapest node and calculate its children cost till we reach finish(get_lowest returns None if current lowest is finish node)
    while let Some(lowest) = get_lowest(&cost, &finish) {
        calculate_cost(graph.get(&lowest).unwrap(), &mut cost, &mut parents);
        // Remove node from cost HashMap when we're done with it.
        cost.remove(&lowest);
    }

    build_chain(finish, &parents)
}

#[cfg(test)]
mod tests {
    use super::dijkstra_search;
    use crate::weighted_graph::WeightedGraph;

    #[test]
    fn should_find_shortest_path() {
        // given
        let mut graph = WeightedGraph::new();
        const BOOK: &str = "book";
        const DISK: &str = "disk";
        const POSTER: &str = "poster";
        const DRUMS: &str = "drums";
        const GUITAR: &str = "guitar";
        const PIANO: &str = "piano";

        graph.insert(BOOK);
        graph.insert(DISK);
        graph.insert(POSTER);
        graph.insert(DRUMS);
        graph.insert(GUITAR);
        graph.insert(PIANO);

        graph.connect(BOOK, DISK, 5);
        graph.connect(BOOK, POSTER, 0);
        graph.connect(DISK, GUITAR, 15);
        graph.connect(DISK, DRUMS, 20);
        graph.connect(POSTER, GUITAR, 30);
        graph.connect(POSTER, DRUMS, 35);
        graph.connect(GUITAR, PIANO, 20);
        graph.connect(DRUMS, PIANO, 10);

        // when
        let shortest_path = dijkstra_search(&graph, BOOK, PIANO);

        // then
        assert_eq!(vec![BOOK, DISK, DRUMS, PIANO], shortest_path);
    }
}
