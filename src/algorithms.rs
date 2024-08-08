pub use binary_search::binary_search;
pub use breadth_first_search::breadth_first_search;
pub use quick_sort::quick_sort;
pub use selection_sort::selection_sort;
pub use selection_sort::selection_sort_by_key;

mod binary_search;
mod selection_sort;
mod quick_sort;
mod breadth_first_search;

#[derive(Clone, Copy)]
pub enum Order {
    Desc,
    Asc,
}