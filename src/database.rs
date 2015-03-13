use std::collections::HashMap;

type ResourceMap = HashMap<Uuid, Quantity>;
type Uuid = String;
type Quantity = i32;

pub struct Database {
    resources: ResourceMap,
    consumers: HashMap<Uuid, ResourceMap>,
}

impl Database {
    pub fn new() -> Database {
        Database { resources: HashMap::new(), consumers: HashMap::new() }
    }
}
