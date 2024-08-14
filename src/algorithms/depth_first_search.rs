use crate::tree::{Tree, TreeNode};
use std::rc::Rc;

/// # Description
///
/// This is traversal algorithm, which means that we recursively go through all nodes in whole tree.
/// So the difference from breadth first search algorithm is that in BFS we search by layers, but in DFS we search branch by branch.
/// In DFS - If desired node is in first level, but in the last branch, then we won't get it until we check all branches before the last one.
/// In BFS - If desired node in the first branch, but it is the latest node, then we won't get it until we check ALL layers, which means all elements
///
/// So in terms of complexity both `DFS` and `BFS` have the same `O` complexity
/// * If used with graphs - `O(n * e)`, where `n` is a number of nodes and `e` is a number of edges.
/// * If used with trees - `O(n)`, where `n` is a number of nodes.
///
/// **Note** that complexity is different for graphs and trees because:
/// * graphs are more flexible with edges, i.e. graphs' nodes may have multiple parents and point to each other making cycles.
/// * trees are more conservative, tree's nodes can have only one parent, and they cannot have cycles(nodes can't point to parents)
///
/// So there is no faster algorithm between DFS and BFS, it depends on details.
pub fn depth_first_search<T, N, K, V, P>(tree: &T, predicate: P) -> Option<Rc<N>>
where
    N: TreeNode<V, K>,
    T: Tree<N, V, K>,
    P: Fn(&N) -> bool,
{
    fn search<N, V, K, P>(node: &Rc<N>, predicate: &P) -> Option<Rc<N>>
    where
        N: TreeNode<V, K>,
        P: Fn(&N) -> bool,
    {
        if predicate(node) {
            return Some(Rc::clone(node));
        }

        for node in node.nodes().borrow().iter() {
            let search_result = search(node, predicate);

            if search_result.is_some() {
                return search_result;
            }
        }

        None
    }

    search(tree.head(), &predicate)
}

#[cfg(test)]
mod tests {
    use crate::algorithms::depth_first_search::depth_first_search;
    use crate::tree::{BasicTree, TreeNode};

    #[test]
    fn should_find_shortest() {
        let mut tree = BasicTree::from_head(1, false);

        tree.insert(2, 1, false);
        tree.insert(3, 1, false);
        tree.insert(4, 2, false);
        tree.insert(5, 2, false);
        tree.insert(6, 3, false);
        tree.insert(7, 3, true);
        tree.insert(8, 6, false);

        assert_eq!(&7, depth_first_search(&tree, |x| *x.value()).unwrap().id())
    }

    #[test]
    fn should_not_find_anything() {
        let mut tree = BasicTree::from_head(1, false);

        tree.insert(2, 1, false);
        tree.insert(3, 1, false);

        assert!(depth_first_search(&tree, |x| *x.value()).is_none());
    }
}
