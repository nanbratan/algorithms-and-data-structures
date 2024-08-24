pub use binary_search::binary_search;
pub use binary_search::binary_search_for_tree;
pub use breadth_first_search::breadth_first_search;
pub use depth_first_search::depth_first_search;
pub use dijkstra_search::dijkstra_search;
pub use quick_sort::quick_sort;
pub use selection_sort::selection_sort;
pub use selection_sort::selection_sort_by_key;

mod binary_search;
mod breadth_first_search;
mod depth_first_search;
mod dijkstra_search;
mod k_nearest_neighbor;
mod quick_sort;
mod selection_sort;

#[derive(Clone, Copy)]
pub enum Order {
    Desc,
    Asc,
}
