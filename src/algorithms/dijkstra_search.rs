use crate::weighted_graph::{WeightedGraph, WeightedGraphNode};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

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

fn calculate_cost<T, K>(
    node: &Rc<WeightedGraphNode<T, K>>,
    cost: &mut HashMap<K, i32>,
    parents: &mut HashMap<K, K>,
) where
    K: Ord + Hash + Copy + Eq,
{
    let current_node_cost = *cost.get(&node.id()).unwrap_or(&0);

    for child in node.nodes().iter() {
        let new_cost_to_child = current_node_cost + child.weight();

        // TODO: Need to change implementation for below code
        //  The issue is - if there is no existing cost, then we should insert new_cost_to_child AND insert node.id() into parents
        //  Currently we don't insert parents if there is no existing cost. Maybe I can use match for that?
        cost.entry(child.node().id())
            .and_modify(|current_min_cost_to_child| {
                if &new_cost_to_child < current_min_cost_to_child {
                    *current_min_cost_to_child = new_cost_to_child;

                    parents
                        .entry(child.node().id())
                        .and_modify(|x| *x = node.id())
                        .or_insert(node.id());
                }
            })
            .or_insert(new_cost_to_child);
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

pub fn dijkstra_search<V, K>(graph: &WeightedGraph<V, K>, start: K, finish: K) -> Vec<K>
where
    K: Ord + Hash + Copy + Eq,
{
    let mut cost: HashMap<K, i32> = HashMap::new();
    let mut parents = HashMap::new();

    calculate_cost(graph.get(&start).unwrap(), &mut cost, &mut parents);

    while let Some(lowest) = get_lowest(&cost, &finish) {
        calculate_cost(graph.get(&lowest).unwrap(), &mut cost, &mut parents);
        cost.remove(&lowest);
    }

    build_chain(finish, &parents)
}
