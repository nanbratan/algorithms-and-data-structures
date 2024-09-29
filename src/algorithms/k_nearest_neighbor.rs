use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
pub trait Neighbor {
    fn calculate_neighbor_distance(&self, another_neighbor: &Self) -> f64;
}

struct NeighborWithDistance<'a> {
    pub neighbor_name: &'a str,
    pub distance: f64,
}

impl<'a> Eq for NeighborWithDistance<'a> {}

impl<'a> PartialEq<Self> for NeighborWithDistance<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<'a> PartialOrd<Self> for NeighborWithDistance<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for NeighborWithDistance<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance > other.distance {
            Ordering::Greater
        } else if self.distance < other.distance {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

// TODO: Docs
#[allow(dead_code)]
pub fn k_nearest_neighbor<'a, T: Neighbor>(
    neighbors: &'a HashMap<&str, T>,
    item_name: &'static str,
    neighbors_count: usize,
) -> Vec<&'a str> {
    let mut priority_queue: BinaryHeap<NeighborWithDistance> =
        BinaryHeap::with_capacity(neighbors_count);
    let item = neighbors.get(item_name).unwrap();

    for (&name, neighbor) in neighbors {
        if name == item_name {
            continue;
        }

        let next_neighbor = NeighborWithDistance {
            neighbor_name: name,
            distance: neighbor.calculate_neighbor_distance(item),
        };

        if priority_queue.len() == neighbors_count {
            let mut biggest_item = priority_queue.peek_mut().unwrap();
            if biggest_item.distance > next_neighbor.distance {
                *biggest_item = next_neighbor;
            }
        } else {
            priority_queue.push(next_neighbor);
        }
    }

    priority_queue
        .into_iter()
        .map(|neighbor| neighbor.neighbor_name)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{k_nearest_neighbor, Neighbor};
    use std::collections::HashMap;

    struct Preferences {
        comedy: i32,
        action: i32,
        drama: i32,
        horror: i32,
        romance: i32,
    }

    struct Viewer<'a> {
        name: &'a str,
        preferences: Preferences,
    }

    impl<'a> Viewer<'a> {
        fn new(
            name: &'a str,
            comedy: i32,
            action: i32,
            drama: i32,
            horror: i32,
            romance: i32,
        ) -> Self {
            Viewer {
                name,
                preferences: Preferences {
                    comedy,
                    action,
                    drama,
                    horror,
                    romance,
                },
            }
        }
    }

    impl<'a> Neighbor for Viewer<'a> {
        fn calculate_neighbor_distance(&self, another_neighbor: &Self) -> f64 {
            let comedy_difference =
                (&self.preferences.comedy - another_neighbor.preferences.comedy).pow(2);
            let action_difference =
                (&self.preferences.action - another_neighbor.preferences.action).pow(2);
            let drama_difference =
                (&self.preferences.drama - another_neighbor.preferences.drama).pow(2);
            let horror_difference =
                (&self.preferences.horror - another_neighbor.preferences.horror).pow(2);
            let romance_difference =
                (&self.preferences.romance - another_neighbor.preferences.romance).pow(2);

            ((comedy_difference
                + action_difference
                + drama_difference
                + horror_difference
                + romance_difference) as f64)
                .sqrt()
        }
    }

    #[test]
    fn should_find_three_nearest() {
        // given
        let bob = Viewer::new("bob", 3, 4, 4, 1, 4);
        let margie = Viewer::new("margie", 4, 3, 5, 1, 5);
        let john = Viewer::new("john", 2, 5, 1, 3, 1);
        let cristy = Viewer::new("cristy", 5, 1, 1, 1, 4);
        let tom = Viewer::new("top", 2, 1, 2, 1, 2);
        let jared = Viewer::new("jared", 2, 1, 4, 1, 4);

        let mut neighbors = HashMap::with_capacity(6);
        neighbors.insert("bob", bob);
        neighbors.insert("margie", margie);
        neighbors.insert("john", john);
        neighbors.insert("cristy", cristy);
        neighbors.insert("tom", tom);
        neighbors.insert("jared", jared);

        // when
        let three_nearest_neighbors = k_nearest_neighbor(&neighbors, "margie", 3);

        // then
        assert_eq!(3, three_nearest_neighbors.len());
        assert!(three_nearest_neighbors.contains(&"bob"));
        assert!(three_nearest_neighbors.contains(&"jared"));
        assert!(three_nearest_neighbors.contains(&"cristy"));
    }
}
