use crate::tree::{Tree, TreeNode};
use std::rc::Rc;

/// TODO: Docs
pub fn depth_first_search<T, N, K, V, P>(tree: &T, predicate: P) -> Option<Rc<N>>
where
    N: TreeNode<Value = V>,
    T: Tree<N, K, V>,
    P: Fn(&N) -> bool,
{
    fn search<N, P>(node: Rc<N>, predicate: &P) -> Option<Rc<N>>
    where
        N: TreeNode,
        P: Fn(&N) -> bool,
    {
        if predicate(&node) {
            return Some(node);
        }

        for node in node.nodes().borrow().iter() {
            let node = Rc::clone(node);
            let search_result = search(node, predicate);

            if search_result.is_some() {
                return search_result;
            }
        }

        None
    }

    search(Rc::clone(tree.head()), &predicate)
}

#[cfg(test)]
mod tests {
    use crate::algorithms::depth_first_search::depth_first_search;
    use crate::tree::{BasicTree, Tree, TreeNode};

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
