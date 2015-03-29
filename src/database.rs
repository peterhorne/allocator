use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::cmp;

#[derive(Debug)]
struct Resource {
    total: u32,
    consumed: u32,
}

impl Resource {
    fn new() -> Self {
        Resource { total: 0, consumed: 0 }
    }

    fn remaining(&self) -> u32 {
        self.total - self.consumed
    }
}

#[derive(Debug)]
pub struct Database {
    resources: HashMap<String, Resource>,
    consumers: HashMap<String, Vec<(String, u32)>>,
}

impl Database {
    pub fn new() -> Self {
        Database { resources: HashMap::new(), consumers: HashMap::new() }
    }

    pub fn stock(&mut self, resource_id: &String, total: u32) -> u32 {
        let resource = match self.resources.entry(resource_id.clone()) {
            Occupied(entry) => entry.into_mut(),
            Vacant(entry)   => entry.insert(Resource::new()),
        };

        resource.total = cmp::max(resource.consumed, total);
        resource.total
    }

    pub fn consume(&mut self, consumer_id: &String, resources: &Vec<(String, u32)>) -> Vec<(String, u32)> {
        if let Some(consumer) = self.consumers.get(consumer_id) {
            for &(ref resource_id, quantity) in consumer {
                if let Some(resource) = self.resources.get_mut(resource_id) {
                    resource.consumed -= quantity;
                };
            };
        };

        let result = resources.iter().filter_map(|&(ref id, quantity)| {
            self.resources.get_mut(id).map (|resource| {
                let delta = cmp::min(resource.remaining(), quantity);
                resource.consumed += delta;
                (id.clone(), delta)
            })
        }).collect::<Vec<(String, u32)>>();

        self.consumers.insert(consumer_id.clone(), result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use super:: Database;

    #[test]
    fn stock() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();

        let result = database.stock(&resource_id, 120);

        assert_eq!(120, result);
    }

    #[test]
    fn increase_stock() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();

        database.stock(&resource_id, 120);
        let result = database.stock(&resource_id, 260);

        assert_eq!(260, result);
    }

    #[test]
    fn decrease_stock() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();

        database.stock(&resource_id, 120);
        let result = database.stock(&resource_id, 80);

        assert_eq!(80, result);
    }

    #[test]
    fn decrease_consumed_stock() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();
        let consumer_id = "4254aa40-b5de-0132-6bde-38f6b11b3709".to_string();

        database.stock(&resource_id, 40);
        database.consume(&consumer_id, &vec![(resource_id.clone(), 15)]);
        let result = database.stock(&resource_id, 10);

        assert_eq!(15, result);
    }

    #[test]
    fn consume() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();
        let consumer_id = "4254aa40-b5de-0132-6bde-38f6b11b3709".to_string();

        database.stock(&resource_id, 120);
        let result = database.consume(&consumer_id, &vec![(resource_id.clone(), 6)]);

        if let [(ref id, ref quantity)] = result.as_ref() {
            assert_eq!(resource_id, *id);
            assert_eq!(6, *quantity);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn consume_greater_than_available() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();
        let consumer_id = "4254aa40-b5de-0132-6bde-38f6b11b3709".to_string();

        database.stock(&resource_id, 4);
        let result = database.consume(&consumer_id, &vec![(resource_id.clone(), 6)]);

        if let [(ref id, ref quantity)] = result.as_ref() {
            assert_eq!(resource_id, *id);
            assert_eq!(4, *quantity);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn consume_missing_resource() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();
        let consumer_id = "4254aa40-b5de-0132-6bde-38f6b11b3709".to_string();

        let result = database.consume(&consumer_id, &vec![(resource_id.clone(), 6)]);

        assert_eq!(0, result.len());
    }

    #[test]
    fn increase_consumed_quantity() {
        let mut database = Database::new();
        let resource_id = "04f921d0-b5dd-0132-6bde-38f6b11b3709".to_string();
        let consumer_id1 = "4254aa40-b5de-0132-6bde-38f6b11b3709".to_string();
        let consumer_id2 = "6b12f730-b7d0-0132-091f-109add5e5b84".to_string();

        database.stock(&resource_id, 10);
        database.consume(&consumer_id1, &vec![(resource_id.clone(), 6)]);
        database.consume(&consumer_id1, &vec![(resource_id.clone(), 8)]);
        let result = database.consume(&consumer_id2, &vec![(resource_id.clone(), 4)]);

        if let [(ref id, ref quantity)] = result.as_ref() {
            assert_eq!(resource_id, *id);
            assert_eq!(2, *quantity);
        } else {
            assert!(false);
        }
    }
}
