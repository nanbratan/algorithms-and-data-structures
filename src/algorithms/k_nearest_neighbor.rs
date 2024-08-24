use std::collections::HashMap;

trait Neighbor {
    fn calculate_neighbor_distance(&self, another_neighbor: &Self) -> i32;
}

pub fn k_nearest_neighbor<'a, T: Neighbor>(
    neighbors: &'a HashMap<&str, T>,
    item_name: &str,
    neighbors_count: usize,
) -> Vec<&'a str> {
    let mut neighbors_with_distance: Vec<(&str, i32)> = Vec::with_capacity(neighbors.len());
    let item = neighbors.get(item_name).unwrap();

    for (&name, neighbor) in neighbors.iter() {
        let distance = neighbor.calculate_neighbor_distance(item);
        neighbors_with_distance.push((name, distance));
    }

    neighbors_with_distance.sort_by(|a, b| a.1.cmp(&b.1));
    neighbors_with_distance.shrink_to(neighbors_count);
    neighbors_with_distance
        .iter()
        .map(|neighbor| neighbor.0)
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

    struct Viewer {
        name: &'static str,
        preferences: Preferences,
    }

    impl Viewer {
        fn new(
            name: &'static str,
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

    impl Neighbor for Viewer {
        fn calculate_neighbor_distance(&self, another_neighbor: &Self) -> i32 {
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
                .sqrt() as i32
        }
    }

    #[test]
    fn should_find_three_nearest() {
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

        let three_nearest_neighbors = k_nearest_neighbor(&neighbors, "margie", 3);

        assert_eq!(vec!["jared"], three_nearest_neighbors);
        assert!(three_nearest_neighbors.contains(&"bob"));
        assert!(three_nearest_neighbors.contains(&"jared"));
        assert!(three_nearest_neighbors.contains(&"cristy"));
    }
}
